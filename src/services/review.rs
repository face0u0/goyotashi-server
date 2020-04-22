use crate::{
    models::{NoIdReview, User, NoIdMember, Member},
    mapper,
    errors::*
};

pub fn add_or_update(current: User, pin_id: i32, good: bool, comment: Option<String>) -> Result<NoIdReview, ErrCode>{
    let member = find_member(current.id, pin_id)?;
    let no_id_review = NoIdReview {
        pin_id,
        member_id: member.id,
        good,
        comment
    };
    mapper::review::create_or_update(&no_id_review)
}

fn find_member(user_id: i32, pin_id: i32) -> Result<Member, ErrCode>{
    let pin = mapper::pin::find(pin_id)?;
    let no_id_member = NoIdMember{
        user_id,
        community_id: pin.community_id
    };
    mapper::member::find(&no_id_member)
}