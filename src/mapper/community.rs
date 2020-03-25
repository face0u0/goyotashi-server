extern crate postgres;

use postgres::{Client, NoTls};
use crate::models::{Community, NoIdCommunity};

pub fn get_client() -> Client {
    let mut client = Client::connect("postgresql://postgres:root@localhost/goyotashi", NoTls).unwrap();
    return client;
}

pub fn select_community(_id: i32) -> Result<Community, &'static str> {
    let mut community = Err("no community");
    for row in get_client().query(
        "SELECT id, name, description, public FROM communities WHERE id = $1", &[&_id]).unwrap() {
        community = Ok(Community {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            public: row.get(3)
        });
    }
    return community;
}

pub fn insert_community(community: &NoIdCommunity) -> Result<Community, &'static str> {
    let mut new_community = Err("no community");
    for row in get_client().query(
        "INSERT INTO communities ( name, description, public ) VALUES ( $1, $2, $3 ) RETURNING id",
        &[&community.name, &community.description, &community.public]).unwrap() {
        new_community = Ok(Community{
            id: row.get(0),
            name: community.name.to_string(),
            description: community.description.to_string(),
            public: community.public
        })
    }
    return new_community;
}

pub fn update_community(community: &Community) -> Result<Community, &'static str> {
    let mut new_community = Err("no community");
    get_client().execute(
        "UPDATE communities SET name = $1, description = $2, public = $3 WHERE id = $4",
        &[&community.name, &community.description, &community.public, &community.id]
    );
    new_community = Ok(Community {
        id: community.id,
        name: community.name.to_string(),
        description: community.description.to_string(),
        public: community.public
    });
    return new_community;
}