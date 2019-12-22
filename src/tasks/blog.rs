use crate::schemas::articles::dsl as article_dsl;
use crate::schemas::categorys::dsl as categorys_dsl;
use crate::models;
use diesel::{self, prelude::*, SqliteConnection};
use crate::dto::blog as dto;

pub fn get_article_brief_by_id(conn: &SqliteConnection, article_id: i32, filter_published: bool) -> Option<dto::ArticleBrief> {
    let mut stmt = article_dsl::articles.inner_join(categorys_dsl::categorys.on(categorys_dsl::id.eq(article_dsl::category_id)))
        .filter(article_dsl::id.eq(article_id)).into_boxed();

    if filter_published {
        stmt = stmt.filter(article_dsl::published.eq(true))
    }

    // 更新访问次数，不管结果是否成功
    let target = article_dsl::articles.find(article_id);
    let _result = diesel::update(target).set(article_dsl::visit_count.eq(article_dsl::visit_count + 1)).execute(conn);

    stmt.first::<(models::Article, models::Category)>(conn)
        .ok()
        .and_then(|data| Some(dto::ArticleBrief {
            id: data.0.id,
            title: data.0.title.to_string(),
            create_at: data.0.create_at.format("%Y-%m-%d").to_string(),
            category: data.1.label.to_string(),
            visit_count: data.0.visit_count,
        }))
}

pub fn get_article_briefs(conn: &SqliteConnection) -> Option<Vec<dto::ArticleBrief>> {
    article_dsl::articles.inner_join(categorys_dsl::categorys.on(categorys_dsl::id.eq(article_dsl::category_id)))
        .filter(article_dsl::published.eq(true))
        .load::<(models::Article, models::Category)>(conn).ok()
        .and_then(|data| {
            Some(data.iter().map(|data| dto::ArticleBrief {
                id: data.0.id,
                title: data.0.title.to_string(),
                create_at: data.0.create_at.format("%Y-%m-%d").to_string(),
                category: data.1.label.to_string(),
                visit_count: data.0.visit_count,
            }).collect())
        })
}

pub fn get_article_body_by_id(conn: &SqliteConnection, id: i32, filter_published: bool) -> Option<String> {
    let mut query = article_dsl::articles.select(article_dsl::body).filter(article_dsl::id.eq(id)).into_boxed();
    if filter_published {
        query = query.filter(article_dsl::published.eq(true));
    }
    query.first::<String>(conn).ok()
}