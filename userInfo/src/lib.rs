// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// #![feature(decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
// #[macro_use]
// extern crate rocket;

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
// use self::models::{User};
// pub fn insert_user(conn: &PgConnection,
//                        name: String,
//                        email: String,
//                        password: String,
//                        date: String,
//                        profile:  Option<String>,
//                        role: Option<String>) {
//     use schema::users;

//     let new_user = User {
//         user_name: name,
//         user_email: email,
//         user_password: password,
//         create_date: date,
//         user_profile: profile,
//         user_role: role,
//     };

//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .execute(conn)
//         .expect("Error saving new user");
// }


use self::models::{User};
pub fn insert_user(conn: &PgConnection, user: User) {
    use schema::users;

    let new_user = User {
        user_name:      user.user_name,
        user_email:     user.user_email,
        user_password:  user.user_password,
        create_date:    user.create_date,
        user_profile:   user.user_profile,
        user_role:      user.user_role
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");
}

// extern crate userInfo;
// use self::userInfo::*;
// use std::env::args;

// pub fn update_user(conn: &PgConnection,
//                    table: self::schema::users::dsl::users,
//                    ) {
                  
// }

// use rocket::request::Form;
extern crate rocket_contrib;
use rocket_contrib::json::Json;

#[post("/register", data = "<user>")]
pub fn register(user: Json<User>) { 
    // format!("{}\t{}\t{}\t{}\t{}\t{}", 
    //     user.user_name,
    //     user.user_email,
    //     user.user_password,
    //     user.create_date,
    //     user.user_profile.as_ref().unwrap(),
    //     user.user_role.as_ref().unwrap())

    let connection = establish_connection();

    let new_user = User {
        user_name:      user.user_name.to_string(),
        user_email:     user.user_email.to_string(),
        user_password:  user.user_password.to_string(),
        create_date:    user.create_date.to_string(),
        user_profile:   user.user_profile.clone(),
        user_role:      user.user_role.clone()
    };
    insert_user(&connection, new_user);
}

pub mod schema;
pub mod models;