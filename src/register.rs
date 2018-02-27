use db::Conn as DbConn;
use super::models::{User};
use rocket_contrib::Json;
use serde_json::Value;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest, Form};
use schema::{users};
use diesel;
use diesel::insert_into;
use diesel::prelude::*;

#[derive(FromForm, Insertable, Serialize, Deserialize, Debug)]
#[table_name="users"]
pub struct UserReg {
    full_name: String,
    email: String,
    pass: String
}
//TODO: validate form data
//TODO: specify proper return type 201 created or something
pub fn register_user(u: &UserReg, conn: &PgConnection) -> QueryResult<usize>{
   // println!("{:?}",user);
    use schema::users::dsl::users;
    insert_into(users)
        .values(u)
        .execute(conn)
}