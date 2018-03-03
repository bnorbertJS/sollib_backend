use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::dsl::Select;

use schema::{users as user, solutions as solution};
//Solution schema
/*#[table_name="solutions"]*/
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Solution {
    id: i32,
    title: String,
    author: String,
    desc: String
}
//User schema
#[derive(Serialize, Queryable, Debug, Clone, PartialEq)]
pub struct User {
    id: i32,
    pub full_name: String,
    pub email: String,
    pass: String
}

impl Solution {
    pub fn all(conn: &PgConnection) -> Vec<Solution>{
        use schema::solutions::dsl::solutions;

        solutions
            .order(solution::id.desc())
            .load::<Solution>(conn)
            .expect("Error querying solutions!")
    }

    pub fn by_id(id: i32, conn: &PgConnection) -> Vec<Solution>{
        use schema::solutions::dsl::solutions;

        solutions
            .find(id)
            .load::<Solution>(conn)
            .expect("Error querying solution by id")
    }
}

impl User{
    pub fn all(conn: &PgConnection) -> Vec<User>{
        use schema::users::dsl::users;

        users
            .order(user::id.desc())
            .load::<User>(conn)
            .expect("Error querying users!")
    }

    pub fn me(email: String, conn: &PgConnection) -> Option<User>{
        use schema::users::dsl::users;
        //Getting currently logged in user by email
    //   let u = user::table.select(ME_COLUMNS).filter(user::email.eq(email)).get_result::<MeColumns>(conn);
    //    let u = users.find(id).first::<User>(conn);
        let u = users
                .filter(user::email.eq(email))
                .first::<User>(conn);
        //creating return user object
        if let Ok(curr_user) = u {
            return Some(curr_user);
        }

        return None;
    }

    pub fn pw_by_email(email: &String, conn: &PgConnection) -> Result<String, diesel::result::Error>{
        use schema::users::dsl::users;

        users
        .filter(user::email.eq(email))
        .select(user::pass)
        .first(conn)
    }
}