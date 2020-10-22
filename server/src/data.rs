use crate::{
    db::PostgresPool,
    graphql::{NewPostInput, NewUserInput},
    models::{Post, User},
};

use async_graphql::{FieldError, FieldResult};
use chrono::NaiveDate;
use crypto::{digest::Digest, sha3::Sha3};
use diesel::prelude::*;
use uuid::Uuid;

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}

pub struct PGQuery {
    pub pool: PostgresPool,
}

// Queries
impl PGQuery {
    pub fn validate_user(&self, username: String, password: String) -> FieldResult<User> {
        use super::schema::users::dsl::*;

        let mut passwordhash = Sha3::sha3_256();
        passwordhash.input_str(&password.to_string().as_str());
        let passwordhash = passwordhash.result_str();

        let res = users
            .filter(user_name.eq(username))
            .filter(password_hash.eq(passwordhash))
            .first(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn user(&self, username: String) -> FieldResult<User> {
        use super::schema::users::dsl::*;

        let res = users
            .filter(user_name.eq(username))
            .first(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn post(&self, post_id: Uuid) -> FieldResult<Post> {
        use super::schema::posts::dsl::*;

        let res = posts
            .filter(id.eq(post_id))
            .first(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn posts(&self) -> FieldResult<Vec<Post>> {
        use super::schema::posts::dsl::*;

        let res = posts.load::<Post>(&self.pool.get().unwrap());

        graphql_translate(res)
    }
}

// Mutations
impl PGQuery {
    pub fn add_user(&self, input: NewUserInput) -> FieldResult<User> {
        use super::schema::users;

        let NewUserInput {
            user_name,
            full_name,
            descr,
            email,
            password,
        } = input;

        let mut password_hash = Sha3::sha3_256();
        password_hash.input_str(&password.to_string().as_str());
        let password_hash = password_hash.result_str();

        let new_user = User {
            id: Uuid::new_v4(),
            user_name,
            full_name,
            descr,
            email,
            password_hash,
            article_ids: Some(vec![]),
            draft_ids: Some(vec![]),
        };

        let res = diesel::insert_into(users::table)
            .values(new_user)
            .get_result(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn add_post(&self, input: NewPostInput) -> FieldResult<Post> {
        use super::schema::posts;

        let NewPostInput {
            title,
            date,
            body,
            author,
            published,
        } = input;

        let new_post = Post {
            id: Uuid::new_v4(),
            title,
            date: NaiveDate::parse_from_str(date.as_str(), "%d-%m-%Y")
                .expect("Wrong date value: (Expected: DD-MM-YYYY)"),
            body,
            author,
            published,
        };

        let res = diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn delete_post(&self, id: Uuid) -> FieldResult<Post> {
        todo!()
    }

    pub fn update_post(&self, id: Uuid) -> FieldResult<Post> {
        todo!()
    }
}
