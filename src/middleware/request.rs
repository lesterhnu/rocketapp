use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct Token;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = request.headers().get_one("Authorization");
        if let Some(t) = auth {
            println!("{}", t)
        } else {
            println!("token not found")
        }
        Outcome::Success(Token {})
    }
}