use rocket_contrib::Json;
use serde_json::Value;
use bcrypt::{DEFAULT_COST, hash, verify};
use frank_jwt::{Algorithm, encode, decode};
use std::env;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest, Form};
// api key expiration refactor needed !!! + adding email dynamically
pub struct ApiKey(String);

pub fn is_valid(key: &str) -> bool {
    let s_key = env::var("SECRET_KEY").expect("cannot find env variable SECRET_KEY");

    match decode(&key.to_string(), &s_key, Algorithm::HS256) {
        Ok(token) => true,
        Err(err) => false
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("x-sollib-key").collect();

        if keys.len() != 1 || !is_valid(keys[0]) {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        //processing vector
        
       /* if !is_valid(keys[0]) {
            return Outcome::Forward(());
        }*/
        let key = keys[0];
        return Outcome::Success(ApiKey(key.to_string()));
    }
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub pass: String
}

pub fn generate_token() -> String{
    let mut payload = json!({
        "iss": "solution-library.io",
        "email" : "bnorbertjs@gmail.com",
        "admin" : false
    });
    let secret = env::var("SECRET_KEY").expect("cannot find env variable SECRET_KEY");

    let jwt = encode(json!({}), &secret.to_string(), &payload, Algorithm::HS256);

    match jwt{
        Ok(token) => token.to_string(),
        Err(_) => "Error genrating token for ya.".to_string()
    }
}

pub fn validate_pw(client_pw: String, db_pw: String) -> String{
    println!("{:?}",client_pw);
    println!("{:?}",db_pw);
    print!("{:?}",verify(&client_pw, &db_pw).unwrap());
    match verify(&client_pw, &db_pw) {
        Ok(valid) => if valid { generate_token() } else { "Invalid Username/Password".to_string() },
        Err(_) => "Invalid Username/Password".to_string()
    }
}