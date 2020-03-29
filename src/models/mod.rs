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

#[derive(Debug, Serialize, Deserialize)]
pub struct Member{
    pub id: i32,
    pub user_id: i32,
    pub community_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoIdMember{
    pub user_id: i32,
    pub community_id: i32,
}