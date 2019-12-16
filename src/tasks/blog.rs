use crate::schemas::articles::dsl as article_dsl;
use crate::schemas::categorys::dsl as categorys_dsl;
use crate::models;
use diesel::{self, prelude::*, SqliteConnection};
use crate::dto::blog as dto;

pub fn get_article_brief_by_id(conn: &SqliteConnection, article_id: i32) -> dto::ArticleBrief {
    let data = article_dsl::articles.inner_join(categorys_dsl::categorys.on(categorys_dsl::id.eq(article_dsl::category_id)))
        .filter(article_dsl::id.eq(article_id))
        .filter(article_dsl::published.eq(true)).first::<(models::Article, models::Category)>(conn).unwrap();
    dto::ArticleBrief {
        id: data.0.id,
        title: data.0.title.to_string(),
        create_at: data.0.create_at.format("%Y-%m-%d").to_string(),
        category: data.1.label.to_string(),
        visit_count: data.0.visit_count,
    }
}

pub fn get_article_briefs(conn: &SqliteConnection) -> Vec<dto::ArticleBrief> {
    let data = article_dsl::articles.inner_join(categorys_dsl::categorys.on(categorys_dsl::id.eq(article_dsl::category_id)))
        .filter(article_dsl::published.eq(true))
        .load::<(models::Article, models::Category)>(conn).unwrap();
    data.iter().map(|data| dto::ArticleBrief {
        id: data.0.id,
        title: data.0.title.to_string(),
        create_at: data.0.create_at.format("%Y-%m-%d").to_string(),
        category: data.1.label.to_string(),
        visit_count: data.0.visit_count,
    }).collect()
}

pub fn get_article_body_by_id(conn: &SqliteConnection, id: i32) -> String {
    article_dsl::articles.select(article_dsl::body).filter(article_dsl::id.eq(id)).first::<String>(conn).unwrap()
}