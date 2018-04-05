use git2;
use reqwest;
use std::io::Read;
use iman_error::IManError;

fn url_splitter(ch: char) -> bool {
    ch == '@' || ch == ':' || ch == '/'
}

pub fn get_server(path: &str, remote: &str) -> Result<String, IManError> {
    let repo = git2::Repository::open(path)?;
    let url = repo.find_remote(remote)?;
    /* TODO: Make this use proper errors */
    Ok(url.url().map(String::from).unwrap())
}

/* TODO: Translating address to API url can fail */
/* TODO: Create struct and add some FromStr trait to it to be able to parse addresses from Strings */
/* TODO: Build some kind of structure with repo information */
pub fn to_api_address(addr: &str) -> String {
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
pub fn get_issues(addr: &str) -> Result<String, IManError> {
    let mut result = String::new();
    reqwest::get(&format!("{}/issues", addr))?.read_to_string(&mut result)?;
    Ok(result)
}
