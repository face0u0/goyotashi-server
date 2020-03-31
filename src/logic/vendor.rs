use reqwest;
// use serde_json::from_str;
use serde::{Deserialize, Serialize};

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