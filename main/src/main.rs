extern crate userInfo;
// mod users;
// use userInfo::hello;
use userInfo::establish_connection;
use userInfo::insert_user;

extern crate diesel;
use self::userInfo::*;
use self::models::*;
use self::diesel::prelude::*;

use chrono::offset::Utc;

fn main() {
    // println!("Hello, world!");
    // hello();
    use userInfo::schema::users::dsl::*;
    // use users::schema;

    let connection = establish_connection();
//show_user

//     let user_result = users.load::<User>(&connection)
//         .expect("Error loading posts");

//     println!("Displaying {} users", user_result.len());
//     // if user_result.len() == 0 {
//     //     println!("Database is EMPTY");
//     // }

//     println!("{:#?}", user_result);
// 

//insert_user
    let name = String::from("shenshing");
    let email = String::from("shing@gmail.com");
    let password = String::from("123");
    let date = Utc::now().naive_utc();
    let profile = String::from("userProfile");
    let role = String::from("Admin");

    // println!("name:     {}", name);
    // println!("email:    {}", email);
    // println!("password: {}", password);
    // println!("date:     {}", date);
    // println!("profile:  {}", profile);
    // println!("role:     {}", role);

    //insert_user(&connection,
    //            name,
    //            email,
    //            password,
    //            date,
    //            Some(profile),
    //            Some(role));



    // for user in user_result {
    //     println!("{}", user.user_name);
    // }
}


// extern crate userInfo;

// fn main() {

// }
