use rocket::{
    response::{
        status::Custom
    },
    Catcher
};
use rocket_contrib::json::Json;
use crate::{
    models::ResponseErr,
    errors::*,
};

#[catch(404)]
fn err_404() -> Custom<Json<ResponseErr>>{
    ErrCode::new(Stat::NotFound, "Resource not found.").render()
}

#[catch(422)]
fn err_422() -> Custom<Json<ResponseErr>>{
    ErrCode::new(Stat::BadRequest, "Invalid request type.").render()
}

#[catch(401)]
fn err_401() -> Custom<Json<ResponseErr>>{
    ErrCode::new(Stat::Unauthorized, "Authorization header maybe invalid.").render()
}

pub fn router() -> Vec<Catcher>{
    return catchers![err_404, err_422, err_401];
}