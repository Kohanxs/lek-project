#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
mod database;
mod webserver;
mod models;
mod schema;
use rocket::{routes};
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


fn main() {
    rocket::ignite()
    .attach(database::DbConn::fairing())
    .mount("/", routes![webserver::graphiql]).launch();
}

