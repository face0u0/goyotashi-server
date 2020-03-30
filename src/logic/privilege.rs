use crate::{
    mapper,
    models::{User, NoIdMember},
    errors::*,
};

pub fn check_joined(user: &User, community_id: &i32) -> Result<(), ErrCode>{
    let no_id_member = NoIdMember{user_id: user.id, community_id: community_id.to_owned()};
    mapper::member::check_joined(&no_id_member)
        .and_then(|judge| if judge {Ok(())} else { Err(ErrCode::new(Stat::ForBitten, "not joined community!")) })
}