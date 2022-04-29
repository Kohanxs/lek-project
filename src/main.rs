#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
mod database;
mod webserver;
mod schema;
mod models;
mod hashing;
mod utils;
mod graphql;
use rocket::{routes, launch};
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket_session_store::{
	memory::MemoryStore,
	SessionStore,
	SessionResult,
	Session,
	CookieConfig,
};

use std::time::Duration;

#[launch]
async fn rocket() -> _ {
    let memory_store: MemoryStore::<String> = MemoryStore::default();
	let store: SessionStore<String> = SessionStore {
		store: Box::new(memory_store),
		name: "token".into(),
		duration: Duration::from_secs(3600 * 24 * 3),
		// The cookie config is used to set the cookie's path and other options.
		cookie: CookieConfig::default(),
	};
    rocket::build()
    .attach(database::DbConn::fairing())
	.manage(graphql::create_schema())
    .mount("/", routes![webserver::graphiql, webserver::get_graphql_handler, webserver::post_graphql_handler])
}

