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



pub mod models;
mod schema;