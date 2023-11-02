

use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::posts;
use crate::models::posts::Post;
use crate::schema::posts::dsl::*;
use crate::models::posts::NewPost;

pub fn create_post(new_post: NewPost, con: &mut PgConnection) -> QueryResult<Post> {
    diesel::insert_into(posts::table)
    .values(&new_post)
    .get_result(con)
}

pub fn show_posts(con: &mut PgConnection) -> QueryResult<Vec<Post>> {
    posts.limit(5).load::<Post>(con)
}

pub fn get_post(post_id: i32, con: &mut PgConnection) -> QueryResult<Post> {
    posts.find(post_id).get_result::<Post>(con)
}

pub fn update_post(post_id: i32, post: Post, con: &mut PgConnection) -> QueryResult<Post> {
    diesel::update(posts.find(post_id)).set(&post).get_result(con)
}
pub fn delete_post(post_id: i32, con: &mut PgConnection) -> Result<usize, Error> {
    diesel::delete(posts.find(post_id)).execute(con)
}

