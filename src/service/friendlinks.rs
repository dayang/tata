use super::err_str;
use crate::dto::friendlink::*;
use crate::entity::FriendLink;
use crate::schema::friendlink::dsl::*;
use diesel::prelude::*;

pub fn all_friendlinks(conn: &SqliteConnection) -> Result<Vec<FriendLink>, String> {
    friendlink
        .filter(show.eq(true))
        .load::<FriendLink>(conn)
        .map_err(err_str)
}

pub fn apply_for(conn: &SqliteConnection, apply_req: ApplyFriendLink) -> Result<usize, String> {
    diesel::insert_into(friendlink)
        .values((
            display_text.eq(apply_req.display_text),
            link.eq(apply_req.link),
            show.eq(false),
            remark.eq(apply_req.remark),
        ))
        .execute(conn)
        .map_err(err_str)
}
