#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
mod database;
mod webserver;
mod schema;
mod models;
mod auth;
mod utils;
mod graphql;
use rocket::{routes, launch, fairing::{AdHoc, Fairing}, error, Rocket, Build};
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel_migrations;
use clap::Parser;
use std::{sync::Arc};

diesel_migrations::embed_migrations!("migrations/");

async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let conn = database::DbConn::get_one(&rocket).await.expect("database connection");
    conn.run(|conn| match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    migrations: bool
}

pub trait ConditionalAttach {
    fn attach_if(self, condition: bool, fairing: impl Fairing) -> Self;
}

impl ConditionalAttach for Rocket<Build> {
	#[inline]
    fn attach_if(self, condition: bool, fairing: impl Fairing) -> Self {
        if condition {
            self.attach(fairing)
        } else {
            self
        }
    }
}

#[launch]
async fn rocket() -> _ {

    let args = Cli::parse();

    rocket::build()
    .attach(database::DbConn::fairing())
    .attach_if(args.migrations, AdHoc::try_on_ignite("Database migration", run_db_migrations))
	.manage(graphql::create_schema())
	.manage(Arc::new(auth::get_jwt_config()))
    .mount("/", routes![webserver::graphiql, webserver::get_graphql_handler, webserver::post_graphql_handler])
}

