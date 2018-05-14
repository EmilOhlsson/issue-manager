#[macro_use]
extern crate hyper;

#[macro_use]
extern crate log;

#[macro_use]
extern crate prettytable;

#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate env_logger;
extern crate git2;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;

mod error;
mod git_server;
mod issues;
mod settings;

use clap::{App, Arg};
use error::IMResult;
use git_server::*;
use issues::{IMIssue, Issue};
use log::LevelFilter;
use prettytable::Table;
use settings::get_settings;

/* TODO: Read fields from repository */
const ABOUT: &str = "Reads GitHub/GitLab issues for git repository";
const AUTHOR: &str = "Emil Ohlsson";
const DIRECTORY: &str = "directory";
const PROG: &str = "issue-handler";
const REMOTE: &str = "remote";
const VERBOSITY: &str = "verbosity";
const VERSION: &str = "0.1.0";

fn list_issues() -> IMResult<()> {
    let matches = App::new(PROG)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::with_name(VERBOSITY)
                .short("v")
                .long(VERBOSITY)
                .multiple(true)
                .help("Sets level of verbosity"),
        )
        .arg(
            Arg::with_name(REMOTE)
                .short("r")
                .long(REMOTE)
                .value_name(REMOTE)
                .help("Remote to use URL from")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(DIRECTORY)
                .short("C")
                .long(DIRECTORY)
                .value_name(DIRECTORY)
                .help("Working directory")
                .takes_value(true),
        )
        .get_matches();

    let remote = matches.value_of(REMOTE).unwrap_or("origin");
    let directory = matches.value_of(DIRECTORY).unwrap_or(".");
    let verbosity = match matches.occurrences_of(VERBOSITY) {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };

    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .filter_module(&PROG.replace("-", "_"), verbosity)
        .init();

    let settings = get_settings()?;

    let server = get_server(directory, remote, &settings)?;
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
        Err(e) => error!("{}", e),
    }
}
