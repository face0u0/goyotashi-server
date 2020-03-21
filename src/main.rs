#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use crate::models::Community;

mod controllers;
mod models;
mod services;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/communities?<search>")]
fn search_community(search: Option<String>) -> Json<Vec<Community>> {
    return match search {
        Some(s) => Json(services::community::get_community(&s)),
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

fn main() {
    rocket::ignite()
        .mount("/", routes![index, search_community, community_detail])  // ここにルーティングをセットする
        .launch();
}