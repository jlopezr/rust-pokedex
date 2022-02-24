use std::sync::Arc;

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
    message: String,
}

pub fn serve(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
    match rouille::input::json_input::<Request>(req) {
        Ok(_) => {},
        _ => return rouille::Response::from(Status::BadRequest),
    }


    rouille::Response::json(&Response {
        message: String::from("Pokemon created!"),
    })
}