#[macro_use]
extern crate rocket;

use rocket::{catch, Request, Rocket};
use rocket::serde::json::{serde_json::json, Value};

use crate::response::Response;

mod entity;
mod api;
mod response;
mod middleware;

#[catch(404)]
fn not_found(_req: &Request) -> Value {
    json!(Response{
        code:404,
        msg:"not_found",
        data:()
    })
}

#[catch(default)]
fn catch_default() -> String {
    "something error ".to_string()
}

#[rocket::main]
#[allow(unused)]
pub async fn main() {
    // let res =
    let mut res = Rocket::build()
        .register("/", catchers![not_found,catch_default])
        .mount("/", api::greetings::export_routes());
        res.launch().await;
}
