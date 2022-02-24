use std::sync::Arc;
use crate::domain::create_pokemon::{self, Error};

use rouille;
use serde::{Deserialize, Serialize};
use crate::api::Status;
use crate::repositories::pokemon::Repository;

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>
}

#[derive(Serialize)]
struct Response {
    number: u16,
}

pub fn serve(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
    let req = match rouille::input::json_input::<Request>(req) {
        Ok(req) => create_pokemon::Request {
            number: req.number,
            name: req.name,
            types: req.types,

        },
        _ => return rouille::Response::from(Status::BadRequest),
    };
        
    match create_pokemon::execute(repo, req) {
        Ok(number) => rouille::Response::json(&Response { number }),
        Err(Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(Error::Conflict) => rouille::Response::from(Status::Conflict),
        Err(Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}