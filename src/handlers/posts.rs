

use rocket_contrib;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use diesel::result::Error as  DieselErr;

use crate::connection::DbCon;
use crate::repo::posts as posts_repo;
use crate::models::posts::{Post, NewPost};

fn err_status(e: DieselErr) -> Status {
    match e {
        DieselErr::NotFound => Status::NotFound,
        _ => Status::BadRequest,
    }
}


#[get("/")]
pub fn all_posts(mut con: DbCon) -> Result<Json<Vec<Post>>, Status> {
    posts_repo::show_posts(&mut con).map(|post| Json(post)).map_err(|e| err_status(e))
}

#[post("/", format="application/json", data="<new_post>")]
pub fn create_post(new_post: Json<NewPost>, mut con: DbCon) -> Result<status::Created<Json<Post>>, Status> {
    posts_repo::create_post(new_post.into_inner(), &mut con).map(|post| status::Created("200".to_string(), Some(Json(post)))).map_err(|e| err_status(e))
}

#[get("/<id>")]
pub fn get_post(id: i32, mut con: DbCon) -> Result<Json<Post>, Status> {
    posts_repo::get_post(id, &mut con).map(|post| Json(post)).map_err(|e| err_status(e))
}

#[post("/<id>", format="application/json", data="<post>")]
pub fn update_post(id: i32, post: Json<Post>, mut con: DbCon) -> Result<Json<Post>, Status> {
    posts_repo::update_post(id, post.into_inner(), &mut con).map(|post| Json(post)).map_err(|e| err_status(e))
}

#[delete("/<id>")] 
pub fn delete_post(id: i32, mut con: DbCon) -> Result<status::NoContent, Status> {
    posts_repo::delete_post(id, &mut con).map(|_| status::NoContent).map_err(|e| err_status(e))
}