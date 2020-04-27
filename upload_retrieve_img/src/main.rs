#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket_include_static_resources;

extern crate rocket_raw_response;

#[macro_use]
extern crate rocket;

extern crate rocket_multipart_form_data;

use rocket::http::ContentType;
use rocket::Data;

// extern crate rand;
mod paste_id;

// use rocket_include_static_resources::{EtagIfNoneMatch, StaticResponse};

use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
    RawField,
};

// #[derive(Serialize)]
use rocket_raw_response::RawResponse;

// #[get("/")]
// fn index(etag_if_none_match: EtagIfNoneMatch) -> StaticResponse {
//     static_response!(etag_if_none_match, "html-image-uploader")
// }

#[get("/")]
pub fn index() -> String {
    format!("Welcome")
}

use std::io::prelude::*;
use std::fs::File;

use crate::paste_id::PasteID;

const name_length: usize = 4;

#[post("/upload", data = "<data>")]
fn upload(content_type: &ContentType, data: Data) -> Result<RawResponse, &'static str> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(
        MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    );

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err("The file is too large.")
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err("The file is not an image.")
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let image = multipart_form_data.raw.remove("image");

    match image {
        Some(image) => {
            match image {
                RawField::Single(raw) => {
                    let content_type = raw.content_type;
                    // let file_name = raw.file_name.unwrap_or("Image".to_string());
                    /*------------*/
                    // let user = String::from("userName");
                    // let image_name = PasteID::new(name_length);
                    // let file_name = format!("{}-{image_name}", user, image_name = image_name);
                    let file_name = format!("{}", PasteID::new(name_length));
                    /*-----------*/
                    let data = raw.raw;

                    // println!("{:?}", data);
                    // println!("content_type: {:?}", content_type);
                    // println!("file_name: {:?}", file_name);


                    // let file_path = PathBuf::from("/home/koompi/Documents/koompi-play-production/upload_retrieve_img/image-bank");
                    // let file = File::create(file_name.clone()).unwrap();
                    // let write_res = file.write(file_path, &data[0..]).unwrap();
                    
                    let file_fmt = format!("/home/koompi/Documents/koompi-play-production/upload_retrieve_img/image-bank/{}", file_name);
                    let mut file = File::create(file_fmt).unwrap();
                    // let mut file = File::create(file_name.clone()).unwrap();

                    let write_res = file.write(&data[0..]).unwrap();

                    Ok(RawResponse::from_vec(data, Some(file_name), content_type))
                    // Err("please baby")
                }
                RawField::Multiple(_) => unreachable!(),
            }
        }
        None => Err("Please input a file."),
    }
}


use std::io::BufReader;
use std::io;


// pub fn get_image_back(content_type: &ContentType, name: PasteID<'_>) -> Result<RawResponse, &'static str> {
#[get("/back")]
pub fn get_image_back() -> Result<RawResponse, &'static str> {
// pub fn get_image_back(name: PasteID<'_>) -> Result<RawResponse, &'static str> {
    // let mut file = File::open("/home/koompi/Desktop/upload_retrieve_img/save_image1.jpeg").unwrap();
    let mut file = File::open("/home/koompi/Documents/koompi-play-production/upload_retrieve_img/userName-gQuJ").unwrap();
    println!("file: {:?}", file);

    // let mut file = File::open("image-bank/boy-default-profile.jpg").unwrap();
    // let mut file = File::open("userName-d")
    // println!("inside back");
    // let mut f_string = format!("image-bank/{name}", name = name);
    // let mut file = File::open(f_string).unwrap();
    // println!("file : {:?}", file);
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);
    // let file_name = String::from("userName-CWLd");
    let file_name = String::from("a");
    // let file_name = name;


    // Ok(RawResponse::from_vec(buffer, Some(file_name), content_type))
    Ok(RawResponse::from_vec(buffer, Some(file_name), Some(mime::IMAGE_STAR)))
}


use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Image_ID {
    // id: i32
    id: String
}

use rocket::{Request, Outcome::*};
use rocket::data::{self, FromDataSimple};

impl FromDataSimple for Image_ID {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        
        let new_image = Image_ID {
            id: String::from("0")
        };

        Success(new_image)
    }  
}

extern crate rocket_contrib;
use rocket_contrib::json::Json;





/*--------send image back to browser (WORKING)-----------*/
#[post("/img", data="<image_id>")]
// fn retrieve(id: PasteID<'_>) -> Result<RawResponse, &'static str> {
// fn retrieve(id: Json<PasteID<'_>>) -> Result<RawResponse, &'static str> {
fn retrieve(image_id: Json<Image_ID>) -> Result<RawResponse, &'static str> {
    // println!("{}", id);
    // let filename = format!("upload1/{id}", id = id);
    let file_format = format!("image-bank/{id}", id = image_id.id);
    let mut file = File::open(file_format).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);

    let name = String::from("a");

    Ok(RawResponse::from_vec(buffer, Some(name), Some(mime::IMAGE_STAR)))
}


// #[get("/")]
// pub fn return_url(res: Result<RawResponse, &'static str>){
    
//     let result = match res {
//         Ok(ok) => format!("ok"),
//         Err(err) => format!("err"),
//     };

//     println!("{}", result);
// }

#[derive(Serialize)]
pub struct ST {
    string: String
}

#[post("/img1", data="<image_id>")]
pub fn retrieve1(image_id: Json<Image_ID>) -> Json<ST> {
    let file_format = format!("image-bank/{id}", id = image_id.id);
    let mut file = File::open(file_format).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);

    let name = String::from("a");

    let mut st = String::new();

    let res: Result<RawResponse, &'static str> = Ok(RawResponse::from_vec(buffer, Some(name), Some(mime::IMAGE_STAR))); 

    match res {
        Ok(ok) => st = format!("http://localhost:8000/ret_img/{}", image_id.id),
        Err(err) => st = format!("{}", err)
    }

    let r = ST {
        string: st
    };

    return Json(r);
       
}

#[get("/ret_img/<id>")]
fn retrieve_img(id: PasteID<'_>) -> Result<RawResponse, &'static str> {
    let file_format = format!("image-bank/{id}", id = id);
    let mut file = File::open(file_format).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);
    let name = String::from("a");
    Ok(RawResponse::from_vec(buffer, Some(name), Some(mime::IMAGE_STAR)))
}

#[post("/test_str", data="<image_id>")]
pub fn test_str(image_id: Json<Image_ID>) -> String {
    let st1 = format!("http://localhost:8000/ret_img/{}", image_id.id);
    println!("st1 = {}", st1);

    let st2 = format!("http://localhost:8000/ret_img/{image_id}", image_id = image_id.id);
    println!("st2 = {}", st2);

    return String::from("Ok")
}



/*------------test return url------------------------*/
// extern crate url;
// use url::{ParseError};

// #[derive(Responder)]
// use url::Url;
// // pub struct Url();

// #[get("/")]
// pub fn return_url() -> Url {
//     let str_url = format!("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open");
    
//     let typ_url = Url::parse(&str_url).unwrap();

//     return typ_url;
// }

/*
impl Serialize for RawResponse {
    fn serialize<S>(&self, serialzer: S) -> Result<S::Ok, S::Error>
    where 
        S: Serializer,
    {
        serializer.serialize_
    }
}

#[post("/img", data="<image_id>")]
// fn retrieve(id: PasteID<'_>) -> Result<RawResponse, &'static str> {
// fn retrieve(id: Json<PasteID<'_>>) -> Result<RawResponse, &'static str> {
fn retrieve(image_id: Json<Image_ID>) -> Json<RawResponse> {
    // println!("{}", id);
    // let filename = format!("upload1/{id}", id = id);
    let file_format = format!("image-bank/{id}", id = image_id.id);
    let mut file = File::open(file_format).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);

    let name = String::from("a");

    Json(RawResponse::from_vec(buffer, Some(name), Some(mime::IMAGE_STAR)))
}
*/
/*
#[get("/img/<id>")]
// fn retrieve(id: PasteID<'_>) -> Result<RawResponse, &'static str> {
fn retrieve(id: PasteID<'_>) -> Result<RawResponse, &'static str> {
    // println!("{}", id);
    // let filename = format!("upload1/{id}", id = id);
    let file_format = format!("image-bank/{id}", id = id);
    let mut file = File::open(file_format).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);

    let name = String::from("a");

    Ok(RawResponse::from_vec(buffer, Some(name), Some(mime::IMAGE_STAR)))
}
*/

// extern crate image;
// use image::GenericImageView;
// use image::io::Reader;

/*--------------test with real database--------------*/
#[macro_use]
extern crate diesel;
// extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
// use dotenv::dotenv;
use std::env;

// pub fn establish_connection

extern crate rocket_cors;

fn main() {

    // let res = return_url();
    // println!("{:?}", res);

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();;
    rocket::ignite()
        // .attach(StaticResponse::fairing(|resources| {
        //     static_resources_initialize!(
        //         resources,
        //         // "examples/image_uploader.rs"
        //         "html-image-uploader",
        //         "/home/koompi/Documents/koompi-play/r-d/rocket-multipart-form-data/examples/front-end/html/image-uploader.html",
        //     );
        // }))
        .mount("/", routes![index])
        .mount("/", routes![upload])
        .mount("/", routes![get_image_back])
        .mount("/", routes![retrieve])
        .mount("/", routes![retrieve_img])
        .mount("/", routes![retrieve1])
        .mount("/", routes![test_str])
        // .mount("/", routes![return_url])
        // .mount("/", routes![back])
        .attach(cors)
        .launch();


}
