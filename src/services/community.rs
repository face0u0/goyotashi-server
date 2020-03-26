use crate::{
    models::{Community},
    mapper,
    errors::*
};
use crate::models::NoIdCommunity;

pub fn search_by_name(search: String) -> Result<Vec<Community>, ErrCode> {
    mapper::community::find_all_by_name(search)
}

pub fn find(_id: i32) -> Result<Community, ErrCode> {
    mapper::community::find(_id)
}

pub fn create(no_id_community: NoIdCommunity) -> Result<Community, ErrCode> {
    mapper::community::create(&no_id_community)
}

pub fn update(_id: i32, new_community_data: NoIdCommunity) -> Result<Community, ErrCode> {
    mapper::community::update(_id, &new_community_data)
}