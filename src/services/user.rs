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

pub fn index_included_by(community_id: i32) -> Result<Vec<ShowUser>, ErrCode>{
    mapper::user::find_all_included_by(&community_id)
        .map(|users| -> Vec<ShowUser>{
            let mut show_users: Vec<ShowUser> = Vec::new();
            for user in &users {
                show_users.push(ShowUser{
                    id: user.id.to_owned(),
                    email: user.email.to_owned(),
                    name: user.name.to_owned()
                })
            }
            show_users
        })
}
