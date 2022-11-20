use juniper::GraphQLObject;
use crate::{schema::users};

#[derive(GraphQLObject, Debug, diesel::Queryable, diesel::Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub nickname: String
}

#[derive(Deserialize, diesel::Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub password_hash: String,
    pub username: String,
    pub nickname: String
}

#[derive(diesel::Queryable, Deserialize, juniper::GraphQLInputObject)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub nickname: String
}

#[derive(Deserialize, juniper::GraphQLInputObject)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

#[derive(GraphQLObject, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(diesel::Queryable)]
pub struct SafeUser {
    pub id: i32,
    pub username: String,
    pub nickname: String
}