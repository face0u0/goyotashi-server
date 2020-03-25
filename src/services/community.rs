use crate::{
    models::{Community},
    mapper::{community}
};
use crate::models::NoIdCommunity;

pub fn search_community(_: &str) -> Vec<Community> {
    return vec![
        Community {
            id: 8198,
            name: "PALLET".to_string(),
            description: "GOYOTASHI FIRST".to_string(),
            public: true
        }
    ]
}

pub fn find(_id: i32) -> Community {
    return community::find(_id).unwrap()
}

pub fn create(no_id_community: NoIdCommunity) -> Community {
    return community::create(&no_id_community).unwrap()
}

pub fn update(new_community: Community) -> Community {
    return community::update(&new_community).unwrap();
}