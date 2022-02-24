use repositories::pokemon::{InMemoryRepository, Repository, SqliteRepository, AirtableRepository};
use std::sync::Arc;
use clap::{Arg, Command, Values};

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
        .arg(
            Arg::new("airtable")
                .long("airtable")
                .value_names(&["API_KEY", "WORKSPACE_ID"]),
        )
        .get_matches();

    let repo = build_repo(matches.value_of("sqlite"), matches.values_of("airtable"));


    match matches.occurrences_of("cli") {
        0 => api::serve("localhost:8000", repo),
        _ => cli::run(repo),
    }        
}

fn build_repo(sqlite_value: Option<&str>, airtable_values: Option<Values>) -> Arc<dyn Repository> {
    if let Some(values) = airtable_values {
        if let [api_key, workspace_id] = values.collect::<Vec<&str>>()[..] {
            match AirtableRepository::try_new(api_key, workspace_id) {
                Ok(repo) => return Arc::new(repo),
                _ => panic!("Error while creating airtable repo"),
            }
        }
    }
    
    if let Some(path) = sqlite_value {
        match SqliteRepository::try_new(path) {
            Ok(repo) => return Arc::new(repo),
            _ => panic!("Error while creating sqlite repo"),
        }
    }

    Arc::new(InMemoryRepository::new())
}
