#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
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

#[derive(Debug, PartialEq)]
pub enum deleteMessage {
    Success,
    Unsuccess,
}

use self::diesel::prelude::*;
use crate::schema::users;
pub fn remove_user(userName: String, userPassword: String) -> deleteMessage {
    use self::schema::users::dsl::*;

    let name_pattern = format!("%{}%", format_args!("{}", userName));
    let password_pattern = format!("%{}%", format_args!("{}", userPassword));

    let connection = establish_connection();
    let delete_user = diesel::delete(users.filter(user_name.like(name_pattern)).filter(user_password.like(password_pattern)))
        .execute(&connection);
    if(delete_user == Ok(1)) {
        return deleteMessage::Success;
    } else {
        return deleteMessage::Unsuccess;
    }
}

#[derive(Debug, PartialEq)]
pub enum updateMessage {
    Success,
    Unsuccess,
}

pub fn update_name(oldUserName: String, userPassword: String, newUserName: String) -> updateMessage {
    use self::schema::users::dsl::{users, user_name};

    let update_name = diesel::update(users.filter(user_name.eq(oldUserName))
        .filter(user_password.eq(userPassword)))
        .set(user_name.eq(newUserName))
        .execute(&establish_connection());
    if(update_name == Ok(1)) {
        return updateMessage::Success;
    } else {
        return updateMessage::Unsuccess;
    }
}

pub fn update_password(userName: String, userPassword: String, newUserPassword: String) -> updateMessage {
    use self::schema::users::dsl::{users, user_name, user_password};

    let update_password = diesel::update(users.filter(user_name.eq(userName))
        .filter(user_password.eq(userPassword)))
        .set(user_password.eq(newUserPassword))
        .execute(&establish_connection());
    if(update_password == Ok(1)) {
        return updateMessage::Success;
    } else {
        return updateMessage::Unsuccess;
    }
}

pub fn update_profile(userName: String, userPassword: String, newUserProfile: String) -> updateMessage {
    use self::schema::users::dsl::{users, user_profile, user_name};

    let update_profile = diesel::update(users.filter(user_name.eq(userName))
        .filter(user_password.eq(userPassword)))
        .set(user_profile.eq(newUserProfile))
        .execute(&establish_connection());
    if(update_profile == Ok(1)) {
        return updateMessage::Success;
    } else {
        return updateMessage::Unsuccess;
    }
}

pub fn update_role(userName: String, userPassword: String, newUserRole: String) -> updateMessage {
    use self::schema::users::dsl::{users, user_name, user_role};

    let update_role = diesel::update(users.filter(user_name.eq(userName))
        .filter(user_password.eq(userPassword)))
        .set(user_role.eq(newUserRole))
        .execute(&establish_connection());
    if(update_role == Ok(1)) {
        return updateMessage::Success;
    } else {
        return updateMessage::Unsuccess;
    }
}

pub fn update_phone(userName: String, userPassword: String, newUserPhone: String) -> updateMessage {
    use self::schema::users::dsl::{users, user_name, phone_number};

    let update_phone = diesel::update(users.filter(user_name.eq(userName))
        .filter(user_password.eq(userPassword)))
        .set(phone_number.eq(newUserPhone))
        .execute(&establish_connection());
    if(update_phone == Ok(1)) {
        return updateMessage::Success;
    } else {
        return updateMessage::Unsuccess;
    }
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
                let role = _user.user_role.as_ref().unwrap();
                string = generate_token(_user.user_name.to_string(),   
                                        _user.user_password.to_string(), 
                                        role.to_string());
                // check_user_role(string.clone());
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

#[post("/delete", data = "<token>")]
pub fn self_destroy(token: String) -> String {
    let dec_res = decode_token(token);
    let userName = dec_res.claims.user_name;
    let userPassword = dec_res.claims.user_password;

    if(remove_user(userName.clone(), userPassword.clone()) == deleteMessage::Success) {
        format!("user delete successfull")
    } else if (remove_user(userName.clone(), userPassword.clone()) == deleteMessage::Unsuccess) {
        format!("user delete unsuccessful") 
    } else {
        format!("Something went wrong when delete user")
    }
}

use crate::schema::users::columns::user_password;
use self::models::updateItem;
#[post("/updateName", data = "<newInfo>")]
pub fn updateName(newInfo: Json<updateItem>) -> String {
    let dec_res = jsonwebtoken::decode::<Claims>(&newInfo.token.clone(), "secret".as_ref(), &Validation::default()).unwrap();
    let userName = dec_res.claims.user_name;
    let userPassword = dec_res.claims.user_password;
    let new_name = newInfo.newName.clone();

    if(update_name(userName.clone(), userPassword.clone(), new_name.clone()) == updateMessage::Success) {
        format!("update userName Successfully")
    } else if (update_name(userName.clone(), userPassword.clone(), new_name.clone()) == updateMessage::Unsuccess) {
        format!("update userName Unsuccessful")
    } else {
        format!("Something went wrong when trying to update \"userName : {} \" to \"userName : {} \"", userName.clone(), new_name.clone())
    }
}

#[post("/updatePassword", data = "<newInfo>")]
pub fn updatePassword(newInfo: Json<updateItem>) -> String {
    let dec_res = jsonwebtoken::decode::<Claims>(&newInfo.token.clone(), "secret".as_ref(), &Validation::default()).unwrap();
    let userName = dec_res.claims.user_name;
    let userPassword = dec_res.claims.user_password;
    let new_password = newInfo.newPassword.clone();

    if(update_password(userName.clone(), userPassword.clone(), new_password.clone()) == updateMessage::Success) {
        format!("update user password Successfully")
    } else if (update_password(userName.clone(), userPassword.clone(), new_password.clone()) == updateMessage::Unsuccess) {
        format!("update user password Unsuccessful")
    } else {
        format!("Something went wrong when trying to update Password")
    }
}

#[post("/updateProfile", data = "<newInfo>")]
pub fn updateProfile(newInfo: Json<updateItem>) -> String {
    let dec_res = jsonwebtoken::decode::<Claims>(&newInfo.token.clone(), "secret".as_ref(), &Validation::default()).unwrap();
    let userName = dec_res.claims.user_name;
    let userPassword = dec_res.claims.user_password;
    let new_profile = newInfo.newProfile.clone();

    if(update_profile(userName.clone(), userPassword.clone(), new_profile.clone()) == updateMessage::Success) {
        format!("update user profile Successfully")
    } else if (update_profile(userName.clone(), userPassword.clone(), new_profile.clone()) == updateMessage::Unsuccess) {
        format!("update user profile Unsuccessful")
    } else {
        format!("Something went wrong when trying to update Profile")
    }
}

#[post("/updateRole", data = "<newInfo>")]
pub fn updateRole(newInfo: Json<updateItem>) -> String {
    let dec_res = jsonwebtoken::decode::<Claims>(&newInfo.token.clone(), "secret".as_ref(), &Validation::default()).unwrap();
    let userName = dec_res.claims.user_name;
    let userPassword = dec_res.claims.user_password;
    let new_role = newInfo.newRole.clone();

    if(update_role(userName.clone(), userPassword.clone(), new_role.clone()) == updateMessage::Success) {
        format!("update user role Successfully")
    } else if (update_role(userName.clone(), userPassword.clone(), new_role.clone()) == updateMessage::Unsuccess) {
        format!("update user role Unsuccessful")
    } else {
        format!("Something went wrong when trying to update Role")
    }
}

#[post("/updatePhone", data = "<newInfo>")]
pub fn updatePhone(newInfo: Json<updateItem>) -> String {
    let dec_res = jsonwebtoken::decode::<Claims>(&newInfo.token.clone(), "secret".as_ref(), &Validation::default()).unwrap();
    let userName = dec_res.claims.user_name;
    let userPassword = dec_res.claims.user_password;
    let new_role = newInfo.newPhone.clone();

    if(update_phone(userName.clone(), userPassword.clone(), new_role.clone()) == updateMessage::Success) {
        format!("update user phone number Successfully")
    } else if (update_phone(userName.clone(), userPassword.clone(), new_role.clone()) == updateMessage::Unsuccess) {
        format!("update user phone number Unsuccessful")
    } else {
        format!("Something went wrong when trying to update Phone Number")
    }
}
use std::time::{SystemTime};
extern crate jsonwebtoken;
use jsonwebtoken::{Header, decode, Validation};
extern crate chrono;
use chrono::{Utc, DateTime, Duration};
pub use self::token::{Claims, generate_token};


#[get("/admin")]
pub fn admin_dashboard() -> String {
    format!("Admin dashboard")
}

#[get("/user")]
pub fn user_dashboard() -> String {
    format!("User dashboard")
}

#[get("/error")]
pub fn error_dashboard() -> String {
    format!("Error dashboard")
}

#[get("/delete_success")]
pub fn delete_sucess() -> String {
    format!("Self Destoy completed!!!")
}

use rocket::response::Redirect;
use token::decode_token;
#[post("/checking", data = "<token>")]
pub fn check_user_role(token: String) -> Redirect {
    //I don't why it has "token=a;sldkfja;sldjfa;lsdf" 
    //when i try to send token from postman so i need to delete some string before
    //token to make it become ValidToken
    // let dec_res = jsonwebtoken::decode::<Claims>(&ok_token, "secret".as_ref(), &Validation::default()).unwrap();
    
    let dec_res = decode_token(token);
    let user_role = dec_res.claims.user_role;
    println!("user role = {}", user_role);

    if(user_role == "Admin".to_string()) {
        //redirect to admin dashboard
        Redirect::to(uri!(admin_dashboard))
    } else if (user_role == "User".to_string()) {
        //redirect to user dasboard
        Redirect::to(uri!(user_dashboard))
    } else {
        //user role conflict
        Redirect::to(uri!(error_dashboard))
    }
}


pub mod schema;
pub mod models;
pub mod token;