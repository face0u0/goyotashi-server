use crate::models::{User, NoIdUser};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn find(_id: &i32) -> Result<User, ErrCode> {
    get_client().query(
        "SELECT id, uid, name, email FROM users WHERE id = $1",
        &[&_id]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::NotFound, "No user found."))
}

#[allow(dead_code)]
pub fn find_by_uid(uid: &String) -> Result<User, ErrCode> {
    get_client().query(
        "SELECT id, uid, name, email FROM users WHERE uid = $1",
        &[&uid]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::NotFound, "No user found."))
}

pub fn insert_or_update(no_id_user: &NoIdUser) -> Result<User, ErrCode> {
    get_client().query(
        "INSERT INTO users ( name, uid, email ) VALUES ( $1, $2, $3 ) \
        ON CONFLICT (uid) \
        DO UPDATE SET name = $4, email = $5 \
        RETURNING id, uid, name, email",
        &[&no_id_user.name, &no_id_user.uid, &no_id_user.email, &no_id_user.name, &no_id_user.email]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::NotFound, "No user found."))
}


pub fn find_all_included_by(community_id: &i32) -> Result<Vec<User>, ErrCode> {
    get_client().query(
        "SELECT u.id, u.uid, u.name, u.email FROM members LEFT JOIN users u on members.user_id = u.id WHERE members.community_id = $1",
        &[&community_id]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .map(|rows| {
            let mut res_v: Vec<User> = vec![];
            for row in &rows{
                res_v.push(row_to_user(row))
            }
            res_v
        })
}

fn extract_one(rows: &Vec<Row>, stat: Stat, err: &'static str) -> Result<User, ErrCode>{
    rows.last()
        .ok_or(ErrCode::new(stat, err))
        .map(|row| row_to_user(row))
}

fn row_to_user(row: &Row) -> User {
    User {
        id: row.get(0),
        uid: row.get(1),
        name: row.get(2),
        email: row.get(3)
    }
}
