
use crate::database;
use crate::database::DbConn;
use crate::models::comment::Comment;
use diesel::result::Error::NotFound;
use jsonwebtoken::get_current_timestamp;
use juniper::{FieldResult, RootNode, graphql_object, EmptySubscription, graphql_value, FieldError};
use crate::models::{user, comment, question, category};
use crate::auth::{self, validate_token, TokenType};
use crate::utils::{self, BackendError};
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
            (None, None) => context.db_connection.run(|conn| database::get_all_questions_ql(conn)).await?,
            (Some(id), _) => context.db_connection.run(move|conn| database::get_question_by_id_ql(conn, id)).await?,
            (None, Some(id)) => context.db_connection.run(move|conn| database::get_questions_by_category_ql(conn, id)).await?
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
        // TODO timing problem, wrong username -> instant reaction, wrong password -> takes some time
        let user = context.db_connection.run(move |conn| database::get_user_by_username(conn, &input.username)).await.map_err(|err| {
            match err {
                BackendError::DatabaseError(NotFound) => BackendError::WrongCredentials,
                do_not_change => do_not_change
            }
        })?;
        
        let password_matches = auth::check_hash(&input.password, &user.password_hash)?;
        
        let result = if password_matches {
            let timestamp = get_current_timestamp();
            let tokens = (auth::generate_access_token(user.id, timestamp, &context.jwt_config, user.is_admin)?, auth::generate_refresh_token(user.id, timestamp, &context.jwt_config, user.is_admin)?);
            Ok(user::Tokens {access_token: tokens.0, refresh_token: tokens.1})
        } else {
            Err(FieldError::from(utils::BackendError::WrongCredentials))
        };

        result
    }

    pub async fn refresh_tokens(
        context: &GraphQLContext,
        refresh_token: String,
    ) -> FieldResult<user::Tokens> {
        
        let validation_claims = validate_token(&refresh_token, &context.jwt_config, TokenType::Refresh)?;
        let timestamp = get_current_timestamp();
        let access_token = auth::generate_access_token(validation_claims.sub.parse::<i32>()?, timestamp, &context.jwt_config, validation_claims.admin)?;

        Ok(user::Tokens { access_token: access_token, refresh_token: "".to_owned() })
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

    pub async fn delete_comment(
        context: &GraphQLContext,
        input: comment::DeleteComment,
    ) -> FieldResult<bool> {
        
        let user = context.user.as_ref().ok_or(utils::BackendError::NotAuthorized)?; //TODO boilerplate

        let comment = context.db_connection.run(move |conn| database::get_comment_by_id(conn, input.id)).await?;

        let result = if comment.users_fk == user.id {
            context.db_connection.run(move |conn| database::delete_comment(conn, input.id)).await?;
            Ok(true)
        } else {
            /* Trying to delete someone else's comment  */
            Err(FieldError::from(utils::BackendError::NotAuthorized))
        };

        result
    }

    pub async fn modify_comment(
        context: &GraphQLContext,
        input: comment::ModifyCommentForm,
    ) -> FieldResult<Comment> {
        
        let user = context.user.as_ref().ok_or(utils::BackendError::NotAuthorized)?; //TODO boilerplate

        let comment = context.db_connection.run(move |conn| database::get_comment_by_id(conn, input.id)).await?;

        let result = if comment.users_fk == user.id {
            let modification_result = context.db_connection.run(move |conn| database::modify_comment(conn, &input.into())).await?;
            Ok(modification_result)
        } else {
            /* Trying to modify someone else's comment  */
            Err(FieldError::from(utils::BackendError::NotAuthorized))
        };
        result
    }

    pub async fn delete_user<'db>(
        context: &'db GraphQLContext
    ) -> FieldResult<bool> {
        {
            let user = context.user.as_ref().ok_or(utils::BackendError::NotAuthorized)?; //TODO boilerplate
            let user_id = user.id;
            context.db_connection.run(move |conn| database::delete_user(conn, user_id)).await?;
        }
        
        Ok(true)
    }

    pub async fn modify_question(
        context: &GraphQLContext,
        input: comment::ModifyCommentForm,
    ) -> FieldResult<Comment> {
        
        let user = context.user.as_ref().ok_or(utils::BackendError::NotAuthorized)?; //TODO boilerplate

        let comment = context.db_connection.run(move |conn| database::get_comment_by_id(conn, input.id)).await?;

        let result = if comment.users_fk == user.id {
            let modification_result = context.db_connection.run(move |conn| database::modify_comment(conn, &input.into())).await?;
            Ok(modification_result)
        } else {
            /* Trying to modify someone else's comment  */
            Err(FieldError::from(utils::BackendError::NotAuthorized))
        };
        result
    }


}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}