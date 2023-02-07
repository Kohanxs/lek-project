
use crate::auth;
use crate::database;
use crate::graphql::{GraphQLContext, Schema};
use crate::models::user::SafeUser;
use rocket::{response::content, State};
use std::sync::Arc;


#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    conn: database::DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    jwt_config: &State<Arc<auth::JWTConfig>>,
    authenticated_user: Option<SafeUser>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &GraphQLContext { db_connection: conn, user: authenticated_user, jwt_config: jwt_config.inner().clone()} ).await
}

#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    conn: database::DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    jwt_config: &State<Arc<auth::JWTConfig>>, //TODO how to get away from Arc?
    authenticated_user: Option<SafeUser>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &GraphQLContext { db_connection: conn, user: authenticated_user, jwt_config: jwt_config.inner().clone()} ).await
}








// fn load_from_db(conn: &MysqlConnection) -> Result<Json<Vec<models::question::Question>>, String>{
//     use schema::questions::dsl::*;

//     questions.load(conn).map_err(|err| -> String {
//         "Error occured".into()
//     }).map(Json)
    
// }

// #[put("/register", data = "<form>")]
// pub fn register(conn: database::DbConn, form: Json<NewUserForm>){
//     database::new_user(&*conn, &form.user_name, &form.password);
// }

// #[put("/login", data = "<form>")]
// pub fn login(conn: database::DbConn, form: Json<LoginForm>) {
//     let user = database::get_user_by_username(&*conn, &form.user_name).expect("Could not get the user").unwrap();

//     let is_verified = hashing::check_hash(&form.password, &user.password_hash).unwrap();
// }





