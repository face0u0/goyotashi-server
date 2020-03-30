use crate::{
    models::{NoIdMember, Member, User},
    mapper,
    errors::*
};

pub fn join(current: User, community_id: i32) -> Result<Member, ErrCode>{
    let no_id_member = NoIdMember{
        user_id: current.id,
        community_id
    };
    mapper::member::create(&no_id_member)
}
