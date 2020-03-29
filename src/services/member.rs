use crate::{
    models::{NoIdMember, Member},
    mapper,
    logic,
    errors::*
};

pub fn join(jwt: String, community_id: i32) -> Result<Member, ErrCode>{
    let user = logic::authorize::auth(&jwt)?;
    let no_id_member = NoIdMember{
        user_id: user.id,
        community_id
    };
    mapper::member::create(&no_id_member)
}
