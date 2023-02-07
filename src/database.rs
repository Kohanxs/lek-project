use crate::models::comment::{Comment, ModifyComment};
use crate::models::question::{QuestionDB, Question};
use crate::models::user::{User, InsertableUser, SafeUser};
use crate::models::comment::{CommentDB, InsertableComment};
use crate::models::category::{Category};
use crate::models::question_category::{QuestionCategory};
use crate::diesel::RunQueryDsl;
use diesel::OptionalExtension;
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::dsl::{count_star};
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

pub fn delete_user(conn: &PgConnection, user_id: i32) -> Result<() ,BackendError>{
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id))).execute(conn)?;

    return Ok(());
}

pub fn new_comment(conn: &PgConnection, comment_to_insert: &InsertableComment) -> Result<Comment, BackendError> {
    use crate::schema::comments;
    use crate::schema::users::dsl::*;

    let new_comment = diesel::insert_into(comments::table).values(comment_to_insert).get_result::<CommentDB>(conn)?;
    let user_lookup = users.find(new_comment.users_fk).first(conn)?;
    Ok(Comment::from((new_comment, user_lookup, None)))
}

pub fn delete_comment(conn: &PgConnection, comment_id: i32) -> Result<() ,BackendError>{
    use crate::schema::comments::dsl::*;

    diesel::delete(comments.filter(id.eq(comment_id))).execute(conn)?;

    return Ok(());
}

pub fn modify_comment(conn: &PgConnection, modification: &ModifyComment) -> Result<Comment, BackendError> {
    use crate::schema::comments::dsl::*;
    use crate::schema::comment_user::dsl::*;

    let result: CommentDB = diesel::update(comments.filter(id.eq(modification.id))).set(modification).get_result::<CommentDB>(conn)?;
    let user = get_user_by_id(conn, result.users_fk)?;
    let result_count = comment_user.filter(comment_fk.eq(result.id)).count().get_result::<i64>(conn).map(|x| x as i32);
    let likes = result_count.optional()?;
    Ok(Comment::from((result, user, likes)))

}

pub fn get_user_by_username(conn: &PgConnection, user_name: &str) -> Result<User, BackendError> {
    use crate::schema::users::dsl::*;

    let database_result = users.filter(username.eq(user_name)).first(conn)?;

    Ok(database_result)
}

pub fn get_user_by_id(conn: &PgConnection, user_id: i32) -> Result<User, BackendError> {
    use crate::schema::users::dsl::*;

    let database_result = users.filter(id.eq(user_id)).first(conn)?;

    Ok(database_result)
}

pub fn get_safeuser_by_id(conn: &PgConnection, user_id: i32) -> Result<SafeUser, BackendError> {
    use crate::schema::users::dsl::*;
    
    let database_result = users.select((id, username, nickname, is_admin)).find(user_id).first(conn)?;

    Ok(database_result)

}

pub fn get_likes_by_comment(conn: &PgConnection, comment_id: i32) -> Result<i32, BackendError> {
    use crate::schema::comment_user::dsl::*;

    let database_result: i64 = comment_user.filter(comment_fk.eq(comment_id)).count().get_result(conn)?;

    Ok(database_result as i32)
}

pub fn get_questions_by_category_ql(conn: &PgConnection, category_id: i32) -> Result<Vec<Question>, BackendError> {
    use crate::schema::questions::dsl::*;
    use crate::schema;
    use crate::schema::question_category::dsl::*;
    use crate::schema::category::dsl::*;

    let filtered_questions: Vec<QuestionDB> = question_category.left_join(questions).filter(category_fk.eq(category_id)).select(schema::questions::dsl::id, content, answer_1, answer_2, answer_3, answer_4, answer_5, correct_answer).load::<QuestionDB>(conn)?;

    let grouped_categories: Vec<Category> = QuestionCategory::belonging_to(&filtered_questions).select(category).left_join(category).grouped_by(&filtered_questions).load::<Category>(conn)?;

    let ql_result = filtered_questions.into_iter().zip(grouped_categories).map(|x| Question::from(x)).collect();
    
    Ok(ql_result)

}


pub fn get_questions_by_id(conn: &PgConnection, question_id: i32) -> Result<Vec<QuestionDB>, BackendError> {
    use crate::schema::questions::dsl::*;

    let database_result = questions.filter(id.eq(question_id)).load::<QuestionDB>(conn)?;

    Ok(database_result)

}

pub fn get_question_by_id_ql(conn: &PgConnection, question_id: i32) -> Result<Vec<Question>, BackendError> {
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

pub fn get_all_questions_ql(conn: &PgConnection) -> Result<Vec<Question>, BackendError> {
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

// pub fn like_comment(conn: &PgConnection, comment_id: i32) -> Result<CommentDB, BackendError> {
//     use crate::schema::comments::dsl::*;

//     let database_result = conn.build_transaction()
//         .run(|| {
//             diesel::update(comments.filter(id.eq(comment_id))).set(likes.eq(likes+1)).get_result::<CommentDB>(conn)
//         })?;
//     Ok(database_result)
// }

pub fn get_comments_for_question(conn: &PgConnection, question_id: i32) -> Result<Vec<Comment>, BackendError> {
    use crate::schema::comments::dsl::*;
    use crate::schema::users::dsl::*;
    use crate::schema::comment_user::dsl::*;

    let likes_query = comment_user.select(comment_fk, count_star()).group_by(comment_fk);

    let comments_for_question = comments.inner_join(users).left_join(likes_query).filter(questions_fk.eq(question_id)).load::<CommentDB, User, Option<i32>>(conn)?.drain(..).map(|x| Comment::from(x)).collect::<Vec<_>>();

    Ok(comments_for_question)
}

pub fn get_comment_by_id(conn: &PgConnection, comment_id: i32) -> Result<CommentDB, BackendError> {
    use crate::schema::comments::dsl::*;

    let database_result = comments.filter(id.eq(comment_id)).first(conn)?;

    Ok(database_result)
}