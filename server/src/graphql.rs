use async_graphql::*;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::data::PGQuery;

// TODO: Make all functions below return a result for exposing errors to the top-level instead of
// crashing the app

fn translate_post_id_to_post(database: &PGQuery, id_vec: Vec<Uuid>, depth: u8) -> Vec<Post> {
    // TODO: Make logic a little more sensible here

    if depth <= 0 {
        vec![]
    } else {
        id_vec
            .iter()
            .map(|post_id| {
                model_post_to_graphql(
                    database,
                    database.post(post_id.to_owned()).unwrap(),
                    depth - 1,
                )
            })
            .collect()
    }
}

fn model_user_to_graphql(database: &PGQuery, model_user: crate::models::User, depth: u8) -> User {
    let crate::models::User {
        id,
        user_name,
        full_name,
        email,
        descr,
        password_hash,
        article_ids,
        draft_ids,
    } = model_user;

    User {
        id: ID(id.to_string()),
        user_name,
        full_name,
        email,
        descr,
        password_hash,
        articles: translate_post_id_to_post(database, article_ids, depth - 1),
        drafts: translate_post_id_to_post(database, draft_ids, depth - 1),
    }
}

fn model_post_to_graphql(database: &PGQuery, model_post: crate::models::Post, depth: u8) -> Post {
    let crate::models::Post {
        id,
        title,
        date,
        body,
        author,
        published,
    } = model_post;

    Post {
        id: ID(id.to_string()),
        title,
        date,
        body,
        author: {
            // TODO: Make logic a little more sensible here
            if depth <= 0 {
                Author::Guest(Guest { full_name: author })
            } else if let Ok(user) = database.user(author.clone()) {
                Author::User(model_user_to_graphql(database, user, depth - 1))
            } else {
                Author::Guest(Guest { full_name: author })
            }
        },
        published,
    }
}

pub struct ContextData {
    pub db: PGQuery,
}

#[derive(Union, Debug)]
enum Author {
    User(User),
    Guest(Guest),
}

#[derive(SimpleObject, Debug)]
pub struct Guest {
    full_name: String,
}

#[derive(InputObject)]
pub struct NewUserInput {
    pub user_name: String,
    pub full_name: String,
    pub email: String,
    pub descr: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct NewPostInput {
    pub title: String,
    pub date: String,
    pub body: String,
    pub author: String,
    pub published: bool,
}

#[derive(InputObject)]
pub struct UserUpdate {
    pub user_name: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub descr: Option<String>,
    pub password: Option<String>,
}

#[derive(InputObject)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub date: Option<String>,
    pub body: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
}

#[derive(SimpleObject, Debug)]
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

#[derive(SimpleObject, Debug)]
pub struct Post {
    id: ID,
    title: String,
    date: NaiveDate,
    body: String,
    author: Author,
    published: bool,
}

pub struct Query;

#[Object]
impl Query {
    async fn validate_user(
        &self,
        ctx: &Context<'_>,
        user_name: String,
        password: String,
    ) -> Result<bool> {
        let database = &ctx.data::<ContextData>()?.db;
        let res = database.validate_user(user_name, password)?;

        Ok(res)
    }

    async fn user(&self, ctx: &Context<'_>, user_name: String) -> Result<User> {
        let database = &ctx.data::<ContextData>()?.db;
        let user_details = database.user(user_name)?;

        let user = model_user_to_graphql(database, user_details, 2);

        Ok(user)
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let database = &ctx.data::<ContextData>()?.db;
        let model_users = database.users()?;

        Ok(model_users
            .iter()
            .map(|model_user| model_user_to_graphql(database, model_user.clone(), 2))
            .collect())
    }

    async fn post(&self, ctx: &Context<'_>, id: ID) -> Result<Post> {
        let database = &ctx.data::<ContextData>()?.db;
        let model_post = database.post(Uuid::parse_str(id.as_str())?)?;

        Ok(model_post_to_graphql(database, model_post, 2))
    }

    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let database = &ctx.data::<ContextData>()?.db;
        let model_posts = database.posts()?;

        Ok(model_posts
            .iter()
            .map(|model_post| model_post_to_graphql(database, model_post.clone(), 2))
            .collect())
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_post(&self, ctx: &Context<'_>, post_input: NewPostInput) -> Result<Post> {
        let database = &ctx.data::<ContextData>()?.db;
        let res = database.add_post(post_input)?;

        Ok(model_post_to_graphql(database, res, 3))
    }

    async fn delete_post(&self, ctx: &Context<'_>, post_id: ID) -> Result<Post> {
        let database = &ctx.data::<ContextData>()?.db;
        let model_post = database.delete_post(Uuid::parse_str(post_id.as_str())?)?;

        Ok(model_post_to_graphql(database, model_post, 3))
    }

    async fn update_post(
        &self,
        ctx: &Context<'_>,
        post_id: ID,
        post_input: PostUpdate,
    ) -> Result<Post> {
        let database = &ctx.data::<ContextData>()?.db;
        let model_post = database.update_post(Uuid::parse_str(post_id.as_str())?, post_input)?;

        Ok(model_post_to_graphql(database, model_post, 3))
    }

    async fn add_user(&self, ctx: &Context<'_>, new_user_input: NewUserInput) -> Result<User> {
        let database = &ctx.data::<ContextData>()?.db;
        let res = database.add_user(new_user_input)?;

        Ok(model_user_to_graphql(database, res, 1))
    }

    async fn delete_user(&self, ctx: &Context<'_>, user_name: String) -> Result<User> {
        let database = &ctx.data::<ContextData>()?.db;
        let res = database.delete_user(user_name)?;

        Ok(model_user_to_graphql(database, res, 3))
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        user_name: String,
        user_input: UserUpdate,
    ) -> Result<User> {
        let database = &ctx.data::<ContextData>()?.db;
        let res = database.update_user(user_name, user_input)?;

        Ok(model_user_to_graphql(database, res, 3))
    }
}
