#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


extern crate userInfo;
use userInfo::insert_user;
extern crate game_back_end;
use self::game_back_end::user::*;

extern crate diesel;
use self::userInfo::*;
extern crate rocket_cors;

use rocket_contrib::templates::Template;

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, 
    Cors, CorsOptions 
};

fn main() {



    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
        "http://127.0.0.1:5500",
        "http://localhost:3000",
        "http://localhost:3001",
        "http://localhost:3002",
        "http://localhost:3003",
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
                            userData2,
                            save_player_data])
        .attach(cors)
        .attach(Template::fairing())
        .launch();

}

