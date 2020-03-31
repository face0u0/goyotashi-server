use rocket::{
    Route,
    response::{status::Custom},
};
use rocket_contrib::json::Json;
use crate::{
    models::{ResponseErr, Restaurant},
    services,
};

#[get("/?<lat>&<lng>")]
fn search(lat: f64, lng: f64) -> Result<Json<Vec<Restaurant>>, Custom<Json<ResponseErr>>> {
    services::restaurant::search(lat, lng)
        .map(|v_res| Json(v_res))
        .map_err(|err| err.render())
}

#[get("/<rid>")]
fn find(rid: i32) -> Result<Json<Restaurant>, Custom<Json<ResponseErr>>> {
    services::restaurant::find(rid)
        .map(|res| Json(res))
        .map_err(|err| err.render())
}

pub fn router() -> Vec<Route>{
    return routes![search, find];
}