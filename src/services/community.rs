use crate::{
    models::{Community},
    mapper::{community},
    errors::*
};
use crate::models::NoIdCommunity;

pub fn search_by_name(_: &str) -> Vec<Community> {
    return vec![
        Community {
            id: 8198,
            name: "PALLET".to_string(),
            description: "GOYOTASHI FIRST".to_string(),
            public: true
        }
    ]
}

pub fn find(_id: i32) -> Result<Community, ErrCode> {
    community::find(_id)
}

pub fn create(no_id_community: NoIdCommunity) -> Result<Community, ErrCode> {
    community::create(&no_id_community)
}

pub fn update(new_community: Community) -> Result<Community, ErrCode> {
    community::update(&new_community)
}