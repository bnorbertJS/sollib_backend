use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::{users as user, solutions as solution};

/*#[table_name="solutions"]*/
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Solution {
    id: i32,
    title: String,
    author: String,
    desc: String
}

#[table_name="user"]
#[derive(Serialize, Insertable, Queryable, Debug, Clone, PartialEq)]
pub struct User {
    id: i32,
    full_name: String,
    email: String,
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

    pub fn pw_by_email(email: String, conn: &PgConnection) -> Result<String, diesel::result::Error>{
        use schema::users::dsl::users;

        users
        .filter(user::email.eq(email))
        .select(user::pass)
        .first(conn)
    }
}