use super::err_str;
use crate::consts::*;
use crate::dto::dict::Dicts;
use crate::schema::dict::dsl as dict_dsl;
use diesel::prelude::*;

pub fn get_dict_value(key: &str, conn: &SqliteConnection) -> Option<String> {
    dict_dsl::dict
        .find(key.to_string())
        .select(dict_dsl::d_value)
        .first::<String>(conn)
        .ok()
}

pub fn set_dict_value<T: ToString>(
    key: &str,
    value: T,
    conn: &SqliteConnection,
) -> Result<usize, String> {
    match dict_dsl::dict
        .find(key.to_string())
        .first::<crate::entity::Dict>(conn)
        .ok()
    {
        Some(dict_item) => diesel::update(&dict_item)
            .set(dict_dsl::d_value.eq(value.to_string()))
            .execute(conn)
            .map_err(err_str),
        None => diesel::insert_into(dict_dsl::dict)
            .values((
                dict_dsl::d_key.eq(key.to_string()),
                dict_dsl::d_value.eq(value.to_string()),
            ))
            .execute(conn)
            .map_err(err_str),
    }
}

pub fn get_dicts(conn: &SqliteConnection) -> Result<Dicts, String> {
    Ok(Dicts {
        index_title: get_dict_value(DICT_INDEX_TITLE, conn).unwrap_or("".into()),
        meta_key_words: get_dict_value(DICT_KEYWORDS, conn).unwrap_or("".into()),
        meta_description: get_dict_value(DICT_DESCRIPTION, conn).unwrap_or("".into()),
        site_info: get_dict_value(DICT_SITE_INFO, conn).unwrap_or("".into()),
        copyright: get_dict_value(DICT_COPYRIGHT, conn).unwrap_or("".into()),
        post_page_num: get_dict_value(DICT_POST_PAGE_NUM, conn).unwrap_or("".into()),
        comment_page_num: get_dict_value(DICT_COMMENT_PAGE_NUM, conn).unwrap_or("".into()),
        common_scripts: get_dict_value(DICT_SCRIPTS, conn).unwrap_or("".into()),
        index_quote: get_dict_value(DICT_INDEX_QUOTE, conn).unwrap_or("".into()),
        about_page: get_dict_value(DICT_ABOUT_PAGE, conn).unwrap_or("".into()),
        avatar_url: get_dict_value(DICT_AVATAR_URL, conn).unwrap_or("".into()),
    })
}

pub fn update_dicts(conn: &SqliteConnection, dicts: Dicts) -> Result<usize, String> {
    set_dict_value(DICT_INDEX_TITLE, dicts.index_title, conn)?;
    set_dict_value(DICT_KEYWORDS, dicts.meta_key_words, conn)?;
    set_dict_value(DICT_DESCRIPTION, dicts.meta_description, conn)?;
    set_dict_value(DICT_SITE_INFO, dicts.site_info, conn)?;
    set_dict_value(DICT_COPYRIGHT, dicts.copyright, conn)?;
    set_dict_value(DICT_POST_PAGE_NUM, dicts.post_page_num, conn)?;
    set_dict_value(DICT_COMMENT_PAGE_NUM, dicts.comment_page_num, conn)?;
    set_dict_value(DICT_SCRIPTS, dicts.common_scripts, conn)?;
    set_dict_value(DICT_INDEX_QUOTE, dicts.index_quote, conn)?;
    set_dict_value(DICT_AVATAR_URL, dicts.avatar_url, conn)?;
    set_dict_value(DICT_ABOUT_PAGE, dicts.about_page, conn)
}
