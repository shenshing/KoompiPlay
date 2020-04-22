extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub profile: Option<String>
}

use super::schema::users;
#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct newUser {
    pub name: Option<String>,
    pub profile: Option<String>
}


// impl From




/*------------------------------------------------------------------_*/


#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name="files"]
pub struct File {
    pub id: i32,
    pub images: Vec<String>
}

use super::schema::files;
#[derive(Insertable, Serialize, Deserialize)]
#[table_name="files"]
pub struct newFile {
    pub images: Vec<String>
}