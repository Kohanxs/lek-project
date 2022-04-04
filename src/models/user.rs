use crate::schema::users;

#[derive(Debug, diesel::Queryable, diesel::Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub password_hash: String,
    pub user_name: String,
    pub salt: String
}

#[derive(Deserialize, diesel::Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub password_hash: String,
    pub user_name: String,
    pub salt: String
}

#[derive(Deserialize)]
pub struct NewUserForm {
    pub user_name: String,
    pub password: String
}