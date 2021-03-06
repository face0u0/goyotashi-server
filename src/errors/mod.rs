use rocket::{
    http::Status,
    response::status::Custom
};
use rocket_contrib::json::Json;

use crate::models::ResponseErr;
use std::error::Error;

// #[derive(Debug)]
pub struct ErrCode{
    pub status: Stat,
    msg: String
}

impl ErrCode {
    pub fn new(status: Stat, msg: &'static str) -> ErrCode {
        ErrCode {status, msg: msg.to_owned()}
    }

    pub fn new_db_err(err: &dyn Error) -> ErrCode {
        ErrCode {status: Stat::UnprocessableEntity, msg: err.to_string()}
    }

    pub fn render(&self) -> Custom<Json<ResponseErr>> {
        Custom(self.status.convert(), Json(ResponseErr{msg: self.msg.to_owned()}))
    }
}

#[allow(dead_code)]
pub enum Stat {
    Ok,
    Accepted,
    Created,
    BadRequest,
    Unauthorized,
    ForBitten,
    NotFound,
    UnprocessableEntity
}

impl Stat {
    fn convert(&self) -> Status {
        match self {
            Stat::Ok => Status::Ok,
            Stat::Accepted => Status::Accepted,
            Stat::Created => Status::Created,
            Stat::BadRequest => Status::BadRequest,
            Stat::Unauthorized => Status::Unauthorized,
            Stat::ForBitten => Status::Forbidden,
            Stat::NotFound => Status::NotFound,
            Stat::UnprocessableEntity => Status::UnprocessableEntity
        }
    }
}