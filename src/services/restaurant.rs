use crate::{
    models::{Restaurant, PostedPin, Pin, User, NoIdRestaurant, NoIdPin},
    mapper,
    logic,
    errors::*
};

pub fn index_included_by(community_id: i32) -> Result<Vec<Restaurant>, ErrCode>{
    mapper::restaurant::find_all_pined_by(&community_id)
}

pub fn add(current: User, posted_pin: PostedPin) -> Result<Pin, ErrCode>{
    logic::privilege::check_joined(&current, &posted_pin.community_id)?;
    //TODO google mapから取るようにする
    let no_id_restaurant = NoIdRestaurant{
        place_id: posted_pin.place_id.to_owned(),
        name: "やんぱお".to_string(),
        lat: 23.1313,
        lng: 134.3422
    };
    mapper::restaurant::insert_or_update(&no_id_restaurant)
        .and_then(|restaurant| {
            let no_id_pin = NoIdPin{ restaurant_id: restaurant.id, community_id: posted_pin.community_id };
            mapper::pin::create(&no_id_pin)
        })
}
