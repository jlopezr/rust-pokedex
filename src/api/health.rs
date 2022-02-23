use rouille;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

pub fn serve() -> rouille::Response {
    //rouille::Response::text("Gotta catch them all!")

    rouille::Response::json(&Response {
        message: String::from("Gotta catch them all!"),
    })
}