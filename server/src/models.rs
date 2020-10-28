use async_graphql::InputObject;
use chrono::NaiveDate;
use uuid::Uuid;

use super::schema::{posts, users};

#[derive(Queryable, Insertable, Clone, Debug)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub date: NaiveDate,
    pub body: String,
    pub author: String,
    pub published: bool,
}

#[derive(Queryable, Insertable, Identifiable, Clone, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub full_name: String,
    pub descr: String,
    pub email: String,
    pub password_hash: String,
    // TODO: Create associations for foreign keys
    pub article_ids: Vec<Uuid>,
    pub draft_ids: Vec<Uuid>,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserUpdate {
    pub user_name: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub descr: Option<String>,
    pub password_hash: Option<String>,
    pub article_ids: Option<Vec<Uuid>>,
    pub draft_ids: Option<Vec<Uuid>>,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostUpdate {
    pub title: Option<String>,
    pub date: Option<NaiveDate>,
    pub body: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
}
