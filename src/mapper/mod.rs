use postgres::{Client, NoTls};

pub mod community;

pub fn get_client() -> Client {
    let client = Client::connect("postgresql://postgres:root@localhost/goyotashi", NoTls).unwrap();
    return client;
}