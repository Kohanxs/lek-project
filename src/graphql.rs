
use crate::database;
use crate::database::DbConn;
use jsonwebtoken::get_current_timestamp;
use juniper::{FieldResult, RootNode, graphql_object, EmptySubscription, graphql_value};
use crate::models::{user, comment, question, category};
use crate::auth;
use crate::utils;
use std::sync::Arc;

pub struct GraphQLContext{
    pub db_connection: DbConn,
    pub user: Option<user::SafeUser>,
    pub jwt_config: Arc<auth::JWTConfig>
}

impl<'a> juniper::Context for GraphQLContext {}


// The root GraphQL query
pub struct Query;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Mysql queries.
#[graphql_object(context = GraphQLContext)]
impl Query{


    pub async fn categories(context: &GraphQLContext) -> FieldResult<Vec<category::Category>> {
        let result = context.db_connection.run(|conn| database::get_categories(conn)).await?;
        Ok(result)
    }
    
    pub async fn questions(context: &GraphQLContext, id: Option<i32>, category_id: Option<i32>) -> FieldResult<Vec<question::Question>> {
        let result = match (id, category_id) {
            (None, None) => context.db_connection.run(|conn| database::get_all_questionsQL(conn)).await?,
            (Some(id), _) => context.db_connection.run(move|conn| database::get_question_by_idQL(conn, id)).await?,
            (None, Some(id)) => context.db_connection.run(move|conn| database::get_questions_by_categoryQL(conn, id)).await?
        };
        Ok(result)
    }

    pub async fn comments(context: &GraphQLContext, question_id: i32) -> FieldResult<Vec<comment::Comment>> {

        let result = context.db_connection.run(move |conn| database::get_comments_for_question(conn, question_id)).await?;
        Ok(result)
    }

}

// The root GraphQL mutation
pub struct Mutation;

#[graphql_object(context = GraphQLContext)]
impl Mutation {

    pub async fn login(
        context: &GraphQLContext, 
        input: user::LoginForm,
    ) -> FieldResult<user::Tokens> {
        
        let user = context.db_connection.run(move |conn| database::get_user_by_username(conn, &input.username)).await?;
        
        let result = auth::check_hash(&input.password, &user.password_hash)?;
        
        if result {
            let timestamp = get_current_timestamp();
            let tokens = (auth::generate_access_token(user.id, timestamp, &context.jwt_config)?, auth::generate_refresh_token(user.id, timestamp, &context.jwt_config)?);
            return Ok(user::Tokens {access_token: tokens.0, refresh_token: tokens.1})
        } else {
            return Err(juniper::FieldError::new("Password is incorrect!", graphql_value!({ "authentication error": "incorrect password"})))
        }
    }
    
    pub async fn signup(
        context: &GraphQLContext,
        input: user::NewUser,
    ) -> FieldResult<user::User> {
        let hash = auth::get_hash(&input.password)?;


        let user_to_insert = user::InsertableUser {
            username: input.username.to_owned(),
            password_hash: hash.to_owned(),
            nickname: input.nickname.to_owned(),
        };
        let result = context.db_connection.run(move |conn| database::new_user(conn, &user_to_insert)).await?;
        Ok(result)
    }

    pub async fn add_comment(
        context: &GraphQLContext,
        input: comment::NewComment,
    ) -> FieldResult<comment::Comment> {
        
        let user = context.user.as_ref().ok_or(utils::BackendError::NotAuthorized)?; //TODO boilerplate

        let comment_to_insert = comment::InsertableComment {
                content: input.content.to_owned(),
                suggested_answer: input.suggested_answer,
                users_fk: user.id,
                questions_fk: input.question
            };
        let result = context.db_connection.run(move |conn| database::new_comment(conn, &comment_to_insert)).await?;
        Ok(result)
    }


}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}