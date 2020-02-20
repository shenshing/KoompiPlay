extern crate chrono;
extern crate serde;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

// #[derive(Serialize, Deserialize, Debug)]
// #[derive(Queryable, Debug, Serialize, Deserialize)]
// #[derive(Queryable, Debug)]
// pub struct User<'a> {
//     pub user_id: i32,
//     pub user_name: &'a str,
//     pub user_email: &'a str,
//     pub user_password: &'a str,
//     pub create_date: NaiveDateTime,
//     pub user_profile: Option<&'a str>,
//     pub user_role: Option<&'a str>,
// }


#[derive(Queryable, Debug)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub create_date: NaiveDateTime,
    pub user_profile: Option<String>,
    pub user_role: Option<String>,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub create_date: NaiveDateTime,
    pub user_profile: Option<String>,
    pub user_role: Option<String>,
}
