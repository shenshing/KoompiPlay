#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
// extern crate userInfo;


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


use self::models::{User};
pub fn insert_user(conn: &PgConnection, user: User) {
    use schema::users;

    let new_user = User {
        user_name:      user.user_name,
        user_email:     user.user_email,
        user_password:  user.user_password,
        create_date:    user.create_date,
        user_profile:   user.user_profile,
        user_role:      user.user_role,
        phone_number:   user.phone_number,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");
}

use self::models::{_User};
pub fn get_user(conn: &PgConnection) -> Vec<_User>{
    use self::schema::users::dsl::*;

    let user_list = users.load::<_User>(conn)
        .expect("Error retrieve user from database");
    return user_list;
}

extern crate rocket_contrib;
use rocket_contrib::json::Json;
mod email_addr;
use email_addr::{Validate_Email, valid_email};


#[post("/register", data = "<user>")]
pub fn register(user: Json<User>) { 
    let connection = establish_connection();

    let new_user = User {
        user_name:      user.user_name.to_string(),
        user_email:     user.user_email.to_string(),
        user_password:  user.user_password.to_string(),
        create_date:    user.create_date.to_string(),
        user_profile:   user.user_profile.clone(),
        user_role:      user.user_role.clone(),
        phone_number:   user.phone_number.clone()
    };
    insert_user(&connection, new_user);
}

enum LogIn {
    Success,
    Failed,
}
use self::models::{loginInfo};
#[post("/login", data = "<log_info>")]
pub fn login(log_info: Json<loginInfo>) -> String {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

    let user_list = get_user(&connection);
    let mut string = String::new();

    for _user in user_list.iter() {
        if(_user.user_name.trim() == log_info.user_name.trim()) {
            if(_user.user_password.trim() == log_info.user_password.trim()) {
                string = format!("Log in Successful");
                break;
            } else {
                string = format!("Log in Failed");
            }
        } else {
            string = format!("Log in Failed");
        }
    }
    return string;
}

pub mod schema;
pub mod models;