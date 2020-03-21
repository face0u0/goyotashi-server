use crate::{
    models::{Community}
};

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

pub fn find_community(_id: u32) -> Community {
    return Community {
        id: _id,
        name: "PALLET".to_string(),
        description: "GOYOTASHI FIRST".to_string(),
        public: true
    }
}

pub fn create_community(name: &str, description: &str, public: bool) -> Community {
    return Community{
        id: 0,
        name: name.to_string(),
        description: description.to_string(),
        public
    }
}

pub fn update_community(_id: u32, name: &str, description: &str, public: bool) -> Community {
    return Community{
        id: _id,
        name: name.to_string(),
        description: description.to_string(),
        public
    }
}