use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub public: bool,
}