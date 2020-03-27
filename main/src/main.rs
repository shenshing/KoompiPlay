#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


extern crate userInfo;
use userInfo::insert_user;
// use userInfo::register;

extern crate diesel;
use self::userInfo::*;
// use userInfo::{login, 
//                admin_dashboard, 
//                user_dashboard, 
//                error_dashboard, 
//                check_user_role,
//                self_destroy,
//                updateName};

fn main() {

    rocket::ignite()
        .mount("/", routes![register, 
                            login, 
                            admin_dashboard, 
                            user_dashboard, 
                            error_dashboard,
                            check_user_role,
                            self_destroy,
                            updateName,
                            updatePassword,
                            updateProfile,
                            updateRole,
                            updatePhone])
        .launch();

}

