use db::Conn as DbConn;
use super::models::{User};
use rocket_contrib::Json;
use serde_json::Value;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Form, FromFormValue};
use schema::{users};
use diesel;
use diesel::insert_into;
use diesel::prelude::*;
// REFACTOR PRIO2
/*#[derive(Debug)]
struct CantMiss(&'static str);

impl<'v> FromFormValue<'v> for CantMiss{
    type Error = &'static str;
     fn from_form_value(s: &'v RawStr) -> Result<Self, Self::Error> {
         if s.len() == 0 {
             Err("Cant be blank!")
         }else{
             Ok(CantMiss(s.to_string()))
         }
     }
}*/

#[derive(FromForm, Insertable, Serialize, Deserialize, Debug)]
#[table_name="users"]
pub struct UserReg {
    pub full_name: String,
    pub email: String,
    pass: String
}
//TODO: validate form data with FromFormValue
//TODO: specify proper return type
pub fn register_user<'a>(u: &'a UserReg, conn: &PgConnection) -> Result<&'a UserReg, String>{
    use schema::users::dsl::users;

    if u.full_name == "" || u.email == "" || u.pass == "" {
        return Err("Fields cannot be blank".to_string());
    }

    //TODO hash pw PRIO 1
    
    let insert_res = insert_into(users)
                        .values(u)
                        .execute(conn);

    if let Err(e) = insert_res {
        Err(format!("Error: {}",e))
    }else{
        Ok(u)
    }
}