use crate::{
    models::{Community, NoIdCommunity, User, NoIdMember},
    mapper,
    logic,
    errors::*
};

pub fn search_by_name(search: String) -> Result<Vec<Community>, ErrCode> {
    mapper::community::find_all_by_name(&search)
}

pub fn find(_id: i32) -> Result<Community, ErrCode> {
    mapper::community::find(&_id)
}

pub fn create(current: User, no_id_community: NoIdCommunity) -> Result<Community, ErrCode> {
    let community = mapper::community::create(&no_id_community);
    community.and_then(|com| {
        let no_id_member = NoIdMember{community_id: com.id, user_id: current.id};
        mapper::member::create(&no_id_member).map(|_| com)
    })
}

pub fn update(current: User, _id: i32, new_community_data: NoIdCommunity) -> Result<Community, ErrCode> {
    logic::privilege::check_joined(&current, &_id)
        .and_then( |_| mapper::community::update(&_id, &new_community_data))
}