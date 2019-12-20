// https://docs.rs/chrono/0.4.9/chrono/
extern crate argon2;
extern crate rand;
#[macro_use]
extern crate warp;
use warp::{http::StatusCode, Filter};
extern crate console;
use console::Style;

use argon2::{
    Config,
    hash_encoded,
    // verify_encoded,
};
use rand::Rng;

pub mod user {
    tonic::include_proto!("user");
}

// use std::future::Future;
// use user::{user_service_client::UserServiceClient, UserReply, UserRequest};

// hash(new_user.password.as_bytes())
pub fn hash(credential: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    hash_encoded(credential, &salt, &config).unwrap()
}

// pub fn verify(hash: &str, credential: &[u8]) -> bool {
//     verify_encoded(hash, credential).unwrap_or(false)
// }

// async fn get_hashed_full_name(
//     // id: String
// ) -> Result<impl warp::Reply, Box<dyn std::error::Error>> {
//     let mut client = UserServiceClient::connect("http://0.0.0.0:50051").await?;

//     let request = tonic::Request::new(UserRequest {
//         // id,
//         id: "steadylearner".to_string()
//     });

//     let response = client.get_user(request).await?;

//     println!("RESPONSE={:?}", response);

//     let UserReply {
//         id: _,
//         first_name,
//         last_name,
//         date_of_birth: _,
//     } = &response.into_inner();

//     let full_name: String = format!("{}{}", first_name, last_name);
//     Ok(full_name)
// }

// It seems warp is similar to use gulp or functional programming?
#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    let user = warp::path("user")
        .and(warp::path::param())
        .map(|id: String| format!("Give the hashed full name of {}!", id));

    let routes = warp::get().and(user);

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
