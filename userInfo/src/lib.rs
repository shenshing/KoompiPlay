// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use chrono::NaiveDateTime;
use self::models::{User, NewUser};
pub fn insert_user(conn: &PgConnection,
                       name: String,
                       email: String,
                       password: String,
                       date: NaiveDateTime,
                       profile:  Option<String>,
                       role: Option<String>) {
    use schema::users;

    let new_user = NewUser {
        user_name: name,
        user_email: email,
        user_password: password,
        create_date: date,
        user_profile: profile,
        user_role: role,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");
}

pub mod schema;
pub mod models;