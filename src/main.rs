#[macro_use]
extern crate rocket;

use rocket::{catch, Request, Rocket, Route};
use rocket::serde::json::{serde_json::json, Value};

use crate::response::Response;

mod models;
mod api;
mod response;
mod middleware;
mod db;
mod entity;

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

#[get("/h1")]
pub fn index1() -> &'static str {
    "index1  "
}

#[get("/h2")]
pub fn index2() -> &'static str {
    "index2"
}

#[get("/h3")]
pub fn index3() -> &'static str {
    panic!("index3")
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.r * self.r)
    }
}

#[get("/circle_area")]
pub fn circle_area() ->String {
    let _a:f64 = 3.51123123;
    
    let c = Circle{r:0.5};
    format!("{}",c.area())
}

#[rocket::main]
#[allow(unused)]
pub async fn main() {
    let groups: Vec<(&str, Vec<Route>)> = vec![("/h1", routes![index1]), ("/h2", routes![index2,index3])];
    let mut res = Rocket::build()
        .register("/", catchers![not_found,catch_default])
        .mount("/", api::greetings::export_routes())
        .mount("/",routes![circle_area]);
    
    for i in groups {
        res = res.mount(i.0, i.1);
    }


    res.launch().await;
}
