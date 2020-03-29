use rocket::{
    Route,
    response::{
        status::Custom
    }
};
use rocket_contrib::json::Json;
use crate::{
    models::{ShowUser, ResponseErr},
    services,
    errors::*,
};

#[get("/<_id>")]
fn show(_id: Option<i32>) -> Result<Json<ShowUser>, Custom<Json<ResponseErr>>> {
    _id
        .ok_or(ErrCode::new(Stat::BadRequest, "ID is invalid."))
        .and_then( |user_id| services::user::show(user_id))
        .map_err(|err| err.render())
        .map(|us| Json(us))
}

pub fn router() -> Vec<Route>{
    return routes![show];
}