use git2;
use reqwest;
use std::io::Read;
use iman_error::IManError;
use std::{fmt, fs::File, str::FromStr};

use toml;

use hyper;
use hyper::header::{Authorization, Scheme};

header! { (PrivateToken, "PRIVATE_TOKEN") => [String] }

// TODO: Token should probably be pushed upstream
#[derive(Clone, PartialEq, Debug)]
pub struct Token {
	pub token: String
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
		Ok(Token { token: s.to_owned()})
	}
}

/* TODO: Apart from URL this also need to store server name to be able to lookup
 * which token should use for the server */
#[derive(Debug)]
pub enum GitServer {
    Github(String),
    Gitlab(String),
}

fn url_splitter(ch: char) -> bool {
    ch == '@' || ch == ':' || ch == '/'
}

pub fn get_server(path: &str, remote: &str) -> Result<String, IManError> {
    let repo = git2::Repository::open(path)?;
    let git_remote = repo.find_remote(remote)?;

    if let Some(url) = git_remote.url() {
        Ok(String::from(url))
    } else {
        Err(IManError::new(format!("No url associated with {}", remote)))
    }
}

/* TODO: Translating address to API url can fail */
/* TODO: Create struct and add some FromStr trait to it to be able to parse addresses from Strings */
/* TODO: Build some kind of structure with repo information */
pub fn to_api_address(addr: &str) -> GitServer {
    /* Assume that address is either something like
     * https://github.com/EmilOhlsson/issue-manager.git
     * git@github.com:EmilOhlsson/issue-manager.git
     */
    if addr.starts_with("git@") {
        let ts = addr.split(url_splitter).collect::<Vec<&str>>();
        if ts[1] == "github.com" {
            
            GitServer::Github(format!(
                "https://api.{}/repos/{}/{}",
                ts[1],
                ts[2],
                ts[3].replace(".git", "")
            ))
        } else {
            GitServer::Gitlab(format!(
                "https://{}/api/v4/projects/{}%2F{}",
                ts[1],
                ts[2],
                ts[3].replace(".git", "")
            ))
        }
    } else if addr.starts_with("https://") {
        panic!("Have't fixed this yet");
    } else {
        panic!("Can not handle address");
    }
}

/* TODO: Return a list of Issues */
pub fn get_issues(server: &GitServer) -> Result<String, IManError> {
    let mut result = String::new();
    let client = reqwest::Client::new();
    match server {
        &GitServer::Github(ref addr) => {
            client.get(&format!("{}/issues", addr))
                .header(Authorization(
                    Token {
                        token: "foo-licious".to_owned()
                    }))
                .send()?
                .read_to_string(&mut result)?;
        }
        &GitServer::Gitlab(ref addr) => {
            client.get(&format!("{}/issues", addr))
                .header(PrivateToken("foo-licious".to_owned()))
                .send()?
                .read_to_string(&mut result)?;
        }
    }
    Ok(result)
}
