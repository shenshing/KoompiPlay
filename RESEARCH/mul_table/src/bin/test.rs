#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use mul_table::insert_user;

use mul_table::insert_to_storage;

use mul_table::establish_connection;
use diesel::sql_query;
use diesel::query_dsl::RunQueryDsl;

use mul_table::models::User;
use mul_table::search_table;
use mul_table::search_storage;
use mul_table::search_user;
use mul_table::insert_img;
use mul_table::img_struct;
// use mul_table::static_rocket_route_info_for_testing;
// use mul_table::static_rocket_route_info_for_user_data;
// use mul_table::static_rocket_route_info_for_user_data;
use mul_table::*;
// use mul_table::post_img;

// use mul_table::Test;
use mul_table::return_img;

fn main() {

    // rocket::ignite()
    //     .mount("/", routes![testing])
    //     .launch();


    // let name = String::from("you");
    // let prof = String::from("you.profile");

    // let result = insert_user(name.clone(), prof.clone());

    // println!("{}", result);


    // let name = String::from("you");
    // let id_val: i32 = 1;

    // let result = insert_to_storage(id_val, name);
    // // println!("{:#?}", result);
    // match result {
    //     Ok(_) => println!("Hello"),
    //     Err(err) => println!("Error {}", err),
    // }

/*  //query all data by "sql_query"
    let conn = establish_connection();
    let tab_name = format!("users");
    let statement = format!("Select table_name From information_schema.tables Where table_name = \'{}\';", tab_name.clone());
    

    match sql_query(statement).execute(&conn) {
        Ok(ok) => println!("{}", ok),
        Err(err) => println!("{}", err),
    }

    let st = format!("Select * from {};", tab_name.clone());
    let result: Vec<User> = sql_query(st)
        .get_results(&conn)
        .unwrap();

    // let result = tab_name.clone()
    //     .load::<User>(&conn)
    //     .unwrap();

    println!("{:#?}", result);
*/
/*
    let conn = establish_connection();
    let st = format!("Create Table ab (
                            index Integer Primary Key
                    )");
                
    match sql_query(st).execute(&conn) {
        Ok(ok) => println!("ok"),
        Err(err) => println!("{}", err)
    }
*/
/*
    let conn = establish_connection();
    let id_arg: i32 = 1;
    let name_arg = String::from("hi");

    let profile = format!("{}_{}_Profile", name_arg.clone(), id_arg);
    let post = format!("{}_{}_Post", id_arg, name_arg.clone());

    println!("profile: {}", profile);
    let profile_statement = format!(
        " Create Table {} (
            index Integer Primary Key,
            img_path Varchar Not Null Default 'Images/boy.jpg',
            img_date Timestamp Not Null Default Current_Timestamp
        ); "
    , profile);

    match sql_query(profile_statement).execute(&conn) {
        Ok(ok) => println!("ok"),
        Err(err) => println!("{}", err),
        // Err(err) => println!("error"),
    }
*/


    // let tab_name = String::from("you_11_post");
    // let result = search_table(tab_name);
    // println!("{:#?}", result);
    // if (result == Ok(1)) {
    //     println!("ok");
    // } else {
    //     println!("err");
    // }


    // let id_arg: i32 = 11;
    // let result = search_storage(id_arg).unwrap();
    // println!("result : {:#?}", result);

    // println!("id: {}", result.id);
    // println!("profile_key: {}", result.profile_key);
    // println!("post_key: {}", result.post_key);

    // let id_arg: i32 = 11;
    // let name_arg = String::from("you");
    // let result = search_user(id_arg, name_arg);
    // // println!("{:#?}", result);
    // match result {
    //     Ok(ok) => println!("{:#?}", ok),
    //     Err(err) => println!("{}", err),
    // }

    
    // let tab_name = String::from("you_11_profile");
    // let path_arg = String::from("Images/young.jpeg");
    // let result = insert_img(tab_name, path_arg);
    // println!("result : {:#?}", result);


    // let id_arg: i32 = 11;
    // let name_arg = String::from("you");
    // let btn_arg: i32 = 3;
    // let path_arg = String::from("Images/young.jpeg");

    // post_img(id_arg, name_arg, btn_arg, path_arg);

    // let conn = establish_connection();
    // let statement = format!("Select * From you_11_profile;");
    // let result: Vec<img_struct> = sql_query(statement).get_results(&conn).unwrap();
    // println!("{:#?}", result);

    let tab_name = String::from("you_11_profile");
    let tab_name1 = String::from("you_11_post");

    // let test = Test {
    //     name: String::from("hello"),
    // };

    // let img_return = return_img(tab_name1);
    // match img_return {
    //     Ok(test) => {
    //         // println!("{:#?}", test);
    //         println!("len: {}", test.len());
    //         let first = &test[0];
    //         println!("{:#?}", first);
    //         println!("first - index: {}", first.index);
    //     },
    //     Err(err) => {
    //         println!("error: {}", err);
    //     }
    // }

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
        .mount("/", routes![return_img,
                            testing,
                            user_data])
        .attach(cors)
        .launch();

/*
    let conn = establish_connection();

    let statement = format!("Select * From users where username = \'you\';");

    // let user = User {
    //     id: 0i32,
    //     username: String::from("a"),
    //     profile: Some(String::from("a"))
    // };

    // let res: User = 
    match sql_query(statement).get_result::<User>(&conn) {
        Ok(ok) => println!("{:#?}", ok),
        Err(err) => println!("{}", err),
    }    
    // let res = sql_query(statement).load::<User>(&conn);

    // println!("{:#?}", res);

    

    // match res {
    //     Ok(user) => println!("ok"),
    //     Err(err) => println!("error: {}", err)
    // }
*/
    // let user = Username {
    //     name_arg: String::from("you"),
    // };

    // let res = user_data(user);

    // println!("{:#?}", res);
}