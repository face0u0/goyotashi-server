#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;
use crate::models::{Community};

mod controllers;
mod models;
mod services;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/communities?<search>")]
fn community_search(search: Option<String>) -> Json<Vec<Community>> {
    return match search {
        Some(s) => Json(services::community::search_community(&s)),
        None => panic!()
    }
}

#[get("/communities/<_id>")]
fn community_detail(_id: Option<u32>) -> Json<Community> {
    return match _id {
        Some(i) => Json(services::community::find_community(i)),
        None => panic!()
    }
}

#[derive(Serialize, Deserialize)]
struct PostCommunity {
    pub name: String,
    pub description: String,
    pub public: bool,
}
#[post("/communities", data = "<community>")]
fn community_create(community: Json<PostCommunity>) -> Json<Community> {
    Json(services::community::create_community(&community.name, &community.description, community.public))
}

#[put("/communities/<_id>", data = "<community>")]
fn community_update(_id: u32, community: Json<PostCommunity>) -> Json<Community> {
    Json(services::community::update_community(_id, &community.name, &community.description, community.public))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, community_search, community_detail, community_create, community_update])  // ここにルーティングをセットする
        .launch();
}