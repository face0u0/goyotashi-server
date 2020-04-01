use crate::{
    errors::*,
    models::{User, NoIdUser},
    mapper
};
use jsonwebtoken;
use serde::{Deserialize, Serialize};

pub fn auth(jwt: &String) -> Result<User, ErrCode>{
    let claims = extract_uid(jwt.to_owned())?;
    mapper::user::insert_or_update(&NoIdUser{
        name: claims.name,
        email: claims.email,
        uid: claims.sub,
        icon: claims.picture
    })
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
    pub email: String,
    pub picture: String
}

fn decode_token(token: String) -> Result<Claims, ErrCode> {
    jsonwebtoken::dangerous_unsafe_decode(&token)
        .map_err(|_| ErrCode::new(Stat::Unauthorized, "token cannot parse. check token is valid."))
        .map(|decoded| decoded.claims)
}

fn extract_uid(token: String) -> Result<Claims, ErrCode> {
    let claims = decode_token(token)?;
    Ok(claims)
}