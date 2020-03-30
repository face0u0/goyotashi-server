use rocket::{
    Route,
    response::{
        status::Custom
    },
    http
};
use rocket_contrib::json::Json;
use crate::{
    models::{Community, NoIdCommunity, ResponseErr, Member, Restaurant, NoIdPin},
    services,
    logic,
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
    services::community::create(community.into_inner())
        .map_err(|err| err.render())
        .map(|com| Json(com))
}

#[put("/<_id>", data = "<community>")]
fn update(_id: i32, community: Json<NoIdCommunity>) -> Result<Json<Community>, Custom<Json<ResponseErr>>> {
    services::community::update(_id, community.into_inner())
        .map_err(|err| err.render())
        .map(|com| Json(com))
}

#[post("/<_id>/users")]
fn join(_id: Option<i32>) -> Result<Json<Member>, Custom<Json<ResponseErr>>> {
    _id
        .ok_or(ErrCode::new(Stat::BadRequest, "ID is invalid."))
        .and_then( |community_id| services::member::join(logic::authorize::sample_jwt(), community_id))
        .map_err(|err| err.render())
        .map(|com| Json(com))
}

#[get("/<_id>/restaurants")]
fn restaurant_index(_id: Option<i32>) -> Result<Json<Vec<Restaurant>>, Custom<Json<ResponseErr>>> {
    _id
        .ok_or(ErrCode::new(Stat::BadRequest, "ID is invalid."))
        .and_then( |community_id| services::restaurant::index_included_by(community_id))
        .map_err(|err| err.render())
        .map(|res| Json(res))
}

#[post("/<_id>/restaurants/<_rid>")]
fn restaurant_add(_id: Option<i32>, _rid: Option<i32>) -> Result<http::Status, Custom<Json<ResponseErr>>> {
    _id
        .ok_or(ErrCode::new(Stat::BadRequest, "community ID is invalid."))
        .and_then( |community_id| {
            _rid
                .ok_or(ErrCode::new(Stat::BadRequest, "restaurant id is invalid."))
                .and_then(|restaurant_id| {
                    services::restaurant::add(NoIdPin{restaurant_id, community_id})
                })
        })
        .map_err(|err| err.render())
        .map(|_| http::Status::Created)
}

pub fn router() -> Vec<Route>{
    return routes![search, detail, create, update, join, restaurant_index, restaurant_add];
}