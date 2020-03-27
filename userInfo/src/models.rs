

extern crate chrono;
extern crate serde;

use serde::{Deserialize, Serialize};

use super::schema::users;
#[derive(Insertable, Deserialize, PartialEq)]
#[table_name="users"]
pub struct User {
    pub user_name:      String,
    pub user_email:     String,
    pub user_password:  String,
    pub create_date:    String,
    pub user_profile:   Option<String>,
    pub user_role:      Option<String>,
    pub phone_number:   Option<String>,
}


#[derive(Queryable, Deserialize, PartialEq)]
pub struct _User {
    pub user_id:        i32,
    pub user_name:      String,
    pub user_email:     String,
    pub user_password:  String,
    pub create_date:    String,
    pub user_profile:   Option<String>,
    pub user_role:      Option<String>,
    pub phone_number:   Option<String>,
}

use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

impl FromDataSimple for User {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

            let new_user = User {
                user_name:      String::from("username"),
                user_email:     String::from("user email"),
                user_password:  String::from("user password"),
                create_date:    String::from("date time"),
                user_profile:   Some(String::from("user profile")),
                user_role:      Some(String::from("user role")),
                phone_number:   Some(String::from("023 322 233"))
            };
        Success(new_user)
    }
}

#[derive(Deserialize)]
pub struct loginInfo {
    pub user_name: String,
    pub user_password: String,
}
impl FromDataSimple for loginInfo {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let login_info = loginInfo {
            user_name:  String::from("ok"),
            user_password: String::from("ok"),
        };
        
        Success(login_info)
    }
}

#[derive(Deserialize)]
pub struct updateItem {
    pub token:          String,
    pub newName:        String,
    pub newPassword:    String,
    pub newProfile:     String,
    pub newRole:        String,
    pub newPhone:       String
}

impl FromDataSimple for updateItem {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let update_info = updateItem {
            token:  String::from("ok"),
            newName: String::from("ok"),
            newPassword: String::from("ok"),
            newProfile: String::from("ok"),
            newRole: String::from("ok"),
            newPhone: String::from("ok"),
        };
        
        Success(update_info)
    }
}
