use crate::models::{NoIdReview};
use crate::mapper::get_client;
use crate::errors::*;
use postgres::Row;

pub fn create_or_update(review: &NoIdReview) -> Result<NoIdReview, ErrCode> {
    get_client().query(
        "INSERT INTO reviews ( pin_id, member_id, good, comment ) VALUES ( $1, $2, $3, $4 ) \
        ON CONFLICT (pin_id, member_id) \
        DO UPDATE SET good = $3, comment = $4 \
        RETURNING id, pin_id, member_id, good, comment",
        &[&review.pin_id, &review.member_id, &review.good, &review.comment]
    )
        .map_err(|err| ErrCode::new_db_err(&err))
        .and_then(|rows| extract_one(&rows, Stat::BadRequest, "Invalid Review Object."))
}

fn extract_one(rows: &Vec<Row>, stat: Stat, err: &'static str) -> Result<NoIdReview, ErrCode>{
    rows.last()
        .ok_or(ErrCode::new(stat, err))
        .map(|row| row_to(row))
}

fn row_to(row: &Row) -> NoIdReview {
    NoIdReview {
        pin_id: row.get(1),
        member_id: row.get(2),
        good: row.get(3),
        comment: row.get(4)
    }
}