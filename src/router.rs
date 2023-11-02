use rocket;

use crate::handlers::posts as post_handlers;
use crate::handlers::articles as article_handlers;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
    .manage(connection::init_pool())
    .mount("/posts", routes![
        post_handlers::get_post,
        post_handlers::all_posts,
        post_handlers::create_post,
        post_handlers::update_post,
        post_handlers::delete_post,
        ])
    .mount("/articles", routes![
        article_handlers::get_article,
        article_handlers::all_articles,
        article_handlers::create_article,
        article_handlers::update_article,
        article_handlers::delete_article,
    ]).launch();
}