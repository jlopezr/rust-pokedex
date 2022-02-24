use repositories::pokemon::{InMemoryRepository, Repository, SqliteRepository};
use std::sync::Arc;
use clap::{Arg, Command};


mod api;
mod domain;
mod repositories;
mod cli;

#[macro_use]
extern crate rouille;

extern crate serde;

#[macro_use]
extern crate clap;

fn main() {        
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::new("cli").long("cli").help("Runs in CLI mode"))
        .arg(Arg::new("sqlite").long("sqlite").value_name("PATH"))
        .get_matches();

    let repo = build_repo(matches.value_of("sqlite"));

    match matches.occurrences_of("cli") {
        0 => api::serve("localhost:8000", repo),
        _ => cli::run(repo),
    }        
}

fn build_repo(sqlite_value: Option<&str>) -> Arc<dyn Repository> {
    if let Some(path) = sqlite_value {
        match SqliteRepository::try_new(path) {
            Ok(repo) => return Arc::new(repo),
            _ => panic!("Error while creating sqlite repo"),
        }
    }

    Arc::new(InMemoryRepository::new())
}
