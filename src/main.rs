#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


mod repo;
mod models;
mod router;
mod schema;
mod handlers;
mod connection;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    router::create_routes();
}
