#![feature(plugin, decl_macro, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derives;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate bcrypt;
extern crate frank_jwt;

use dotenv::dotenv;
use std::env;

mod auth;
mod models;
mod db;
mod routes;
mod schema{
    table! {
        solutions (id) {
            id -> Int4,
            title -> Varchar,
            author -> Varchar,
            descr -> Text,
        }
    }

    table! {
        users (id) {
            id -> Int4,
            full_name -> Varchar,
            email -> Text,
            pass -> Varchar,
        }
    }

    allow_tables_to_appear_in_same_query!(
        solutions,
        users,
    );

}


fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("cannot find env variable DATABASE_URL");

    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![routes::welcome])
        .mount("/api/v1/", routes![
            routes::get_solutions,
            routes::get_solution_by_id,
            routes::login
        ])
        .launch();
}
