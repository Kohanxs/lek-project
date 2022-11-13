use crate::models::comment::Comment;
use crate::models::question::{QuestionDB, Question};
use crate::models::user::{User, InsertableUser, SafeUser};
use crate::models::comment::{CommentDB, InsertableComment};
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
    use crate::schema::users::dsl::*;

    let new_comment = diesel::insert_into(comments::table).values(comment_to_insert).get_result::<CommentDB>(conn)?;
    let user_lookup = users.find(new_comment.users_fk).first(conn)?;
    Ok(Comment::from((new_comment, user_lookup)))
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

pub fn get_questions_by_category(conn: &PgConnection, category_id: i32) -> Result<Vec<QuestionDB>, BackendError> {
    use crate::schema::questions::dsl::*;

    let database_result = questions.filter(category_fk.eq(category_id)).load::<QuestionDB>(conn)?;

    Ok(database_result)

}

pub fn get_questions_by_categoryQL(conn: &PgConnection, category_id: i32) -> Result<Vec<Question>, BackendError> {
    use crate::schema::questions::dsl::*;
    use crate::schema::category::dsl::*;
    
    let ql_result = questions.left_join(category).filter(category_fk.eq(category_id)).load::<(QuestionDB, Option<Category>)>(conn)?.drain(..).map(|x| Question::from(x)).collect();
    Ok(ql_result)

}


pub fn get_questions_by_id(conn: &PgConnection, question_id: i32) -> Result<Vec<QuestionDB>, BackendError> {
    use crate::schema::questions::dsl::*;

    let database_result = questions.filter(id.eq(question_id)).load::<QuestionDB>(conn)?;

    Ok(database_result)

}

pub fn get_question_by_idQL(conn: &PgConnection, question_id: i32) -> Result<Vec<Question>, BackendError> {
    use crate::schema::questions::dsl::questions;
    use crate::schema::questions::dsl::id;
    use crate::schema::category::dsl::*;

    let ql_result = questions.left_join(category).filter(id.eq(question_id)).load::<(QuestionDB, Option<Category>)>(conn)?.drain(..).map(|x| Question::from(x)).collect();
    Ok(ql_result)

}


pub fn get_all_questions(conn: &PgConnection) -> Result<Vec<QuestionDB>, BackendError> {
    use crate::schema::questions::dsl::*;

    let database_result = questions.load::<QuestionDB>(conn)?;

    Ok(database_result)
}

pub fn get_all_questionsQL(conn: &PgConnection) -> Result<Vec<Question>, BackendError> {
    use crate::schema::questions::dsl::*;
    use crate::schema::category::dsl::*;

    let ql_result = questions.left_join(category).load::<(QuestionDB, Option<Category>)>(conn)?.drain(..).map(|x| Question::from(x)).collect();

    Ok(ql_result)
}

pub fn get_categories(conn: &PgConnection) -> Result<Vec<Category>, BackendError> {
    use crate::schema::category::dsl::*;

    let database_result = category.load::<Category>(conn)?;

    Ok(database_result)
}

pub fn like_comment(conn: &PgConnection, comment_id: i32) -> Result<CommentDB, BackendError> {
    use crate::schema::comments::dsl::*;

    let database_result = conn.build_transaction()
        .run(|| {
            diesel::update(comments.filter(id.eq(comment_id))).set(likes.eq(likes+1)).get_result::<CommentDB>(conn)
        })?;
    Ok(database_result)
}

pub fn get_comments_for_question(conn: &PgConnection, question_id: i32) -> Result<Vec<Comment>, BackendError> {
    use crate::schema::comments::dsl::*;
    use crate::schema::users::dsl::*;

    let commentsql: Vec<Comment> = comments.inner_join(users).filter(questions_fk.eq(question_id)).load::<(CommentDB, User)>(conn)?.drain(..).map(|x| Comment::from(x) ).collect::<Vec<_>>();

    Ok(commentsql)
}