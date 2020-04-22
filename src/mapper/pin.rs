use crate::models::{Pin, NoIdPin};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn create(no_id_pin: &NoIdPin) -> Result<Pin, ErrCode> {
    get_client().query(
        "INSERT INTO pins ( restaurant_id, community_id ) VALUES ( $1, $2 ) RETURNING id, restaurant_id, community_id",
        &[&no_id_pin.restaurant_id, &no_id_pin.community_id]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::BadRequest, "Invalid restaurant or community."))
}

pub fn find(_id: i32) -> Result<Pin, ErrCode> {
    get_client().query(
        "SELECT id, restaurant_id, community_id FROM pins WHERE id = $1",
        &[&_id]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::BadRequest, "Invalid restaurant or community."))
}

fn extract_one(rows: &Vec<Row>, stat: Stat, err: &'static str) -> Result<Pin, ErrCode>{
    rows.last()
        .ok_or(ErrCode::new(stat, err))
        .map(|row| row_to_pin(row))
}

fn row_to_pin(row: &Row) -> Pin {
    Pin {
        id: row.get(0),
        restaurant_id: row.get(1),
        community_id: row.get(2)
    }
}
