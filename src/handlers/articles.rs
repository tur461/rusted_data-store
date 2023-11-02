

use rocket_contrib;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use diesel::result::Error as  DieselErr;

use crate::connection::DbCon;
use crate::repo::articles as article_repo;
use crate::models::articles::{Article, NewArticle};

fn err_status(e: DieselErr) -> Status {
    match e {
        DieselErr::NotFound => Status::NotFound,
        _ => Status::BadRequest,
    }
}


#[get("/")]
pub fn all_articles(
    mut con: DbCon
) -> Result<Json<Vec<Article>>, Status> {
    article_repo::show_articles(&mut con)
    .map(|article| Json(article))
    .map_err(|e| err_status(e))
}

#[post("/", format="application/json", data="<new_article>")]
pub fn create_article(
    new_article: Json<NewArticle>, 
    mut con: DbCon
) -> Result<status::Created<Json<Article>>, Status> {
    article_repo::create_article(new_article.into_inner(), &mut con)
    .map(|article| status::Created("200".to_string(), Some(Json(article))))
    .map_err(|e| err_status(e))
}

#[get("/<id>")]
pub fn get_article(
    id: i32, 
    mut con: DbCon
) -> Result<Json<Article>, Status> {
    article_repo::get_article(id, &mut con)
    .map(|article| Json(article))
    .map_err(|e| err_status(e))
}

#[post("/<id>", format="application/json", data="<article>")]
pub fn update_article(
    id: i32, 
    article: Json<Article>, 
    mut con: DbCon
) -> Result<Json<Article>, Status> {
    article_repo::update_article(id, article.into_inner(), &mut con)
    .map(|article| Json(article))
    .map_err(|e| err_status(e))
}

#[delete("/<id>")] 
pub fn delete_article(
    id: i32, 
    mut con: DbCon
) -> Result<status::NoContent, Status> {
    article_repo::delete_article(id, &mut con)
    .map(|_| status::NoContent)
    .map_err(|e| err_status(e))
}