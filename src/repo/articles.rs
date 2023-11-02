

use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::articles;
use crate::models::articles::Article;
use crate::schema::articles::dsl::*;
use crate::models::articles::NewArticle;

pub fn create_article(
    new_article: NewArticle, 
    con: &mut PgConnection
) -> QueryResult<Article> {
    diesel::insert_into(articles::table)
    .values(&new_article)
    .get_result(con)
}

pub fn show_articles(
    con: &mut PgConnection
) -> QueryResult<Vec<Article>> {
    articles
    .limit(5)
    .load::<Article>(con)
}

pub fn get_article(
    article_id: i32, 
    con: &mut PgConnection
) -> QueryResult<Article> {
    articles
    .find(article_id)
    .get_result::<Article>(con)
}

pub fn update_article(
    article_id: i32, 
    article: Article, 
    con: &mut PgConnection
) -> QueryResult<Article> {
    diesel::update(articles.find(article_id))
    .set(&article)
    .get_result(con)
}
pub fn delete_article(
    article_id: i32, 
    con: &mut PgConnection
) -> Result<usize, Error> {
    diesel::delete(articles.find(article_id))
    .execute(con)
}

