#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod controller;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/communities?<search>")]
fn search_community(search: Option<String>) -> String {
    return match search {
        Some(s) => s,
        None => controller::get_community()
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, search_community])  // ここにルーティングをセットする
        .launch();
}