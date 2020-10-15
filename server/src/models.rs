use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub date: NaiveDate,
    pub body: String,
    pub author_id: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub full_name: String,
    pub descr: String,
    pub email: String,
    pub password_hash: String,
    pub article_ids: Vec<Uuid>,
    pub draft_ids: Vec<Uuid>,
}
