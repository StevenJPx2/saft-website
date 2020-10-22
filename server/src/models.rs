use chrono::NaiveDate;
use uuid::Uuid;

use super::schema::{posts, users};

#[derive(Queryable, Insertable, Clone)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub date: NaiveDate,
    pub body: String,
    pub author: String,
    pub published: bool,
}

#[derive(Queryable, Insertable, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub full_name: String,
    pub descr: String,
    pub email: String,
    pub password_hash: String,
    // TODO: Create associations for foreign keys
    pub article_ids: Option<Vec<Uuid>>,
    pub draft_ids: Option<Vec<Uuid>>,
}
