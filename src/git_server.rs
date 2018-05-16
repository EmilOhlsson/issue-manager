use git2;
use hyper;
use reqwest;
use serde_json;

use error::{IMError, IMResult};
use hyper::header::{Authorization, Scheme};
use issues::{GitHubIssue, GitLabIssue, IMIssue};
use settings::IMSettings;
use std::io::Read;
use std::{fmt, str::FromStr};

header! { (PrivateToken, "PRIVATE-TOKEN") => [String] }

// TODO: Token should probably be pushed upstream
#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token: String,
}

impl Scheme for Token {
    fn scheme() -> Option<&'static str> {
        Some("token")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl FromStr for Token {
    type Err = hyper::Error;
    fn from_str(s: &str) -> hyper::Result<Token> {
        Ok(Token {
            token: s.to_owned(),
        })
    }
}

/* TODO: Apart from URL this also need to store server name to be able to lookup
 * which token should use for the server */
#[derive(Debug)]
enum GitProtocol {
    GitHub,
    GitLab,
}

#[derive(Debug)]
pub struct GitServer {
    server: String,
    api: String,
    key: Option<String>,
    protocol: GitProtocol,
}

fn url_splitter(ch: char) -> bool {
    ch == '@' || ch == ':' || ch == '/'
}

/// Get the remote URL for git repo at `path` using the `remote`
fn get_remote_server(path: &str, remote: &str) -> IMResult<String> {
    let repo = git2::Repository::open(path)?;
    let git_remote = repo.find_remote(remote)?;
    Ok(String::from(git_remote.url().ok_or(IMError::new(
        &format!("No url associated with {}", remote),
    ))?))
}

impl FromStr for GitServer {
    type Err = IMError;

    fn from_str(addr: &str) -> IMResult<GitServer> {
        /* Assume that address is either something like
         * https://github.com/EmilOhlsson/issue-manager.git
         * git@github.com:EmilOhlsson/issue-manager.git
         */
        let ts = addr.split(url_splitter)
            .filter(|s| s.len() > 0)
            .collect::<Vec<&str>>();
        let server = *ts.get(1).ok_or(IMError::new("Could not parse server"))?;
        let group = *ts.get(2).ok_or(IMError::new("Could not parse group"))?;
        let project =
            (*ts.get(3).ok_or(IMError::new("Could not parse project"))?).replace(".git", "");
        let (protocol, api) = if server == "github.com" {
            (
                GitProtocol::GitHub,
                format!("https://api.{}/repos/{}/{}", server, group, project),
            )
        } else {
            (
                GitProtocol::GitLab,
                format!("https://{}/api/v4/projects/{}%2F{}", server, group, project),
            )
        };
        Ok(GitServer {
            server: String::from(server),
            protocol: protocol,
            api: api,
            key: None,
        })
    }
}

/// Retrieve a description of the git API, which protocol is used etc.
pub fn get_server(path: &str, remote: &str, config: &IMSettings) -> IMResult<GitServer> {
    let addr = get_remote_server(path, remote)?;
    let mut server = addr.parse::<GitServer>()?;
    if let Some(srv_setting) = config.get(&server.server) {
        if let Some(key) = srv_setting.get("key") {
            server.key = key.as_str().map(String::from);
        }
    }
    Ok(server)
}

/// Return a list of issues
pub fn get_issues(server: &GitServer) -> IMResult<Vec<IMIssue>> {
    let mut response = String::new();
    let client = reqwest::Client::new();

    trace!("Accessing API at {}", server.api);
    let mut request = client.get(&format!("{}/issues", server.api));
    if let &Some(ref key) = &server.key {
        match server.protocol {
            GitProtocol::GitHub => request.header(Authorization(Token {
                token: key.to_owned(),
            })),
            GitProtocol::GitLab => request.header(PrivateToken(key.to_owned())),
        };
    }

    trace!("Built request: {:?}", request);
    let mut rsp = request.send()?;
    rsp.read_to_string(&mut response)?;
    if rsp.status() != reqwest::StatusCode::Ok {
        error!("Status: {:?}", rsp.status());
        trace!("Got: {:?}", response);
        /* TODO: When receiving an error, parse it and report it readably */
        return Err(IMError::new("Error fetching issues"));
    }

    trace!("Got issue data. Will parse into list: {:?}", response);
    match server.protocol {
        GitProtocol::GitHub => {
            let gh_issues = serde_json::from_str::<Vec<GitHubIssue>>(&response)?;
            Ok(gh_issues
                .into_iter()
                .map(|i| IMIssue::GitHub(i))
                .collect::<Vec<_>>())
        }
        GitProtocol::GitLab => {
            let gl_issues = serde_json::from_str::<Vec<GitLabIssue>>(&response)?;
            Ok(gl_issues
                .into_iter()
                .map(|i| IMIssue::GitLab(i))
                .collect::<Vec<_>>())
        }
    }
}
