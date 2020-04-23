// use crate::schema::articles::dsl as article_dsl;
// use crate::schema::categorys::dsl as categorys_dsl;
// use crate::models;
// use diesel::{self, prelude::*, SqliteConnection, dsl::not};
// use crate::dto::admin as dto;

// pub fn get_admin_article_briefs(conn: &SqliteConnection) -> Vec<dto::GetArticleBriefDto> {
//     let data = article_dsl::articles.inner_join(categorys_dsl::categorys.on(categorys_dsl::id.eq(article_dsl::category_id)))
//         .load::<(models::Article, models::Category)>(conn).unwrap();
//     data.iter().map(|data| dto::GetArticleBriefDto {
//         id: data.0.id,
//         title: data.0.title.to_string(),
//         create_at: data.0.create_at.format("%Y%m%d").to_string(),
//         category: data.1.label.to_string(),
//         visit_count: data.0.visit_count,
//         published: data.0.published
//     }).collect()
// }

// pub fn get_article_dto(conn: &SqliteConnection, id: i32) -> Option<dto::PostArticleDto> {
//     article_dsl::articles.find(id).first::<models::Article>(conn).ok().and_then(|r| Some(dto::PostArticleDto{
//         id: r.id,
//         title: r.title,
//         body: r.body,
//         category_id: r.category_id,
//         published: r.published,
//     }))
// }

// pub fn insert_article(conn: &SqliteConnection, article: dto::PostArticleDto) -> bool {
//     diesel::insert_into(article_dsl::articles).values((
//         article_dsl::title.eq(article.title),
//         article_dsl::body.eq(article.body),
//         article_dsl::category_id.eq(article.category_id),
//         article_dsl::published.eq(article.published)
//     )).execute(conn).is_ok()
// }

// pub fn update_article(conn: &SqliteConnection, article: dto::PostArticleDto) -> bool {
//     let target = article_dsl::articles.filter(article_dsl::id.eq(article.id));
//     diesel::update(target).set((
//         article_dsl::title.eq(article.title),
//         article_dsl::body.eq(article.body),
//         article_dsl::category_id.eq(article.category_id),
//         article_dsl::published.eq(article.published)
//     )).execute(conn).is_ok()
// }

// pub fn get_all_categorys(conn: &SqliteConnection) -> Vec<dto::CategoryDto> {
//     let result = categorys_dsl::categorys.load::<models::Category>(conn).unwrap();
//     result.iter().map(|c| dto::CategoryDto {
//         id: c.id,
//         label: c.label.to_string()
//     }).collect()
// }

// pub fn toggle_article_published(conn: &SqliteConnection, id: i32) -> bool {
//     let target = article_dsl::articles.filter(article_dsl::id.eq(id));
//     diesel::update(target).set(
//         article_dsl::published.eq(not(article_dsl::published))
//     ).execute(conn).is_ok()
// }

// pub fn insert_category(conn: &SqliteConnection, category: dto::CategoryDto) -> bool {
//     diesel::insert_into(categorys_dsl::categorys).values(categorys_dsl::label.eq(category.label)).execute(conn).is_ok()
// }

// pub fn update_category(conn: &SqliteConnection, category: dto::CategoryDto) -> bool {
//     let target = categorys_dsl::categorys.filter(categorys_dsl::id.eq(category.id));
//     diesel::update(target).set(categorys_dsl::label.eq(category.label)).execute(conn).is_ok()
// }
