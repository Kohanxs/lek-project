
use rocket_contrib;

#[rocket_contrib::database("local_db")]
pub struct DbConn(diesel::MysqlConnection);
