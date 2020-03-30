use crate::models::{Restaurant};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn find_all_pined_by(community_id: &i32) -> Result<Vec<Restaurant>, ErrCode> {
    get_client().query(
        "SELECT r.id, r.place_id, r.name, r.lat, r.lng FROM pins LEFT JOIN restaurants r on pins.restaurant_id = r.id WHERE community_id = $1",
        &[&community_id]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .map(|rows| {
            let mut res_v: Vec<Restaurant> = vec![];
            for row in &rows{
                res_v.push(row_to_restaurant(row))
            }
            res_v
        })
}

fn row_to_restaurant(row: &Row) -> Restaurant {
    Restaurant {
        id: row.get(0),
        place_id: row.get(1),
        name: row.get(2),
        lat: row.get(3),
        lng: row.get(4)
    }
}