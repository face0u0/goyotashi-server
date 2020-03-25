#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod mapper;
mod services;


fn main() {
    rocket::ignite()
        .mount("/", controllers::community::router())
        .launch();
}