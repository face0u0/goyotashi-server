use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseErr {
    pub msg: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NoIdCommunity {
    pub name: String,
    pub description: String,
    pub public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub uid: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoIdUser {
    pub name: String,
    pub email: String,
    pub uid: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowUser {
    pub id: i32,
    pub email: String,
    pub name: String,
}