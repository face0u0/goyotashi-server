use reqwest;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn fetch_firebase_key(kid: &String) -> String {
    let fetched_map = reqwest::blocking::get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com").unwrap()
        .json::<HashMap<String, String>>().unwrap();
    fetched_map.get(kid).unwrap().to_string()
}