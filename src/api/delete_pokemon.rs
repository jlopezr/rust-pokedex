use super::Status;
use crate::{domain::delete_pokemon, repositories::Repository};
use std::sync::Arc;

pub fn serve(repo: Arc<dyn Repository>, number: u16) -> rouille::Response {
    let req = delete_pokemon::Request { number };
    match delete_pokemon::execute(repo, req) {
        Ok(()) => rouille::Response::from(Status::Ok),
        Err(delete_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(delete_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
        Err(delete_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}
