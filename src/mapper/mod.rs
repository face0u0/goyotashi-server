use postgres::{Client, NoTls};

pub mod community;
pub mod user;
pub mod member;
pub mod pin;
pub mod review;
pub mod restaurant;

pub fn get_client() -> Client {
    let client = Client::connect("postgresql://postgres:root@localhost/goyotashi", NoTls).unwrap();
    return client;
}