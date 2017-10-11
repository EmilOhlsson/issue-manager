extern crate git2;

use git2::Repository;

fn get_servers() -> Result<Vec<String>, git2::Error> {
    let repo = Repository::open(".")?;
    let servers = repo.remotes()?
        .iter()
        .filter_map(|e| e)
        .map(String::from)
        .collect::<Vec<_>>();
    Ok(servers)
}

fn main() {
    let servers = get_servers().unwrap_or(Vec::new());
    for srv in servers {
        println!("server - {}", srv);
    }
}
