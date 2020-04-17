extern crate serde;
use serde::{Serialize, Deserialize};

// use diesel::deserialize::QueryableByName;
/*===================== User =========================*/
// #[diesel(deserialize_as = "Type")]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone, QueryableByName)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub profile: Option<String>
}

use super::schema::users;
#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub profile: Option<String>
}

use std::error;
use std::fmt;

// impl fmt::Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "invalid")
//     }
// }
// impl error::Error for User {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         None
//     }
// }

// impl error::Error for User {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         // Generic error, underlying cause isn't tracked.
//         None
//     }
// }
/*===================== Storage =========================*/
use super::schema::storages;
#[derive(Insertable, Serialize, Deserialize)]
#[table_name="storages"]
pub struct Storage {
    pub id: i32,
    pub profile_key: String,
    pub post_key: String
}





/*===================== Profile =========================*/






/*===================== Post =========================*/