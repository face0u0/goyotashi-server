use rocket::{
    Route,
    response::{status::Custom},
    http,
};
use rocket_contrib::json::Json;
use crate::{
    models::{ResponseErr, Header, PostedReview},
    services,
};

#[post("/", data = "<review>")]
fn eval(review: Json<PostedReview>, head: Header) -> Result<http::Status, Custom<Json<ResponseErr>>> {
    let review = review.into_inner();
    services::review::add_or_update(head.user, review.pin_id, review.good, review.comment)
        .map_err(|err| err.render())
        .map(|_| http::Status::Ok)
}

pub fn router() -> Vec<Route>{
    return routes![eval];
}