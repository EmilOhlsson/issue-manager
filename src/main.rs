#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate prettytable;

#[macro_use]
extern crate hyper;

extern crate git2;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;

mod issues;
mod error;
mod git_server;
mod settings;

use prettytable::Table;
use settings::get_settings;
use error::IMResult;
use git_server::*;
use issues::{IMIssue, Issue};

fn list_issues() -> IMResult<()> {
    let settings = get_settings()?;

    /* TODO: Path and upstream should be parameters */
    let server = get_server(".", "origin", &settings)?;
    let issues: Vec<IMIssue> = get_issues(&server)?;

    let mut table = Table::new();
    table.add_row(row![b -> "Title", b-> "Assignee", b -> "Description"]);
    for i in &issues {
        table.add_row(row![i.name(), i.assignee(), i.description()]);
    }
    table.printstd();

    Ok(())
}

fn main() {
    match list_issues() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
