use crate::models::{Restaurant, NoIdRestaurant};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn find_all_pined_by(community_id: &i32) -> Result<Vec<Restaurant>, ErrCode> {
    get_client().query(
        "SELECT r.id, r.vendor, r.place_id, r.name, r.lat, r.lng FROM pins LEFT JOIN restaurants r on pins.restaurant_id = r.id WHERE community_id = $1",
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

pub fn insert_or_update(no_id_restaurant: &NoIdRestaurant) -> Result<Restaurant, ErrCode> {
    get_client().query(
        "INSERT INTO restaurants ( vendor, place_id, name, lat, lng ) VALUES ( $1, $2, $3, $4, $5 ) \
        ON CONFLICT (place_id) \
        DO UPDATE SET name = $6, lat = $7, lng = $8 \
        RETURNING id, place_id, name, lat, lng",
        &[&no_id_restaurant.vendor, &no_id_restaurant.place_id, &no_id_restaurant.name, &no_id_restaurant.lat, &no_id_restaurant.lng,
            &no_id_restaurant.name, &no_id_restaurant.lat, &no_id_restaurant.lng]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::NotFound, "No restaurant found."))
}

fn extract_one(rows: &Vec<Row>, stat: Stat, err: &'static str) -> Result<Restaurant, ErrCode>{
    rows.last()
        .ok_or(ErrCode::new(stat, err))
        .map(|row| row_to_restaurant(row))
}

fn row_to_restaurant(row: &Row) -> Restaurant {
    Restaurant {
        id: row.get(0),
        vendor: row.get(1),
        place_id: row.get(2),
        name: row.get(3),
        lat: row.get(4),
        lng: row.get(5)
    }
}