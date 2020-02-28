

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
/*
use diesel::pg::data_types::PgDate;
#[derive(Queryable, Debug)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_profile: Option<String>,
    pub user_role: Option<String>,
}
*/
// use chrono::naive::NaiveDateTime;
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use rocket::request::FromForm;
// use chrono::Datelike;

// impl FromForm for NaiveDateTime {

// }
// impl<'v, T: Datelike> FromFormValue<'v> for Option<T> {
//     type Error = &'v RawStr;

//     // fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateTime, &'v RawStr> {
//     //     match form_value.parse::<NaiveDateTime>() {
//     //         Ok(date) => Ok(NaiveDateTime{form_value}),
//     //         _ => Err(form_value),
//     //     }
//     // }
// }


// struct AdultAge(usize);

// impl<'v> FromFormValue<'v> for AdultAge {
//     type Error = &'v RawStr;

//     fn from_form_value(form_value: &'v RawStr) -> Result<AdultAge, &'v RawStr> {
//         match form_value.parse::<usize>() {
//             Ok(age) if age >= 21 => Ok(AdultAge(age)),
//             _ => Err(form_value),
//         }
//     }
// }


use super::schema::users;
use rocket::request::Form;
// use serde::Deserialize;


#[derive(Queryable, Insertable, Deserialize)]
#[table_name="users"]
pub struct User {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub create_date: String,
    pub user_profile: Option<String>,
    pub user_role: Option<String>,
}

use std::io::Read;

use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

impl FromDataSimple for User {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        
        // let create_at = Utc::now();
        // let create_at = Local::now().format("%A-%Y-%m-%d %H:%M:%S");
        // println!("{}", date);
        // Return successfully.
        // Success(Person { name, age, create_at})

        let profile = Some(String::from("user profile"));
        let role = Some(String::from("user role"));
            let new_user = User {
                user_name: String::from("username"),
                user_email: String::from("user email"),
                user_password: String::from("user password"),
                create_date: String::from("date time"),
                user_profile: profile,
                user_role: role,
            };
        Success(new_user)
    }
}

// #[get("/item?<id>&<user..>")]
// #[get("/login?<id>?<user..>")]
// fn login(id: i32, user: Form<User>) {

// }
