use crate::{
    models::{Community}
};

pub fn get_community(_: &str) -> Vec<Community> {
    return vec![
        Community {
            id: 8198,
            name: "PALLET".to_string(),
            description: "GOYOTASHI FIRST".to_string(),
            public: true
        }
    ]
}

pub fn find_community(_id: u32) -> Community {
    return Community {
        id: 8198,
        name: "PALLET".to_string(),
        description: "GOYOTASHI FIRST".to_string(),
        public: true
    }
}