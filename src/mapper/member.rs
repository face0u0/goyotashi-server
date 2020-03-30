use crate::models::{Member, NoIdMember};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn create(no_id_member: &NoIdMember) -> Result<Member, ErrCode> {
    get_client().query(
        "INSERT INTO members ( user_id, community_id ) VALUES ( $1, $2 ) RETURNING id, user_id, community_id",
        &[&no_id_member.user_id, &no_id_member.community_id]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::BadRequest, "Invalid user or community."))
}

pub fn check_joined(no_id_member: &NoIdMember) -> Result<bool, ErrCode> {
    let rows = get_client().query(
        "SELECT id FROM members WHERE user_id = $1 AND community_id = $2",
        &[&no_id_member.user_id, &no_id_member.community_id]
    );
    match rows {
        Ok(rows) => Ok(rows.len() == 1),
        Err(err) => Err(ErrCode::new_db_err(&err))
    }
}

fn extract_one(rows: &Vec<Row>, stat: Stat, err: &'static str) -> Result<Member, ErrCode>{
    rows.last()
        .ok_or(ErrCode::new(stat, err))
        .map(|row| Member {
            id: row.get(0),
            user_id: row.get(1),
            community_id: row.get(2)
        })
}