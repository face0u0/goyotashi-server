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

pub fn find_community(_id: i32) -> Community {
    return community::select_community(_id).unwrap()
}

pub fn create_community(no_id_community: NoIdCommunity) -> Community {
    return community::insert_community(&no_id_community).unwrap()
}

pub fn update_community(new_community: Community) -> Community {
    return community::update_community(&new_community).unwrap();
}