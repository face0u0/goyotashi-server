use jsonwebtoken;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Claims{
    exp: u32,
    iat: u32,
    aud: String,
    iss: String,
    sub: String,
    auth_time: u32
}

#[allow(dead_code)]
fn fetch_firebase_key(kid: &String) -> String {
   let fetched_map = reqwest::blocking::get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com").unwrap()
       .json::<HashMap<String, String>>().unwrap();
    fetched_map.get(kid).unwrap().to_string()
}

#[test]
// extract correct uid from sample idToken?
fn test_decode(){
    let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjFmODhiODE0MjljYzQ1MWEzMzVjMmY1Y2RiM2RmYjM0ZWIzYmJjN2YiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiaGlyb2tpIG1hdHN1eWFtYSIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vLU5IREVtQUw5dmpJL0FBQUFBQUFBQUFJL0FBQUFBQUFBQUFBL0FDSGkzcmY1ZEtnaVZtNm8wQ3B2OTlYLW1qRmxRRDRfVVEvcGhvdG8uanBnIiwiaXNzIjoiaHR0cHM6Ly9zZWN1cmV0b2tlbi5nb29nbGUuY29tL2dveW91dGFzaGktMzJiMjciLCJhdWQiOiJnb3lvdXRhc2hpLTMyYjI3IiwiYXV0aF90aW1lIjoxNTg1NDA3NDMxLCJ1c2VyX2lkIjoiVHpUUWdnbnQwb2I5Q2ZzOXEzOWJ6UXQ1eGRWMiIsInN1YiI6IlR6VFFnZ250MG9iOUNmczlxMzlielF0NXhkVjIiLCJpYXQiOjE1ODU0MDc0MzIsImV4cCI6MTU4NTQxMTAzMiwiZW1haWwiOiJtYXRzdXlhbWFwb3BvNEBnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiZmlyZWJhc2UiOnsiaWRlbnRpdGllcyI6eyJnb29nbGUuY29tIjpbIjExMDA1MDA5ODAxMTM5MjI4MjYzMSJdLCJlbWFpbCI6WyJtYXRzdXlhbWFwb3BvNEBnbWFpbC5jb20iXX0sInNpZ25faW5fcHJvdmlkZXIiOiJnb29nbGUuY29tIn19.mn5w3XZB_DQPVjypvM_SxgR5oCxKhvmI7EeuUNEqktUiRgzOlj4tWv4bZmD31xGDBpYM0ZEwqQ8HbuyeXyXxoLSsiWgLmOv7JywKJL5PqhSZC-xSOOc53Ig_wzH_IVSmGwcugZlKc6nCriLEMvSjLrYMHmFLQZDAZewxqz8BsRyl95j_jmnlakHbb55HFrelyBZRO8MBCV97BCfafLiINGYyHBFGtXFVA4uG-5AMaCTWXgPzpAroeq_KIDVoTEv_RwrQlHvvdiXnqzQDbkTjrmyvgd_YQiQSGImrMj6StBGOqkODaHMrFTwzOiTGSxYoMpURF9_3cIcPS9myiq6Zvw".to_string();
    let uid = extract_uid(id_token);
    assert_eq!(uid, "TzTQggnt0ob9Cfs9q39bzQt5xdV2")
}

fn decode_token(token: String) -> Claims {
    let decoded = jsonwebtoken::dangerous_unsafe_decode(&token).unwrap();
    decoded.claims
}

pub fn extract_uid(token: String) -> String {
    let claims = decode_token(token);
    claims.sub
}