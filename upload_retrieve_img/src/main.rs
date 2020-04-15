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
                    let user = String::from("userName");
                    let image_name = PasteID::new(name_length);
                    let file_name = format!("{}-{image_name}", user, image_name = image_name);
                    /*-----------*/
                    let data = raw.raw;

                    // println!("{:?}", data);
                    // println!("content_type: {:?}", content_type);
                    // println!("file_name: {:?}", file_name);
                    let mut file = File::create(file_name.clone()).unwrap();

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
#[get("/back")]
pub fn get_image_back(content_type: &ContentType) -> Result<RawResponse, &'static str> {
    let mut file = File::open("/home/koompi/Desktop/upload_retrieve_img/save_image1.jpeg").unwrap();
    println!("file : {:?}", file);
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);
    let file_name = String::from("userName-CWLd");

    // Ok(RawResponse::from_vec(buffer, Some(file_name), content_type))
    Ok(RawResponse::from_vec(buffer, Some(file_name), Some(mime::IMAGE_STAR)))
}


// extern crate image;
// use image::GenericImageView;
// use image::io::Reader;

fn main() {
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
        .launch();


}
