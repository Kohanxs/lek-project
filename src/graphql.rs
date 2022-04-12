
use crate::database;
use crate::database::DbConn;
use juniper::{FieldResult, RootNode, graphql_object, EmptySubscription};
use crate::models::user;
use crate::models::question;

pub struct GraphQLContext {
    pub db_connection: DbConn
}

impl juniper::Context for GraphQLContext {}


// The root GraphQL query
pub struct Query;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Mysql queries.
#[graphql_object(context = GraphQLContext)]
impl Query {
    
    pub async fn all_quetions(context: &GraphQLContext) -> FieldResult<Vec<question::Question>> {
        // TODO: pass the GraphQLContext into the querying functions rather
        // than a PgConnection (for brevity's sake)
        let connection = context.db_connection.run(|conn| database::get_all_questions(conn)).await?;
        Ok(connection)
    }
}

// The root GraphQL mutation
pub struct Mutation;

#[graphql_object(context = GraphQLContext)]
impl Mutation {
    
    pub async fn register(
        context: &GraphQLContext,
        input: user::NewUserInput,
    ) -> FieldResult<String> {
        let result = context.db_connection.run(move |conn| database::new_user(conn, &input.user_name, &input.password)).await?;
        Ok(String::from("Successful registration"))
    }

}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}