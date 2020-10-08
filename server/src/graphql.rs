use chrono::{NaiveDate, Utc};
use juniper::{Executor, FieldResult, ID};
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/schema.graphql");

pub struct Html;

pub struct User {
    id: ID,
    user_name: String,
    full_name: String,
    desc: Html,
    password_hash: String,
    articles: Vec<Post>,
    drafts: Vec<Post>,
}

impl UserFields for User {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&ID> {
        Ok(&self.id)
    }
    fn field_user_name(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.user_name)
    }
    fn field_full_name(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.full_name)
    }
    fn field_desc(&self, executor: &Executor<'_, Context>) -> FieldResult<&Html> {
        Ok(&self.desc)
    }
    fn field_password_hash(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.password_hash)
    }
    fn field_articles(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
    ) -> FieldResult<&Vec<Post>> {
        Ok(&self.articles)
    }
    fn field_drafts(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
    ) -> FieldResult<&Vec<Post>> {
        Ok(&self.drafts)
    }
}

pub struct Post {
    id: ID,
    title: String,
    date: NaiveDate,
    body: HTML,
    author: User,
}

impl PostFields for Post {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&ID> {
        Ok(&self.id)
    }
    fn field_title(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.title)
    }
    fn field_date(&self, executor: &Executor<'_, Context>) -> FieldResult<&NaiveDate> {
        Ok(&self.date)
    }
    fn field_body(&self, executor: &Executor<'_, Context>) -> FieldResult<&Html> {
        Ok(self.body)
    }
    fn field_author(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<&User> {
        Ok(self.author)
    }
}

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;
impl QueryFields for Query {
    fn field_validate_user(
        &self,
        _: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
        user_name: String,
        password_hash: String,
    ) -> FieldResult<&Option<User>> {
        Ok(self.stuff)
    }
    fn field_user(
        &self,
        _: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
        user_name: String,
    ) -> FieldResult<&Option<User>> {
        Ok(self.stuff)
    }

    fn field_me(
        &self,
        _: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<&User> {
        Ok(self.stuff)
    }

    fn field_post(
        &self,
        _: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
        id: ID,
    ) -> FieldResult<&Option<Post>> {
        Ok(self.stuff)
    }

    fn field_posts(
        &self,
        _: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Post, Walked>,
    ) -> FieldResult<&Option<Vec<Post>>> {
        Ok(&Some(self.posts))
    }
}

pub struct Mutation;
impl MutationFields for Mutation {
    fn field_add_post(&self, _: &Executor<'_, Context>, post: Post) -> FieldResult<&String> {
        Ok(&"Added Post!".to_string())
    }
    fn field_delete_post(&self, _: &Executor<'_, Context>, post: Post) -> FieldResult<&String> {
        Ok(&"Deleted Post!".to_string())
    }
    fn field_update_post(&self, _: &Executor<'_, Context>, post: Post) -> FieldResult<&String> {
        Ok(&"Updated Post!".to_string())
    }
}
