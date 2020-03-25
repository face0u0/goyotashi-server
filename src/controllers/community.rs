use rocket::{Route};
use rocket_contrib::json::Json;
use crate::{
    models::{Community, NoIdCommunity},
    services
};

#[get("/?<search>")]
fn community_search(search: Option<String>) -> Json<Vec<Community>> {
    return match search {
        Some(s) => Json(services::community::search_community(&s)),
        None => panic!()
    }
}

#[get("/<_id>")]
fn community_detail(_id: Option<i32>) -> Json<Community> {
    return match _id {
        Some(i) => Json(services::community::find(i)),
        None => panic!()
    }
}

#[post("/", data = "<community>")]
fn community_create(community: Json<NoIdCommunity>) -> Json<Community> {
    Json(services::community::create(NoIdCommunity {
        name: community.name.to_string(),
        description: community.description.to_string(),
        public: community.public
    }))
}

#[put("/<_id>", data = "<community>")]
fn community_update(_id: i32, community: Json<NoIdCommunity>) -> Json<Community> {
    Json(services::community::update(Community{
        id: _id,
        name: community.name.to_string(),
        description: community.description.to_string(),
        public: community.public
    }))
}

pub fn router() -> Vec<Route>{
    return routes![community_search, community_detail, community_create, community_update];
}