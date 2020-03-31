use reqwest;
use serde::{Deserialize, Serialize};
use crate::errors::*;
use crate::models::NoIdRestaurant;

#[derive(Debug, Serialize, Deserialize)]
struct Gnavi {
    pub id: String,
    pub name: String,
    pub latitude: String,
    pub longitude: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GnaviRes {
    pub rest: Vec<Gnavi>
}

pub fn v_id() -> i32{
    return 1;
}

pub fn search(lat: f64, lng: f64) -> Result<Vec<NoIdRestaurant>, ErrCode>{
    fetch_search(lat, lng)
        .map_err(|_| ErrCode::new(Stat::UnprocessableEntity, "Gnavi service unavailable."))
}

fn fetch_search(lat: f64, lng: f64) -> Result<Vec<NoIdRestaurant>, reqwest::Error>{
    let response: GnaviRes = reqwest::blocking::get(
        format!(
            "https://api.gnavi.co.jp/RestSearchAPI/v3/?keyid={key}&latitude={lat}&longitude={lng}&hit_per_page={page}",
            lat=lat, lng=lng, key="75e9a269c366fc995cc6d978441ead40", page=30
        ).as_str()
    )?.json()?;
    let mut restaurants: Vec<NoIdRestaurant> = Vec::new();
    for gnavi in &response.rest{
        restaurants.push(NoIdRestaurant{
            place_id: gnavi.id.to_owned(),
            vendor: v_id(),
            name: gnavi.name.to_owned(),
            addr: gnavi.address.to_owned(),
            lat: gnavi.latitude.parse().unwrap(),
            lng: gnavi.longitude.parse().unwrap()
        })
    }
    Ok(restaurants)
}

#[test]
pub fn fetch_gnavi_search() -> Result<(), reqwest::Error>{
    let lat = 35.033633;
    let lng = 135.781037;
    let response: GnaviRes = reqwest::blocking::get(
        format!(
            "https://api.gnavi.co.jp/RestSearchAPI/v3/?keyid={key}&latitude={lat}&longitude={lng}",
            lat=lat, lng=lng, key="75e9a269c366fc995cc6d978441ead40"
        ).as_str()
    )?.json()?;
    dbg!(response);
    Ok(())
}