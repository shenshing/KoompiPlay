extern crate chrono;
use chrono::Local;
// use chrono::*;
use chrono::prelude::*;

fn main() {
    let date_str = "Wednesday-2020-02-26-04:31:15";
    let _date = Local.datetime_from_str(&date_str, "%A-%Y-%m-%d %H:%M:%S");
    match _date {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e)
    }

    let vec: Vec<&str> = date_str.split('-').collect();
    for st in vec.iter() {
        println!("{}", st);
    }

    let date_str = format!("{}-{}-{}-{}", vec[0], vec[1], vec[2], vec[3]);
    if date_str == String::from("Wednesday-2020-02-26") {
        println!("Match");
    } else {
        println!("Not Match");
    }
}
