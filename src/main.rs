#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
mod database;
mod webserver;
mod schema;
mod models;
mod auth;
mod utils;
mod graphql;
use rocket::{routes, launch};
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use diesel_migrations::embed_migrations;


use std::{sync::Arc};

#[launch]
async fn rocket() -> _ {


    rocket::build()
    .attach(database::DbConn::fairing())
	.manage(graphql::create_schema())
	.manage(Arc::new(auth::get_jwt_config()))
    .mount("/", routes![webserver::graphiql, webserver::get_graphql_handler, webserver::post_graphql_handler])
}

