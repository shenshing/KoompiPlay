#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate rocket_contrib;
extern crate serde;
use rocket_contrib::json::Json;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct Person {
    name: String,
    age: u16,
}

#[post("/person", data = "<person>")]
fn person(person: Json<Person>) -> String {
    // "..."
    // println!("name: {}", person.name)
    format!("name {}, age {}", person.name, person.age)
}

use std::io::Read;

use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

impl FromDataSimple for Person {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let person_ct = ContentType::new("application", "hello");
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        // Split the string into two pieces at ':'.
        let (name, age) = match string.find(':') {
            Some(i) => (string[..i].to_string(), &string[(i + 1)..]),
            None => return Failure((Status::UnprocessableEntity, "':'".into()))
        };

        // Parse the age.
        let age: u16 = match age.parse() {
            Ok(age) => age,
            Err(_) => return Failure((Status::UnprocessableEntity, "Age".into()))
        };

        // Return successfully.
        Success(Person { name, age })
    }
}
fn main() {
    rocket::ignite()
        .mount("/", routes![person])
        .launch();
}
