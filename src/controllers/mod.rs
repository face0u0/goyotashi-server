pub mod community;
pub mod err;
pub mod user;
pub mod review;
pub mod restaurant;

use rocket;
use rocket::request::{FromRequest, Outcome};

pub struct Jwt{
    pub method: String,
    pub token: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Jwt {
    type Error = ();

    fn from_request(request: &'a rocket::request::Request<'r>) -> Outcome<Jwt, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }
        let keys: Vec<&str> = keys[0].split_whitespace().collect();
        if keys.len() != 2 {
            return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }
        Outcome::Success(Jwt{method: keys[0].to_owned(), token: keys[1].to_owned()})
    }
}
