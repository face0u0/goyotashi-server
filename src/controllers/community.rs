use rocket::{
    Route,
    response::{
        status::Custom
    },
    http::{
        Status
    }
};
use rocket_contrib::json::Json;
use crate::{
    models::{Community, NoIdCommunity, ResponseErr},
    services
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
        .ok_or("ID is invalid".to_string())
        .and_then( |community_id| services::community::find(community_id))
        .map_err(|err| Custom(Status::NotFound, Json(ResponseErr {msg: err.to_string()})))
        .map(|com| Json(com))
}

#[post("/", data = "<community>")]
fn create(community: Json<NoIdCommunity>) -> Result<Json<Community>, Custom<Json<ResponseErr>>> {
    services::community::create(NoIdCommunity {
        name: community.name.to_string(),
        description: community.description.to_string(),
        public: community.public
    })
        .map_err(|err| Custom(Status::NotFound, Json(ResponseErr {msg: err.to_string()})))
        .map(|com| Json(com))
}

#[put("/<_id>", data = "<community>")]
fn update(_id: i32, community: Json<NoIdCommunity>) -> Json<Community> {
    Json(services::community::update(Community{
        id: _id,
        name: community.name.to_string(),
        description: community.description.to_string(),
        public: community.public
    }).unwrap())
}

pub fn router() -> Vec<Route>{
    return routes![search, detail, create, update];
}