use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("./src/schema.graphql");

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_hello_world(
        &self,
        executor: &juniper::Executor<'_, Context>,
        name: String,
    ) -> juniper::FieldResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}

fn main() {
    println!("Hello, world!");
}
