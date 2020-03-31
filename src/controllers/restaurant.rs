use rocket::{
    Route,
    response::{status::Custom},
};
use rocket_contrib::json::Json;
use crate::{
    models::{ResponseErr, Restaurant},
    services,
    errors::*,
};

#[get("/?<lat>&<lng>")]
fn search(lat: f64, lng: f64) -> Result<Json<Vec<Restaurant>>, Custom<Json<ResponseErr>>> {
    services::restaurant::search(lat, lng)
        .map(|v_res| Json(v_res))
        .map_err(|err| err.render())
}

pub fn router() -> Vec<Route>{
    return routes![search];
}