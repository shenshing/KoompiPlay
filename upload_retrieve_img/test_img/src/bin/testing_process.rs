// extern crate upload_retrieve_img;
// use self::lib::upload_retrieve_img::*;
use test_img::insert_user;


fn main() {

    let name_val = String::from("shing");
    let profile_val = String::from("shing.profile");
    let result = insert_user(name_val, profile_val);

    println!("{}", result);

}
