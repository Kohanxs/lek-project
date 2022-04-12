use crate::models::question::{Question};
use crate::models::user::{User, InsertableUser};
use crate::models::comment::{Comment, InsertableComment};
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::hashing;
use crate::utils::BackendError;
use rocket_sync_db_pools::{database};
use rocket_sync_db_pools::diesel::MysqlConnection;

#[database("local_db")]
pub struct DbConn(MysqlConnection);


pub fn new_user(conn: &MysqlConnection, user_name: &str, password: &str) -> Result<usize, BackendError> {
    use crate::schema::users;

    let (hash, salt) = hashing::get_hash_and_salt(password)?;

    let user_to_insert = InsertableUser {
        user_name: user_name.to_owned(),
        password_hash: hash.to_owned(),
        salt: salt.to_owned()
    };

    let database_result = diesel::insert_into(users::table).values(&user_to_insert).execute(conn)?;

    Ok(database_result)
}

pub fn new_comment(conn: &MysqlConnection, comment_to_insert: &InsertableComment) -> Result<usize, BackendError> {
    use crate::schema::comments;

    let database_result = diesel::insert_into(comments::table).values(comment_to_insert).execute(conn)?;

    Ok(database_result)
}

// pub fn get_user_by_username(conn: &MysqlConnection, _user_name: &str) -> Result<Option<User>, BackendError> {
//     use crate::schema::users::dsl::*;

//     let database_result = users.filter(user_name.eq(_user_name)).first(conn).optional()?;

//     Ok(database_result)
// }

pub fn get_all_questions(conn: &MysqlConnection) -> Result<Vec<Question>, BackendError> {
    use crate::schema::questions::dsl::*;

    let database_result = questions.load(conn)?;

    Ok(database_result)
}
