use crate::models::{User, NoIdUser};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn find(_id: &i32) -> Result<User, ErrCode> {
    get_client().query(
        "SELECT id, uid, name, email FROM users WHERE id = $1",
        &[&_id]
    )
        .map_err(|_| ErrCode::new_db_err())
        .and_then(|rows| extract_one(&rows, Stat::NotFound, "No user found."))
}

pub fn find_by_uid(uid: &String) -> Result<User, ErrCode> {
    get_client().query(
        "SELECT id, uid, name, email FROM users WHERE uid = $1",
        &[&uid]
    )
        .map_err(|_| ErrCode::new_db_err())
        .and_then(|rows| extract_one(&rows, Stat::NotFound, "No user found."))
}

pub fn create(no_id_user: &NoIdUser) -> Result<User, ErrCode> {
    get_client().query(
        "INSERT INTO users ( name, uid, email ) VALUES ( $1, $2, $3 ) RETURNING id, uid, name, email",
        &[&no_id_user.name, &no_id_user.uid, &no_id_user.email]
    )
        .map_err(|_| ErrCode::new_db_err())
        .and_then(|rows| extract_one(&rows, Stat::BadRequest, "Invalid User Object."))
}

fn extract_one(rows: &Vec<Row>, stat: Stat, err: &'static str) -> Result<User, ErrCode>{
    rows.last()
        .ok_or(ErrCode::new(stat, err))
        .map(|row| User {
            id: row.get(0),
            uid: row.get(1),
            name: row.get(2),
            email: row.get(3)
        })
}