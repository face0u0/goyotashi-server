use crate::models::{Community, NoIdCommunity};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn find_all_by_name(name: String) -> Result<Vec<Community>, ErrCode> {
    get_client().query(
        "SELECT id, name, description, public FROM communities WHERE name LIKE $1 AND public = TRUE",
        &[&name]
    )
        .map_err(|_| ErrCode::new_db_err())
        .map(|rows| {
            let mut com_v: Vec<Community> = vec![];
            for row in &rows{
                com_v.push(row_to_community(row))
            }
            com_v
        })
}

pub fn find(_id: i32) -> Result<Community, ErrCode> {
    get_client().query(
        "SELECT id, name, description, public FROM communities WHERE id = $1",
        &[&_id]
    )
        .map_err(|_| ErrCode::new_db_err())
        .and_then(|rows| extract_one_community(&rows, Stat::NotFound, "No community found."))
}

pub fn create(community: &NoIdCommunity) -> Result<Community, ErrCode> {
    get_client().query(
        "INSERT INTO communities ( name, description, public ) VALUES ( $1, $2, $3 ) RETURNING id, name, description, public",
        &[&community.name, &community.description, &community.public]
    )
        .map_err(|_| ErrCode::new_db_err())
        .and_then(|rows| extract_one_community(&rows, Stat::BadRequest, "Invalid Community Object."))
}

pub fn update(community: &Community) -> Result<Community, ErrCode> {
    get_client().query(
        "UPDATE communities SET name = $1, description = $2, public = $3 WHERE id = $4 RETURNING id, name, description, public",
        &[&community.name, &community.description, &community.public, &community.id]
    )
        .map_err(|_| ErrCode::new_db_err())
        .and_then(|rows| extract_one_community(&rows, Stat::BadRequest, "Invalid Community Object."))
}

fn extract_one_community(rows: &Vec<Row>, stat: Stat, err: &'static str) -> Result<Community, ErrCode>{
    rows.last()
        .ok_or(ErrCode::new(stat, err))
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