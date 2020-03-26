use crate::models::{Community, NoIdCommunity};
use crate::mapper::get_client;
use postgres::Row;

pub fn find(_id: i32) -> Result<Community, String> {
    get_client().query(
        "SELECT id, name, description, public FROM communities WHERE id = $1",
        &[&_id]
    )
        .map_err(|err| err.to_string())
        .and_then(|rows| extract_one_community(&rows, "no community found."))
}

pub fn create(community: &NoIdCommunity) -> Result<Community, String> {
    get_client().query(
        "INSERT INTO communities ( name, description, public ) VALUES ( $1, $2, $3 ) RETURNING id, name, description, public",
        &[&community.name, &community.description, &community.public]
    )
        .map_err(|err| err.to_string())
        .and_then(|rows| extract_one_community(&rows, "community cannot created."))
}

pub fn update(community: &Community) -> Result<Community, String> {
    get_client().query(
        "UPDATE communities SET name = $1, description = $2, public = $3 WHERE id = $4 RETURNING id, name, description, public",
        &[&community.name, &community.description, &community.public, &community.id]
    )
        .map_err(|err| err.to_string())
        .and_then(|rows| extract_one_community(&rows, "community cannot updated."))
}

fn extract_one_community(rows: &Vec<Row>, err: &'static str) -> Result<Community, String>{
    rows.last()
        .ok_or(err.to_string())
        .map(|row| row_to_community(row))
}

fn row_to_community(row: &Row) -> Community {
    Community {
        id: row.get(0),
        name: row.get(1),
        description: row.get(2),
        public: row.get(3)
    }
}