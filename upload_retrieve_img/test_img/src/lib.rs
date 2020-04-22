#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


// extern crate serde;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Database URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{newUser, User};
pub fn insert_user(name_val: String, profile_val: String) -> String {
    use crate::schema::users;
    use crate::schema::files;
    use crate::schema::files::columns::id;
    
    let conn = establish_connection();
    let new_user = newUser {
        name: Some(name_val.clone()),
        profile: Some(profile_val),
    };

    let inserted_user: User = diesel::insert_into(users::table)
        .values(&new_user)
        // .execute(&conn)
        .get_result(&conn)
        .unwrap();
        
    // match insert_user {
    //     Ok(ok) => format!("ok"),
    //     Err(err) => format!("err")
    // }

    if (inserted_user.name == Some(name_val.clone())) {
        let file_id = inserted_user.id;
        let file_image: Vec<String> = Vec::new();


        let insert_image = insert_file(7i32, file_image);
        println!("insert_image = {}", insert_image);
            if(insert_image == String::from("Ok")) {
                return format!("User and Image created");
            } else {
                return format!("User and Image are not created1");
            }
    } else {
        return format!("User and Image are not created2");
    }
}

// pub fn create_user()
/*-----------------------------------*/



use self::models::{newFile, File};
pub fn insert_file(id_val: i32, image: Vec<String>) -> String {
    use crate::schema::files;

    let conn = establish_connection();
    let new_file = File {
        id: id_val,
        images: image,
    };

    let insert = diesel::insert_into(files::table)
        .values(&new_file)
        .execute(&conn);

    match insert {
        Ok(ok) => format!("Ok"),
        Err(err) => format!("{}", err),
    }
}













mod models;
mod schema;