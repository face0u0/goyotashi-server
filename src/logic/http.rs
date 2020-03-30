use reqwest;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn fetch_firebase_key(kid: &String) -> String {
    let fetched_map = reqwest::blocking::get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com").unwrap()
        .json::<HashMap<String, String>>().unwrap();
    fetched_map.get(kid).unwrap().to_string()
}

#[test]
pub fn fetch_google_place_details(){
    let response = reqwest::blocking::get(
        format!(
            "https://maps.googleapis.com/maps/api/place/details/json?place_id={place_id}&fields=name,geometry&key={key}",
            place_id="ChIJN1t_tDeuEmsRUsoyG83frY4", key="AIzaSyAGUl_pXRtcVB-eIlNBG2oWfcZFrLCSLKc"
        ).as_str()
    ).unwrap();
    dbg!(response);
}