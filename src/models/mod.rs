use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseErr {
    pub msg: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header{
    pub method: String,
    pub token: String,
    pub user: User
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
    pub uid: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoIdUser {
    pub name: String,
    pub email: String,
    pub uid: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowUser {
    pub id: i32,
    pub name: String,
    pub icon: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Restaurant{
    pub id: i32,
    pub vendor: i32,
    pub place_id: String,
    pub name: String,
    pub addr: String,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoIdRestaurant{
    pub place_id: String,
    pub vendor: i32,
    pub name: String,
    pub addr: String,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pin{
    pub id: i32,
    pub restaurant_id: i32,
    pub community_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoIdPin{
    pub restaurant_id: i32,
    pub community_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoIdReview{
    pub pin_id: i32,
    pub member_id: i32,
    pub good: bool,
    pub comment: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostedReview{
    pub pin_id: i32,
    pub good: bool,
    pub comment: Option<String>
}