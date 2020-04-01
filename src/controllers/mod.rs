pub mod community;
pub mod err;
pub mod user;
pub mod review;
pub mod restaurant;
pub mod me;

use rocket;
use rocket::request::{FromRequest, Outcome};
use crate::models::{Header};
use crate::logic;

impl<'a, 'r> FromRequest<'a, 'r> for Header {
    type Error = ();

    fn from_request(request: &'a rocket::request::Request<'r>) -> Outcome<Header, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }
        let keys: Vec<&str> = keys[0].split_whitespace().collect();
        if keys.len() != 2 {
            return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }
        let token = keys[1].to_owned();
        let user_like = logic::authorize::auth(&token);
        match user_like{
            Ok(user) => Outcome::Success(Header{method: keys[0].to_owned(), token, user}),
            Err(_) => Outcome::Failure((rocket::http::Status::Unauthorized, ()))
        }
    }
}
