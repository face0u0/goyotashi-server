#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod mapper;
mod services;
mod errors;
mod logic;

fn main() {
    rocket::ignite()
        .mount("/communities", controllers::community::router())
        .mount("/users", controllers::user::router())
        .mount("/restaurants", controllers::restaurant::router())
        .mount("/me", controllers::me::router())
        .mount("/docs", rocket_contrib::serve::StaticFiles::new("./docs", rocket_contrib::serve::Options::Index))
        .register(controllers::err::router())
        .launch();
}