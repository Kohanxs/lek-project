
use diesel::RunQueryDsl;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use crate::database;
use crate::models;
use rocket::{get, response::content};
use crate::schema;

fn load_from_db(conn: &diesel::MysqlConnection) -> Result<Json<Vec<models::Question>>, String>{
    use schema::questions::dsl::*;

    questions.load(conn).map_err(|err| -> String {
        "Error occured".into()
    }).map(Json)
    
}

#[get("/")]
pub fn graphiql(conn: database::DbConn) -> Result<Json<Vec<models::Question>>, String>{
    load_from_db(&*conn)
}


