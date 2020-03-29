use crate::{
    models::{ShowUser},
    mapper,
    errors::*
};

pub fn show(_id: i32) -> Result<ShowUser, ErrCode>{
    mapper::user::find(&_id)
        .map(|user| -> ShowUser {
            ShowUser {id: user.id, name: user.name, email: user.email}
        })
}
