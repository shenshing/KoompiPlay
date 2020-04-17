use mul_table::insert_user;

use mul_table::insert_to_storage;

use mul_table::establish_connection;
use diesel::sql_query;
use diesel::query_dsl::RunQueryDsl;

use mul_table::models::User;
fn main() {

    let name = String::from("you");
    let prof = String::from("you.profile");

    let result = insert_user(name.clone(), prof.clone());

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
}