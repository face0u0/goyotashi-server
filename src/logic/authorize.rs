use crate::{
    errors::*,
    models::{User, NoIdUser},
    mapper
};
use jsonwebtoken;
use serde::{Deserialize, Serialize};

pub fn auth(jwt: &String) -> Result<User, ErrCode>{
    let claims = extract_uid(jwt.to_owned());
    let user_err = mapper::user::find_by_uid(&claims.sub);
    return match user_err {
        Ok(_) => user_err,
        Err(err) => {
            match err.status {
                Stat::NotFound => {
                    let new_user = NoIdUser {
                        name: claims.name,
                        email: claims.email,
                        uid: claims.sub
                    };
                    let created = mapper::user::create(&new_user);
                    created
                },
                _ => Err(err)
            }
        }
    }
}


#[test]
fn test_auth(){
    let id_token = sample_jwt();
    let user = auth(&id_token).ok().unwrap();
    assert_eq!(user.email, "matsuyamapopo4@gmail.com");
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims{
    pub exp: u32,
    pub iat: u32,
    pub aud: String,
    pub iss: String,
    pub sub: String,
    pub auth_time: u32,
    pub name: String,
    pub email: String
}

#[test]
// extract correct uid from sample idToken?
fn test_decode(){
    let id_token = sample_jwt();
    let claims= extract_uid(id_token);
    assert_eq!(claims.sub, "TzTQggnt0ob9Cfs9q39bzQt5xdV2");
    assert_eq!(claims.name, "hiroki matsuyama");
    assert_eq!(claims.email, "matsuyamapopo4@gmail.com")
}

pub fn sample_jwt() -> String{
    let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjFmODhiODE0MjljYzQ1MWEzMzVjMmY1Y2RiM2RmYjM0ZWIzYmJjN2YiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiaGlyb2tpIG1hdHN1eWFtYSIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vLU5IREVtQUw5dmpJL0FBQUFBQUFBQUFJL0FBQUFBQUFBQUFBL0FDSGkzcmY1ZEtnaVZtNm8wQ3B2OTlYLW1qRmxRRDRfVVEvcGhvdG8uanBnIiwiaXNzIjoiaHR0cHM6Ly9zZWN1cmV0b2tlbi5nb29nbGUuY29tL2dveW91dGFzaGktMzJiMjciLCJhdWQiOiJnb3lvdXRhc2hpLTMyYjI3IiwiYXV0aF90aW1lIjoxNTg1NDA3NDMxLCJ1c2VyX2lkIjoiVHpUUWdnbnQwb2I5Q2ZzOXEzOWJ6UXQ1eGRWMiIsInN1YiI6IlR6VFFnZ250MG9iOUNmczlxMzlielF0NXhkVjIiLCJpYXQiOjE1ODU0MDc0MzIsImV4cCI6MTU4NTQxMTAzMiwiZW1haWwiOiJtYXRzdXlhbWFwb3BvNEBnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiZmlyZWJhc2UiOnsiaWRlbnRpdGllcyI6eyJnb29nbGUuY29tIjpbIjExMDA1MDA5ODAxMTM5MjI4MjYzMSJdLCJlbWFpbCI6WyJtYXRzdXlhbWFwb3BvNEBnbWFpbC5jb20iXX0sInNpZ25faW5fcHJvdmlkZXIiOiJnb29nbGUuY29tIn19.mn5w3XZB_DQPVjypvM_SxgR5oCxKhvmI7EeuUNEqktUiRgzOlj4tWv4bZmD31xGDBpYM0ZEwqQ8HbuyeXyXxoLSsiWgLmOv7JywKJL5PqhSZC-xSOOc53Ig_wzH_IVSmGwcugZlKc6nCriLEMvSjLrYMHmFLQZDAZewxqz8BsRyl95j_jmnlakHbb55HFrelyBZRO8MBCV97BCfafLiINGYyHBFGtXFVA4uG-5AMaCTWXgPzpAroeq_KIDVoTEv_RwrQlHvvdiXnqzQDbkTjrmyvgd_YQiQSGImrMj6StBGOqkODaHMrFTwzOiTGSxYoMpURF9_3cIcPS9myiq6Zvw".to_string();
    return id_token;
}

fn decode_token(token: String) -> Claims {
    let decoded = jsonwebtoken::dangerous_unsafe_decode(&token).unwrap();
    decoded.claims
}

fn extract_uid(token: String) -> Claims {
    let claims = decode_token(token);
    claims
}