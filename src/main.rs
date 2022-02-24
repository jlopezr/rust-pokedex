use repositories::pokemon::InMemoryRepository;
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
    let repo = Arc::new(InMemoryRepository::new());
    
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::new("cli").long("cli").help("Runs in CLI mode"))
        .get_matches();

    match matches.occurrences_of("cli") {
        0 => api::serve("localhost:8000", repo),
        _ => cli::run(repo),
    }        
}
