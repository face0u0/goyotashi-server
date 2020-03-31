use crate::{
    models::{Restaurant, Pin, User, NoIdPin},
    mapper,
    logic,
    errors::*
};

pub fn index_included_by(community_id: i32) -> Result<Vec<Restaurant>, ErrCode>{
    mapper::restaurant::find_all_pined_by(&community_id)
}

pub fn add(current: User, no_id_pin: NoIdPin) -> Result<Pin, ErrCode>{
    logic::privilege::check_joined(&current, &no_id_pin.community_id)?;
    mapper::pin::create(&no_id_pin)
}

pub fn search(lat: f64, lng: f64) -> Result<Vec<Restaurant>, ErrCode>{
    let no_id_restaurants = logic::vendor::gnavi::search(lat, lng)?;
    let mut restaurants: Vec<Restaurant> = Vec::new();
    for no_id_restaurant in &no_id_restaurants {
        let restaurant = mapper::restaurant::insert_or_update(&no_id_restaurant)?;
        restaurants.push(restaurant);
    }
    Ok(restaurants)
}
