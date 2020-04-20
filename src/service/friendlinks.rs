use crate::entity::FriendLink;
use diesel::prelude::*;
use crate::schema::friendlink::dsl::*;
use super::err_str;

pub fn all_friendlinks(conn: &SqliteConnection) -> Result<Vec<FriendLink>, String> {
    friendlink.load::<FriendLink>(conn).map_err(err_str)
}