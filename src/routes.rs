use db::Conn as DbConn;
use rocket_contrib::Json;
use super::models::{Solution, User};
use serde_json::Value;
use rocket::response::status;
use bcrypt::{DEFAULT_COST, hash, verify};
use frank_jwt::{Algorithm, encode, decode};
use std::env;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest, Form};
use auth::{ApiKey, is_valid, UserLogin, generate_token, validate_pw};

#[get("/")]
fn welcome() -> &'static str {
    "Welcome to Solution Library !"
}

#[get("/solutions")]
fn get_solutions(conn: DbConn) -> Json<Value> {
    let solutions = Solution::all(&conn);
    let users = User::all(&conn);
    Json(json!({
        "status": 200,
        "result": solutions,
        "users": users
    }))
}

#[get("/solution/<id>")]
fn get_solution_by_id(key: ApiKey, id: i32, conn: DbConn) -> Json<Value> {
    let solution = Solution::by_id(id,&conn);
    let result_json;

    if solution.is_empty(){
        result_json = Json(json!({
            "status": 404,
            "result": "Cannot find solution with this id"
        }))
    }else{
        result_json = Json(json!({
            "status": 200,
            "result": solution
        }))
    }

    result_json
}

#[post("/login", data= "<user_login>")]
fn login(user_login: Form<UserLogin>, conn: DbConn) -> String{
    let login_creds = user_login.get();
    let client_pw = login_creds.pass.to_string();
    let client_email = login_creds.email.to_string();
    //getting password for database by email
    let db_pw = User::pw_by_email(client_email, &conn);
    
    // match pw first checks if there is a password for the provided email address
    // if its OK() => then the email exists in the database with a password,
    // so then we validate if the pw is the same as the pass in the database
    //REFACTORABLE
    match db_pw {
        Ok(res) => validate_pw(client_pw, res),
        Err(_) => "Invalid Username/Password".to_string()
    }
}