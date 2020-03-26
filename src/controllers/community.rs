use rocket::{
    Route,
    response::{
        status::Custom
    }
};
use rocket_contrib::json::Json;
use crate::{
    models::{Community, NoIdCommunity, ResponseErr},
    services,
    errors::*,
};

#[get("/?<search>")]
fn search(search: Option<String>) -> Result<Json<Vec<Community>>, Custom<Json<ResponseErr>>> {
    search
        .ok_or(ErrCode::new(Stat::BadRequest, "Query 'search' not found."))
        .and_then(|search| services::community::search_by_name(search))
        .map_err(|err| err.render())
        .map(|com_v| Json(com_v))
}

#[get("/<_id>")]
fn detail(_id: Option<i32>) -> Result<Json<Community>, Custom<Json<ResponseErr>>> {
    _id
        .ok_or(ErrCode::new(Stat::BadRequest, "ID is invalid."))
        .and_then( |community_id| services::community::find(community_id))
        .map_err(|err| err.render())
        .map(|com| Json(com))
}

#[post("/", data = "<community>")]
fn create(community: Json<NoIdCommunity>) -> Result<Json<Community>, Custom<Json<ResponseErr>>> {
    services::community::create(NoIdCommunity {
        name: community.name.to_string(),
        description: community.description.to_string(),
        public: community.public
    })
        .map_err(|err| err.render())
        .map(|com| Json(com))
}

#[put("/<_id>", data = "<community>")]
fn update(_id: i32, community: Json<NoIdCommunity>) -> Result<Json<Community>, Custom<Json<ResponseErr>>> {
    services::community::update(Community{
        id: _id,
        name: community.name.to_string(),
        description: community.description.to_string(),
        public: community.public
    })
        .map_err(|err| err.render())
        .map(|com| Json(com))
}

pub fn router() -> Vec<Route>{
    return routes![search, detail, create, update];
}