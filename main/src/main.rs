#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


extern crate userInfo;
use userInfo::insert_user;
// use userInfo::register;

extern crate diesel;
use self::userInfo::*;
use userInfo::login;


fn main() {

    rocket::ignite()
        .mount("/", routes![register, login])
        .launch();

    // let connection = establish_connection();
    // get_user(&connection);
}

