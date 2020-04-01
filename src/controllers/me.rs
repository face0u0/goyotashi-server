use rocket::{
    Route,
    response::{status::Custom},
};
use rocket_contrib::json::Json;
use crate::{
    models::{Community, ResponseErr, Header},
    services,
};

#[get("/communities")]
fn belongs(header: Header) -> Result<Json<Vec<Community>>, Custom<Json<ResponseErr>>> {
    services::community::belongings(header.user)
        .map_err(|err| err.render())
        .map(|com_v| Json(com_v))
}

pub fn router() -> Vec<Route>{
    return routes![belongs];
}