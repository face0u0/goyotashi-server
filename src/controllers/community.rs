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
fn search(search: Option<String>) -> Json<Vec<Community>> {
    return match search {
        Some(s) => Json(services::community::search_by_name(&s)),
        None => panic!()
    }
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