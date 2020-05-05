#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


extern crate userInfo;
use userInfo::insert_user;
// use userInfo::register;

extern crate diesel;
use self::userInfo::*;

extern crate rocket_cors;

// use rocket_contrib::templates::Template;
use rocket_contrib::templates::Template;
// use rocket_contrib::templates::Template;
// use userInfo::{login, 
//                admin_dashboard, 
//                user_dashboard, 
//                error_dashboard, 
//                check_user_role,
//                self_destroy,
//                updateName};


use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, 
    Cors, CorsOptions 
};

fn main() {

    // let cors = rocket_cors::CorsOptions::default()
    //     // .allow_credentials(true)
    //     .send_wildcard(true)
    //     .to_cors().unwrap();


    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
        // "http://localhost:8080",
        // "http://127.0.0.1:8080",
        // "http://localhost:8000",
        // "http://0.0.0.0:8000",
        "http://127.0.0.1:5500"
    ]);

    let cors = CorsOptions { 
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), 
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", 
            "token",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");

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
                            updatePhone,
                            displayUser,
                            userData,
                            test_token,
                            upload_profile, 
                            get_profile,
			                test_login,
			                userData1,
                            userData2])
        .attach(cors)
        .attach(Template::fairing())
        .launch();

}

