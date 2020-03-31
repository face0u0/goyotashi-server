use postgres::{Client, NoTls};
use std::env::var;

pub mod community;
pub mod user;
pub mod member;
pub mod pin;
pub mod review;
pub mod restaurant;

pub fn get_client() -> Client {
    let url = var("DATABASE_URL").unwrap_or("localhost".to_string());
    let client = Client::connect(
        format!("postgresql://postgres:root@{url}/goyotashi", url=url).as_str(),
        NoTls
    ).unwrap();
    return client;
}