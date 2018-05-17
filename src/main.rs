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
extern crate textwrap;
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

const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const PROG: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/* Symbols for argument identifying */
const DIRECTORY: &str = "directory";
const REMOTE: &str = "remote";
const VERBOSITY: &str = "verbosity";
const LINE_LENGTH: &str = "line-length";

fn list_issues() -> IMResult<()> {
    /* TODO: Argument parsing should be in a separate function */
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
        .arg(
            Arg::with_name(LINE_LENGTH)
                .short("l")
                .long(LINE_LENGTH)
                .value_name(LINE_LENGTH)
                .help("Line length")
                .takes_value(true),
        )
        .get_matches();

    let line_length = matches.value_of(LINE_LENGTH).map(|s| s.parse::<usize>().unwrap_or(72)).unwrap_or(72);
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
        table.add_row(row![i.name(), i.assignee(), textwrap::fill(i.description(), line_length)]);
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
