#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod mapper;
mod services;
mod errors;


fn main() {
    rocket::ignite()
        .mount("/communities", controllers::community::router())
        .register(controllers::err::router())
        .launch();
}