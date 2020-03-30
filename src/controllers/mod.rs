pub mod community;
pub mod err;
pub mod user;
pub mod review;
pub mod restaurant;

use rocket;
use crate::errors::*;
use rocket::request::FromRequest;
//
// pub fn extract_jwt(request: &rocket::Request) -> Result<String, ErrCode>{
//     let header = request.headers();
//     let spw = header.get("Authorization").last().unwrap().split_whitespace();
//     spw.last()
//         .ok_or(ErrCode::new(Stat::Unauthorized, "Invalid Authorization header."))
//         .map(|jwt| jwt.to_owned())
// }
