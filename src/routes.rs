use db::Conn as DbConn;
use rocket_contrib::Json;
use super::models::{Solution, User};
use serde_json::Value;
use rocket::response::{Redirect, status, Failure};
use rocket::response::status::{Created, BadRequest, Custom};
use bcrypt::{DEFAULT_COST, hash, verify};
use frank_jwt::{Algorithm, encode, decode};
use std::env;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest, Form};
use auth::{ApiKey, is_valid, UserLogin, generate_token, validate_pw};
use register::{UserReg, register_user};

#[get("/")]
fn welcome() -> &'static str {
    "Welcome to Solution Library !"
}

#[get("/solutions")]
fn get_solutions(conn: DbConn) -> Json<Value> {
    let solutions = Solution::all(&conn);
    Json(json!({
        "status": 200,
        "result": solutions
    }))
}

#[get("/solution/<id>")]
fn get_solution_by_id(id: i32, conn: DbConn) -> Json<Value> {
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

#[get("/me")]
fn get_me(key: ApiKey, conn: DbConn) -> Result<Json<Value>,Failure>{
    //getting current user from the token CLaims (email)
    let s_key = env::var("SECRET_KEY").expect("cannot find env variable SECRET_KEY");
    let (header, payload) = decode(&key.0.to_string(), &s_key, Algorithm::HS256).unwrap();
        
    let email = payload["email"].as_str().unwrap();
    // then returning user data from database by email
    let me = User::me(email.to_string(), &conn);

    // if there is a user -> http 200 with json value
    match me {
        Some(user) => {
            return Ok(Json(json!({
                    "result": {
                        "full_name": user.full_name,
                        "email": user.email
                    }
                })));
        },
        None => return Err(Failure(Status::Unauthorized))
    }
}

#[post("/login", data= "<user_login>")]
fn login(user_login: Form<UserLogin>, conn: DbConn) -> Result<String,Failure>{
    let login_creds = user_login.get();
    let client_pw = login_creds.pass.to_string();
    let client_email = login_creds.email.to_string();
    //getting password for database by email
    let db_pw = User::pw_by_email(&client_email, &conn);

    //if user doesnt exists
    if let Err(e) = db_pw {
        return Err(Failure(Status::Unauthorized));
    }
    // if password is wrong
    if validate_pw(client_pw, db_pw.unwrap()){
        if let Ok(token) = generate_token(client_email){
            return Ok(token);
        }
    }
    return Err(Failure(Status::Unauthorized));
}

#[post("/register", data="<user_reg>")]
fn register<'r>(user_reg: Form<UserReg>, conn: DbConn) ->
    Result<Created<Json<Value>>, BadRequest<String>> {
        
    let user_fields = user_reg.get();
    let user = register_user(user_fields, &conn);
    let resp;

    if let Err(e) = user {
        resp = Err(BadRequest(Some(format!("Error happened: {}",e))));

    }else{
        
        resp = Ok(Created("/".to_string(),Some(Json(json!({
            "status": 201,
            "result": {
                "full_name": user_fields.full_name,
                "email": user_fields.email
            } 
        })))));
    }

    resp
}