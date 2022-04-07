use rocket::form::{self, Error, Form, FromForm};
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};
#[allow(unused)]
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, Route};
use sea_orm::{Database, DatabaseConnection};
use crate::entity::prelude::*;
use crate::{entity,middleware};
use middleware::request::Token;

// use crate::response::
pub fn export_routes() -> Vec<Route> {
    routes![
        index,
        orm_test,
        hello,
        get_query_params,
        // get_with_token
        post_json_data,
        test_cookie,
        todo_task
    ]
}

#[get("/orm_test")]
pub async fn orm_test() -> Flash<Redirect> {
    // let opt = format!("{}://{}:{}","mysql","root","")
    let db: DatabaseConnection = Database::connect("mysql://root:123456@localhost/koutu")
        .await
        .unwrap();
    let cf:entity::config::ActiveModel = Config::find_by_id;
    println!("{:?}", res);
    Flash::success(Redirect::to("/h2/h2"), "config insert success")
}

#[derive(FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Person {
    pub name: String,
    #[field(validate = validate_person_age())]
    #[field(validate = range(21..))]
    pub age: u64,
}

fn validate_person_age<'v>(age: &u64) -> form::Result<'v, ()> {
    if age + 1 == 23 {
        Err(Error::validation("not right"))?;
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub id: usize,
    pub msg: String,
}

#[get("/")]
pub fn index() -> &'static str {
    "hello world"
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "hello"
}

#[get("/get_query_params?<person..>")]
pub fn get_query_params(person: Person) -> String {
    format!("got person: name:{},age:{}", person.name, person.age)
}

#[post("/post_json_data", data = "<json_data>", format = "json")]
pub async fn post_json_data(json_data: Json<Message>, _t: Token) -> Value {
    json!({
        "id":json_data.id,
        "msg":json_data.msg
    })
}

#[get("/test_cookie")]
pub fn test_cookie(cookies: &CookieJar<'_>) -> Option<String> {
    for i in cookies.iter() {
        println!("{:?}:{:?}", i.name(), i.value())
    }
    cookies.get("user_id").map(|crumb| {
        println!("{:?}:{:?}", crumb.name(), crumb.value());
        format!("User ID: {}", crumb.value())
    })
}

#[derive(FromForm, Debug)]
pub struct Task<'r> {
    pub complete: bool,
    pub r#type: &'r str,
}

#[get("/todo_task", data = "<task>")]
pub async fn todo_task(task: Form<Task<'_>>) {
    println!("{:?}", task);
}
// token guard
// #[post("/get_with_token", data = "<post_data>", format = "json")]
// pub async fn get_with_token(post_data: Json<Person>) -> Value {
//     json!(Response{
//         code:0,
//         msg:"get_with_token",
//         data:post_data
//     })
// }
