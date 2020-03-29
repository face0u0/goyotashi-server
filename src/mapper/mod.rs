use postgres::{Client, NoTls};

pub mod community;
pub mod user;

pub fn get_client() -> Client {
    let client = Client::connect("postgresql://postgres:root@localhost/goyotashi", NoTls).unwrap();
    return client;
}