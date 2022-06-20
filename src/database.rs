use crate::models::question::{Question};
use crate::models::user::{User, InsertableUser, SafeUser};
use crate::models::comment::{Comment, InsertableComment};
use crate::models::category::{Category};
use crate::diesel::RunQueryDsl;

use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::OptionalExtension;
use crate::utils::BackendError;
use rocket_sync_db_pools::{database};
use rocket_sync_db_pools::diesel::PgConnection;

#[database("local_db")]
pub struct DbConn(PgConnection);


pub fn new_user(conn: &PgConnection, user_to_insert: &InsertableUser) -> Result<User, BackendError> {
    use crate::schema::users;

    let database_result = diesel::insert_into(users::table).values(user_to_insert).get_result::<User>(conn)?;

    Ok(database_result)
}

pub fn new_comment(conn: &PgConnection, comment_to_insert: &InsertableComment) -> Result<Comment, BackendError> {
    use crate::schema::comments;

    let database_result = diesel::insert_into(comments::table).values(comment_to_insert).get_result::<Comment>(conn)?;

    Ok(database_result)
}

pub fn get_user_by_username(conn: &PgConnection, user_name: &str) -> Result<User, BackendError> {
    use crate::schema::users::dsl::*;

    let database_result = users.filter(username.eq(user_name)).first(conn)?;

    Ok(database_result)
}

pub fn get_safeuser_by_id(conn: &PgConnection, user_id: i32) -> Result<SafeUser, BackendError> {
    use crate::schema::users::dsl::*;
    
    let database_result = users.select((id, username, nickname)).find(user_id).first(conn)?;

    Ok(database_result)

}

pub fn get_questions_by_category(conn: &PgConnection, category_id: i32) -> Result<Vec<Question>, BackendError> {
    use crate::schema::questions::dsl::*;

    let database_result = questions.filter(category_fk.eq(category_id)).load::<Question>(conn)?;

    Ok(database_result)

}

pub fn get_all_questions(conn: &PgConnection) -> Result<Vec<Question>, BackendError> {
    use crate::schema::questions::dsl::*;

    let database_result = questions.load::<Question>(conn)?;

    Ok(database_result)
}

pub fn get_comments_by_question(conn: &PgConnection, question_id: i32) -> Result<Vec<Comment>, BackendError> {
    use crate::schema::comments::dsl::*;

    let database_result = comments.filter(questions_fk.eq(question_id)).load::<Comment>(conn)?;

    Ok(database_result)

}

pub fn get_categories(conn: &PgConnection) -> Result<Vec<Category>, BackendError> {
    use crate::schema::category::dsl::*;

    let database_result = category.load::<Category>(conn)?;

    Ok(database_result)
}

pub fn like_comment(conn: &PgConnection, comment_id: i32) -> Result<Comment, BackendError> {
    use crate::schema::comments::dsl::*;

    let database_result = conn.build_transaction()
        .run(|| {
            diesel::update(comments.filter(id.eq(comment_id))).set(likes.eq(likes+1)).get_result::<Comment>(conn)
        })?;
    Ok(database_result)
}

