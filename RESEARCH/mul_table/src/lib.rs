#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;
use rocket_contrib::json::Json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Database URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


/*===================== User =========================*/
use diesel::sql_query;
use self::models::{NewUser, User};
pub fn insert_user(name_val: String, profile_val: String) -> String {
    use crate::schema::users;
    let conn = establish_connection();
    let new_user = NewUser {
        username: name_val.clone(),
        profile: Some(profile_val),
    };

    let _inserted_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&conn);
    
    // println!("{:#?}", _inserted_user);
    
    let user = User {
        id: 0i32,
        username: String::from("name"),
        profile: Some(String::from("profile"))
    };

    let mut return_st = String::new();

    match _inserted_user {
        Ok(user) => {
            // println!("id: {}", user.id);
            // println!("name: {}", user.username);
            // println!("profile: {:?}", user.profile);
            let id_arg = user.id;
            let name_arg = user.username;

            match insert_to_storage(id_arg, name_arg.clone()) {
                Ok(ok) => {
                    let profile = format!("{}_{}_Profile", name_arg.clone(), id_arg);
                    let post = format!("{}_{}_Post", name_arg.clone(), id_arg);

                    let profile_statement = format!(
                        "Create Table {} (
                            index Integer Primary Key,
                            img_path Varchar Not Null Default 'Images/boy.jpg',
                            img_date Timestamp Not Null Default Current_Timestamp
                        )"
                    , profile);

                    let post_statement = format!(
                        "Create Table {} (
                            index Integer Primary Key,
                            img_path Varchar Not Null Default 'Images/boy.jpg',
                            img_date Timestamp Not Null Default Current_Timestamp
                        )"
                    , post);

                    match sql_query(profile_statement).execute(&conn) {
                        Ok(ok) => {
                            match sql_query(post_statement).execute(&conn) {
                                Ok(ok) => return_st = format!("Everything set up"),
                                Err(err) => return_st = format!("error post statement: {}", err),
                            }
                        },
                        Err(err) => {
                            return_st = format!("error profile statement: {}", err);
                        }
                    }
                    
                    // return_st = format!("Everything set up");
                    
                },
                Err(err) => {
                    return_st = format!("error insert to storage");
                }
            }
        },
        // Err(err) => println!("No Hello"),
        Err(err) => return_st = format!("error insert user"),
    };

    // format!("OK")
    return return_st;
}



/*===================== Storage =========================*/
//id_val & name_val are get from "user"
use self::models::{Storage};
// pub fn insert_to_storage(id_val: i32, name_val: String) -> Option<Storage> {
pub fn insert_to_storage(id_arg: i32, name_arg: String) -> QueryResult<usize> {
    use crate::schema::storages;
    let conn = establish_connection();

    let new_storage = Storage {
        id: id_arg,
        profile_key: format!("{}_{}_Profile", name_arg, id_arg),
        post_key: format!("{}_{}_Post", name_arg, id_arg),
    };

    let _insert_storage = diesel::insert_into(storages::table)
        .values(&new_storage)
        .execute(&conn);

    return _insert_storage;
}







/*===================== Profile =========================*/








/*===================== Post =========================*/







/*===================== etc =========================*/

pub fn search_table(tab_name: String) -> QueryResult<usize> {
    let conn = establish_connection();


    let statement = format!("Select table_name From information_schema.tables Where table_name = '{}';",
        tab_name.clone());
    
    match sql_query(statement).execute(&conn) {
        Ok(ok) => return Ok(ok),
        Err(err) => return Err(err),
    }
}

pub fn search_storage(id_arg: i32) -> Result<Storage, diesel::result::Error> {
    use self::schema::storages::dsl::*;

    let conn = establish_connection();

    match storages.filter(id.eq(id_arg))
        .get_result::<Storage>(&conn) {
            Ok(store) => return Ok(store),
            Err(err) => return Err(err)
    }

    // return result;
}

// pub fn search_user(id_arg: i32, name_arg: String) -> Result<User, core::result::Result> {
pub fn search_user(id_arg: i32, name_arg: String) -> Result<User, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let conn = establish_connection();

    match users.filter(id.eq(id_arg))
        .filter(username.eq(name_arg))
        .get_result::<User>(&conn) {
        Ok(user) => return Ok(user),
        Err(err) => return Err(err),
    }
}

use rocket::{Request, Data, Outcome, Outcome::*};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct test {
    id_arg: i32,
    name_arg: String,
    btn_arg: i32,
    path_arg: String
}


use rocket::data::{self, FromDataSimple};

impl FromDataSimple for test {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        // let now = SystemTime::now();
        let test_ = test {
            id_arg: 0i32,
            name_arg: String::from("test"),
            btn_arg: 0i32,
            path_arg: String::from("test")
        };
        Success(test_)
    }
}

#[post("/testing", data="<arg>")]
pub fn testing(arg: Json<test>) -> String {
// pub fn post_img(id_arg: i32, name_arg: String, btn_arg: i32, path_arg: String) {
    let conn = establish_connection();
    // let return_st = String::new();

    let mut final_res_st = String::new();

    match search_user(arg.id_arg, arg.name_arg.clone()) {
        Ok(user) => {
            //user found
            match search_storage(arg.id_arg) {
                Ok(ok) => {
                    //storage found
                    //divide by btn_arg that passed
                    if(arg.btn_arg == 1) {
                        //search for profile table
                        let table_name = ok.profile_key.clone();
                            
                        match search_table(ok.profile_key) {
                            Ok(ok) => {
                                match insert_img(table_name, arg.path_arg.clone()) {
                                    Ok(ok) => final_res_st = format!("ok insert to profile table"),
                                    Err(err) => final_res_st = format!("insert to profile table error: {}", err),
                                }
                            },
                            Err(err) => {
                                final_res_st = format!("search table error: {}", err);
                            }
                        }
                    } else if (arg.btn_arg == 2) {
                        //search for post table
                        let table_name = ok.post_key.clone();

                        match search_table(ok.post_key) {
                            Ok(ok) => {
                                match insert_img(table_name, arg.path_arg.clone()) {
                                    Ok(ok) => final_res_st = format!("ok insert to post table"),
                                    Err(err) => final_res_st = format!("insert to post table error"),
                                }
                            },
                            Err(err) => {
                                final_res_st = format!("insert to storage error: {}", err);
                            }
                        }
                    } else {
                        final_res_st = format!("invalid btn_arg");
                    }
                },
                Err(err) => {{
                    final_res_st = format!("search storage error: {}", err);
                }}
            }
        }, 
        Err(err) => {
            final_res_st = format!("search user error: {}", err);
        }
    }
    return final_res_st;
}


pub fn insert_img(tab_name: String, path_arg: String) -> QueryResult<usize> {
    let conn = establish_connection();

    let statement = format!("Insert Into {} (img_path) Values (\'{}\');",
        tab_name, path_arg);
    
    let result = match sql_query(statement).execute(&conn) {
        Ok(ok) => return Ok(ok),
        Err(err) => return Err(err),
    };
}

use std::time::SystemTime;
use diesel::sql_types::{Integer, Varchar, Timestamp};
#[derive(Debug, QueryableByName, Serialize, Deserialize)]
// #[table_name=""]
pub struct img_struct {
    #[sql_type = "Integer"]
    pub index: i32,
    #[sql_type = "Varchar"]
    pub img_path: String,
    #[sql_type = "Timestamp"]
    pub img_date: SystemTime
}

impl FromDataSimple for img_struct {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome::<Self, String> {
        let now = SystemTime::now();

        let new_img = img_struct {
            index: 0i32,
            img_path: String::from("path"),
            img_date: now
        };

        Success(new_img)
    }
}


#[derive(Serialize, Deserialize)]
pub struct userArg {
    user_id: i32,
    user_name: String
}


// pub fn retun_user_data(name_arg: String) {

// }

// #[derive(Serialize, Debug, QueryableByName)]
// pub struct Test {
    
//     #[sql_type = "Varchar"]
//     pub name: String
// }

use rocket::response::Redirect;
#[get("/all_img/<tab_name>")]
pub fn return_img(tab_name: String) -> Result<Json<Vec<img_struct>>, diesel::result::Error> {
// pub fn return_img(tab_name: String) -> Json<Vec<img_struct>> {    
    // let res: Vec<Test> = Vec::new();

    let conn = establish_connection();
    let statement = format!("Select * From {};", tab_name);
    let res: Vec<img_struct> = sql_query(statement).get_results(&conn).unwrap();
    Ok(Json(res))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Username {
    pub name_arg: String,
}

impl FromDataSimple for Username {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome::<Self, String> {
        // let now = SystemTime::now();

        let new_username = Username {
            name_arg: String::from("hello"),
        };

        Success(new_username)
    }
}

// // use std::fmt::Debug;
use rocket::response::Debug;
#[post("/user", data="<input_info>")]
// pub fn user_data(input_info: Json<Username>) -> Result<Json<User>, diesel::result::Error> {
// pub fn user_data(input_info: Username) -> Result<Json<User>, Debug<diesel::result::Error>> {
// pub fn user_data(input_info: Json<Username>) -> Option<Json<User>, diesel::result::Error> {
pub fn user_data(input_info: Json<Username>) -> Json<User> {    
    let conn = establish_connection();
    println!("name: {}", input_info.name_arg);
    
    let statement = format!("Select * From users Where username = \'{}\';", input_info.name_arg);
    println!("{}", statement);
    
    let res = sql_query(statement).get_result::<User>(&conn).unwrap();
    //  {
    //     Ok(user) => return user,
    //     Err(err) => return err
    // };

    return Json(res)

    // match res {
    //     Ok(ok) => return Ok(Json(ok)),
    //     Err(err) => return Err(err),
    // }
    // Ok(Json(res))
}

pub mod models;
mod schema;