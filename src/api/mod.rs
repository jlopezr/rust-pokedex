use std::sync::Arc;

use crate::repositories::pokemon::Repository;

mod create_pokemon;
mod fetch_all_pokemons;
pub mod fetch_pokemon;
mod health;

enum Status {
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

impl From<Status> for rouille::Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::Conflict => 409,
            Status::InternalServerError => 500,
        };
        Self {
            status_code,
            headers: vec![],
            data: rouille::ResponseBody::empty(),
            upgrade: None,
        }
    }
}

pub fn serve(url: &str, repo: Arc<dyn Repository>) {
    rouille::start_server(url, move |req| {
        router!(req,
            (GET) (/health) => {
                health::serve()
            },
            (POST) (/) => {
                create_pokemon::serve(repo.clone(), req) //Clones only the ARC pointer
            },
            (GET) (/) => {
                fetch_all_pokemons::serve(repo.clone())
            },
            (GET) (/{number: u16}) => {
                fetch_pokemon::serve(repo.clone(), number)
            },
            _ => {
                rouille::Response::from(Status::NotFound)
            }
        )
    });
}
