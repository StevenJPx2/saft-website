use async_graphql::*;
use chrono::NaiveDate;

#[derive(SimpleObject)]
pub struct User {
    id: ID,
    user_name: String,
    full_name: String,
    email: String,
    descr: String,
    password_hash: String,
    articles: Vec<Post>,
    drafts: Vec<Post>,
}

#[derive(SimpleObject)]
pub struct Post {
    id: ID,
    title: String,
    date: NaiveDate,
    body: String,
    author: User,
    draft: bool,
}

pub struct Query;

#[Object]
impl Query {
    async fn validate_user(&self, user_name: String, password_hash: String) -> Result<&User> {
        todo!()
    }
    async fn user(&self, user_name: String) -> Result<&User> {
        todo!()
    }

    async fn me(&self) -> Result<&User> {
        todo!()
    }

    async fn post(&self, id: ID) -> Result<&User> {
        todo!()
    }

    async fn posts(&self) -> Result<&Vec<Post>> {
        todo!()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_post(&self) -> Result<String> {
        todo!()
    }

    async fn delete_post(&self) -> Result<String> {
        todo!()
    }

    async fn update_post(&self) -> Result<String> {
        todo!()
    }
}
