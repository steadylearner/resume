use serde_derive::{Deserialize, Serialize};

// I want to use this but there are many type relevant problems with this.
// use chrono::NaiveDate;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct NewUser {
//     pub first_name: String,
//     pub last_name: String,
//     pub date_of_birth: NaiveDate,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}

// Use this instead of the code below
// https://github.com/serde-rs/json#constructing-json-values
// let user_success = json!({
//     "full_name": &hashed_full_name,
//     "success": true,
// });
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSuccess {
    pub full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSuccessList {
    pub users: Vec<UserSuccess> // or Vec<Option<UseSuccess>>?
}

// // I don't need it at this point.
// // "success" should be always false for this struct. Find the better design.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UserError {
//     pub error: String,
//     pub success: bool,
// }