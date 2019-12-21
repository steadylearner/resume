use serde_derive::{Deserialize, Serialize};

// "success" should be always true for this struct. Find the better design.
// Use this instead of the code below
// https://github.com/serde-rs/json#constructing-json-values
// let user_success = json!({
//     "full_name": &hashed_full_name,
//     "success": true,
// });
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSuccess {
    pub full_name: String,
    pub success: bool,
}

// // I don't need it at this point.
// // "success" should be always false for this struct. Find the better design.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UserError {
//     pub error: String,
//     pub success: bool,
// }