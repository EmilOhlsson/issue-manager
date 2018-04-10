#[macro_use]
extern crate serde_derive;

extern crate git2;
extern crate serde;

extern crate reqwest;
extern crate serde_json;
extern crate toml;

#[macro_use]
extern crate prettytable;

#[macro_use]
extern crate hyper;

use prettytable::Table;

mod issues;
mod iman_error;
mod git_server;

use issues::{GithubIssue, Issue};
use git_server::*;

fn stuff() {
    use std::fs::File;
    use std::io::prelude::*;
    use toml::Value;

    let mut f = File::open(".iman.toml").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    println!("Read: {}", buffer);
    let table = buffer.parse::<Value>().unwrap();
    println!("{:?}", table);
    println!("Deeenk {}", table["foo"]["yoo"]);
}

fn main() {
    stuff();

    if let Ok(server) = get_server(".", "origin") {
        let url = to_api_address(&server);
        println!("{:?}", url);
    }
    let addr_tmp = get_server(".", "origin").unwrap();
    let addr = to_api_address(&addr_tmp);
    println!("-- {:?}", addr);
    let issues = get_issues(&addr).unwrap();
    let issues_foo = serde_json::from_str::<Vec<GithubIssue>>(&issues);

    match issues_foo {
        Ok(issues_bar) => {
            let mut table = Table::new();
            table.add_row(row![b -> "Title", b-> "Assignee", b -> "Description"]);
            for i in &issues_bar {
                table.add_row(row![i.name(), i.assignee(), i.description()]);
            }
            table.printstd();
        }
        Err(_) => {
            println!("{:?}", issues);
        }
    }
}
