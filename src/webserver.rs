
use diesel::{RunQueryDsl, MysqlConnection};
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use crate::database;
use crate::models;
use crate::models::user::NewUserForm;
use rocket::{get, put, response::content};
use crate::schema;

fn load_from_db(conn: &MysqlConnection) -> Result<Json<Vec<models::question::Question>>, String>{
    use schema::questions::dsl::*;

    questions.load(conn).map_err(|err| -> String {
        "Error occured".into()
    }).map(Json)
    
}

#[put("/register", data = "<form>")]
pub fn register(conn: database::DbConn, form: Json<NewUserForm>){
    database::new_user(&*conn, &form.user_name, &form.password);
}


