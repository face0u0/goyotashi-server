use crate::{
    models::{Restaurant, NoIdPin, Pin},
    mapper,
    errors::*
};

pub fn index_included_by(community_id: i32) -> Result<Vec<Restaurant>, ErrCode>{
    mapper::restaurant::find_all_pined_by(&community_id)
}

pub fn add(no_id_pin: NoIdPin) -> Result<Pin, ErrCode>{
    mapper::pin::create(&no_id_pin)
}
