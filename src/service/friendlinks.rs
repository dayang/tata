use super::err_str;
use crate::dto::friendlink::*;
use crate::entity::FriendLink;
use crate::schema::friendlink::dsl::*;
use diesel::prelude::*;

pub fn all_friendlinks(conn: &SqliteConnection, only_show: bool) -> Result<Vec<FriendLink>, String> {
    if only_show {
        friendlink
            .filter(show.eq(true))
            .load::<FriendLink>(conn)
            .map_err(err_str)
    } else {
        friendlink
            .load::<FriendLink>(conn)
            .map_err(err_str)
    }
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


pub fn get_friendlink(conn: &SqliteConnection, link_id: i32) -> Result<FriendLink, String> {
    friendlink.find(link_id).first::<FriendLink>(conn).map_err(err_str)
}


pub fn add(conn: &SqliteConnection, link_field: FriendLink) -> Result<usize, String> {
    diesel::insert_into(friendlink).values((
        display_text.eq(link_field.display_text),
        link.eq(link_field.link),
        show.eq(link_field.show),
        remark.eq(link_field.remark)
    )).execute(conn).map_err(err_str)
}

pub fn update(conn: &SqliteConnection, link_field: FriendLink) -> Result<usize, String> {
    diesel::update(&link_field).set(&link_field).execute(conn).map_err(err_str)
}

pub fn delete(conn: &SqliteConnection, delete_id: i32) -> Result<usize, String> {
    diesel::delete(friendlink.filter(id.eq(delete_id))).execute(conn).map_err(err_str)
}