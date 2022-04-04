use crate::models::question::{Question};
use crate::models::user::{User, InsertableUser};
use crate::diesel::RunQueryDsl;
use diesel::{MysqlConnection, QueryDsl};
use crate::diesel::ExpressionMethods;
use crate::hashing;
use rocket_contrib::json::Json;



#[rocket_contrib::database("local_db")]
pub struct DbConn(MysqlConnection);


pub fn new_user(conn: &MysqlConnection, user_name: &str, password: &str) -> Result<usize, diesel::result::Error> {
    use crate::schema::users;

    let salt = "How could you?";
    let hash = "fasdfalkhlghlg#$rse";

    let user_to_insert = InsertableUser {
        user_name: user_name.to_owned(),
        password_hash: hash.to_owned(),
        salt: salt.to_owned()
    };

    diesel::insert_into(users::table).values(&user_to_insert).execute(conn)

}

pub fn new_comment(conn: &MysqlConnection, content: &str, ) {
    
}

pub fn get_user_by_username(conn: &MysqlConnection, user_name: &str) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    users.filter(user_name.eq(user_name)).first(conn)

}