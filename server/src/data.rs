use crate::{
    db::PostgresPool,
    graphql::{NewPostInput, NewUserInput, PostUpdate as GPostUpdate, UserUpdate as GUserUpdate},
    models::{Post, PostUpdate, User, UserUpdate},
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

fn post_input_to_update(input: GPostUpdate) -> PostUpdate {
    let GPostUpdate {
        title,
        date,
        body,
        author,
        published,
    } = input;

    PostUpdate {
        title,
        date: {
            match date {
                Some(date_str) => Some(
                    NaiveDate::parse_from_str(date_str.as_str(), "%d-%m-%Y")
                        .expect("Wrong date value: (Expected: DD-MM-YYYY)"),
                ),
                None => None,
            }
        },
        body,
        author,
        published,
    }
}

fn user_input_to_update(input: GUserUpdate) -> UserUpdate {
    let GUserUpdate {
        user_name,
        full_name,
        email,
        descr,
        password,
    } = input;

    UserUpdate {
        user_name,
        full_name,
        email,
        descr,
        password_hash: {
            match password {
                Some(pass) => {
                    let mut passwordhash = Sha3::sha3_256();
                    passwordhash.input_str(&pass.to_string().as_str());
                    Some(passwordhash.result_str())
                }
                None => None,
            }
        },
        article_ids: None,
        draft_ids: None,
    }
}

fn user_article_remove(
    user: &User,
    is_published: bool,
    id: Uuid,
    pool: &PostgresPool,
) -> FieldResult<()> {
    use super::schema::users;

    if is_published {
        let mut id_vec = user.article_ids.clone();
        if let Some(post_index) = id_vec.iter().position(|post_id| post_id == &id) {
            id_vec.remove(post_index);
        }
        diesel::update(user)
            .set(users::article_ids.eq(id_vec))
            .get_result::<User>(&pool.get().unwrap())?;
    } else {
        let mut id_vec = user.draft_ids.clone();
        if let Some(post_index) = id_vec.iter().position(|post_id| post_id == &id) {
            id_vec.remove(post_index);
        }
        diesel::update(user)
            .set(users::draft_ids.eq(id_vec))
            .get_result::<User>(&pool.get().unwrap())?;
    }

    Ok(())
}
fn user_article_push(
    user: &User,
    is_published: bool,
    post_id: Uuid,
    pool: &PostgresPool,
) -> FieldResult<()> {
    use super::schema::users;

    if is_published {
        let mut id_vec = user.article_ids.clone();
        id_vec.push(post_id);
        diesel::update(user)
            .set(users::article_ids.eq(id_vec))
            .get_result::<User>(&pool.get().unwrap())?;
    } else {
        let mut id_vec = user.draft_ids.clone();
        id_vec.push(post_id);
        diesel::update(user)
            .set(users::draft_ids.eq(id_vec))
            .get_result::<User>(&pool.get().unwrap())?;
    }

    Ok(())
}

// Queries
impl PGQuery {
    pub fn validate_user(&self, username: String, password: String) -> FieldResult<bool> {
        use super::schema::users::dsl::*;

        let mut passwordhash = Sha3::sha3_256();
        passwordhash.input_str(&password.to_string().as_str());
        let passwordhash = passwordhash.result_str();

        let res = users
            .filter(user_name.eq(username))
            .filter(password_hash.eq(passwordhash))
            .first::<User>(&self.pool.get().unwrap());

        match res {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn user(&self, username: String) -> FieldResult<User> {
        use super::schema::users::dsl::*;

        let res = users
            .filter(user_name.eq(username))
            .first(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn users(&self) -> FieldResult<Vec<User>> {
        use super::schema::users::dsl::*;

        let res = users.load::<User>(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn post(&self, post_id: Uuid) -> FieldResult<Post> {
        use super::schema::posts::dsl::*;

        let res = posts.find(post_id).first(&self.pool.get().unwrap());

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
            user_name: user_name.clone(),
            full_name,
            descr,
            email,
            password_hash,
            article_ids: vec![],
            draft_ids: vec![],
        };

        let is_existing = users::table
            .filter(users::user_name.eq(user_name))
            .get_result::<User>(&self.pool.get().unwrap());

        if is_existing.is_ok() {
            return FieldResult::Err(FieldError::from("Username already exists"));
        }

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
            author: author.clone(),
            published,
        };

        let post_id = new_post.id.clone();

        if let Ok(user) = &self.user(author) {
            user_article_push(user, published, post_id, &self.pool)?;
        }

        let res = diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(&self.pool.get().unwrap());

        graphql_translate(res)
    }

    pub fn delete_user(&self, user_name: String) -> FieldResult<User> {
        use super::schema::users;

        Ok(
            diesel::delete(users::table.filter(users::user_name.eq(user_name)))
                .get_result::<User>(&self.pool.get().unwrap())?,
        )
    }

    pub fn delete_post(&self, id: Uuid) -> FieldResult<Post> {
        use super::schema::posts;

        let post =
            diesel::delete(posts::table.find(id)).get_result::<Post>(&self.pool.get().unwrap())?;

        let is_published = post.published;

        if let Ok(author) = &self.user(post.author.clone()) {
            user_article_remove(author, is_published, id, &self.pool)?;
        }

        Ok(post)
    }

    pub fn update_user(&self, user_name: String, input: GUserUpdate) -> FieldResult<User> {
        use super::schema::users;

        Ok(
            diesel::update(users::table.filter(users::user_name.eq(user_name)))
                .set(&user_input_to_update(input))
                .get_result::<User>(&self.pool.get().unwrap())?,
        )
    }

    pub fn update_post(&self, id: Uuid, input: GPostUpdate) -> FieldResult<Post> {
        use super::schema::posts;

        let old_post = self.post(id)?;
        let was_published = old_post.published;
        let are_authors_same = {
            match input.author {
                Some(ref author_name) => &old_post.author == author_name,
                None => false,
            }
        };

        let post = diesel::update(posts::table.find(id))
            .set(&post_input_to_update(input))
            .get_result::<Post>(&self.pool.get().unwrap())?;

        let is_published = post.published;

        if let Ok(prev_author) = self.user(old_post.author) {
            if was_published != is_published || !are_authors_same {
                user_article_remove(&prev_author, was_published, id, &self.pool)?;
            }
        }

        if let Ok(author) = self.user(post.author.clone()) {
            if was_published != is_published || !are_authors_same {
                user_article_push(&author, is_published, id, &self.pool)?;
            }
        }

        Ok(post)
    }
}
