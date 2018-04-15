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
mod error;
mod git_server;
mod settings;

use settings::get_settings;

use issues::{GithubIssue, Issue};
use git_server::*;

use std::env;

fn main() {
    let settings = get_settings();

    /*
    if let Ok(server) = get_server(".", "origin") {
        let url = to_api_address(&server);
        println!("{:?}", url);
    }
    */
    /*
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
    */
}
