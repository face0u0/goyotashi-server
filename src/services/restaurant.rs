use crate::{
    models::{Restaurant, NoIdPin, Pin, User},
    mapper,
    logic,
    errors::*
};

pub fn index_included_by(community_id: i32) -> Result<Vec<Restaurant>, ErrCode>{
    mapper::restaurant::find_all_pined_by(&community_id)
}

pub fn add(current: User, no_id_pin: NoIdPin) -> Result<Pin, ErrCode>{
    logic::privilege::check_joined(&current, &no_id_pin.community_id)?;
    mapper::pin::create(&no_id_pin)
}
