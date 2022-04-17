use rocket::serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
}