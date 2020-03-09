#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate jwt;
extern crate hyper;
// extern crate crypto;
extern crate rustc_serialize;
extern crate rocket_contrib;


//nickel
use nickel::status::StatusCode::{self, Forbidden};

// hyper
use hyper::header;
// use hyper::header::{AUTHORIZATION, Bearer};
// use hyper::header::AUTHORIZATION;
// use hyper::header::Bearer;

// jwt
use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{
    Header,
    Registered,
    Token,
};

static AUTH_SECRET: &'static str = "OfqUilNpMrwkyh9pmqaR5xP3"; 


use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};
// use rocket::response::content::Json;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(RustcDecodable, RustcEncodable, Deserialize)]
pub struct UserLogin {
    email: String,
    password: String
}

impl FromDataSimple for UserLogin {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let user_login= UserLogin {
            email:  String::from("ok"),
            password: String::from("ok"),
        };
        
        Success(user_login)
    }
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<UserLogin>) -> String { 
    // let connection = establish_connection();
    
    let user_login = UserLogin {
        email: user.email.to_string(),
        password: user.password.to_string()
    };
    // insert_user(&connection, new_user);?
    // let pass = "123".to_string() {
    let pass = String::from("123");
    if user.password.to_string() == pass {
        let header: Header = Default::default();

        // lelt claims = Registered {
        //     sub: Some(user.email.into()),
        //     ..Default::default()
        // };
        let claims = Registered {
            iss:     Default::default(),
            sub:    Some(user.email.to_string()),
            aud:    Default::default(),
            exp:    Default::default(),
            nbf:    Default::default(),
            iat:    Default::default(),
            jti:    Default::default()
        };

        let token = Token::new(header, claims);

        let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();

        format!("{}", jwt)
    } else {
        format!("Incorrect username or password")
    }
} 

fn main() {
    rocket::ignite()
        .mount("/", routes![register])
        .launch();
}
