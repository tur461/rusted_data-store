use diesel;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable, AsChangeset};

use crate::schema::articles;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "articles"]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_by: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub content: String,
    pub updated_on: NaiveDateTime,
    pub created_by: String,
    pub created_on: NaiveDateTime,
}