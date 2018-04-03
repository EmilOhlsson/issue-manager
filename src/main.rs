extern crate git2;
extern crate reqwest;

use git2::Repository;
use std::io::Read;
use std::error;
use std::fmt;

/* TODO: Should probably add a Trait for Issues and implementing types GithubIssue
 * and GitlabIssue */
/* TODO: Place Error type block in separate file */
#[derive(Debug)]
enum IManError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
}

impl fmt::Display for IManError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IManError::Reqwest(ref err) => write!(f, "Error in request: {}", err),
            IManError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for IManError {
    fn description(&self) -> &str {
        match *self {
            IManError::Reqwest(ref err) => err.description(),
            IManError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            IManError::Reqwest(ref err) => Some(err),
            IManError::Io(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for IManError {
    fn from(err: std::io::Error) -> IManError {
        IManError::Io(err)
    }
}

impl From<reqwest::Error> for IManError {
    fn from(err: reqwest::Error) -> IManError {
        IManError::Reqwest(err)
    }
}

fn url_splitter(ch: char) -> bool {
    ch == '@' || ch == ':' || ch == '/'
}

fn get_server(path: &str, remote: &str) -> Result<String, git2::Error> {
    let repo = Repository::open(path)?;
    let url = repo.find_remote(remote)?;
    /* TODO: Make this use proper errors */
    Ok(url.url().map(String::from).unwrap())
}

/* TODO: Translating address to API url can fail */
/* TODO: Create struct and add some FromStr trait to it to be able to parse addresses from Strings */
/* TODO: Build some kind of structure with repo information */
fn to_api_address(addr: &str) -> String {
    /* Assume that address is either something like
     * https://github.com/EmilOhlsson/issue-manager.git
     * git@github.com:EmilOhlsson/issue-manager.git
     */
    if addr.starts_with("git@") {
        let ts = addr.split(url_splitter).collect::<Vec<&str>>();
        return format!(
            "https://api.{}/repos/{}/{}",
            ts[1],
            ts[2],
            ts[3].replace(".git", "")
        );
    } else if addr.starts_with("https://") {
        panic!("Have't fixed this yet");
    } else {
        panic!("Can not handle address");
    }
}

/* TODO: Return a list of Issues */
fn get_issues(addr: &str) -> Result<String, IManError> {
    let mut result = String::new();
    reqwest::get(&format!("{}/issues", addr))?.read_to_string(&mut result)?;
    Ok(result)
}

fn main() {
    if let Ok(server) = get_server(".", "origin") {
        let url = to_api_address(&server);
        println!("{:?}", url);
    }
    let addr_tmp = get_server(".", "origin").unwrap();
    let addr = to_api_address(&addr_tmp);
    println!("-- {:?}", addr);
    println!("{:?}", get_issues(&addr));
}
