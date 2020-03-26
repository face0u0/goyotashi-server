use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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