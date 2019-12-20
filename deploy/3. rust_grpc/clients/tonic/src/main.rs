// https://docs.rs/chrono/0.4.9/chrono/
extern crate argon2;
extern crate rand;
extern crate tonic;

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

use user::{user_service_client::UserServiceClient, UserReply, UserRequest};

// hash(new_user.password.as_bytes())
pub fn hash(credential: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    hash_encoded(credential, &salt, &config).unwrap()
}

// pub fn verify(hash: &str, credential: &[u8]) -> bool {
//     verify_encoded(hash, credential).unwrap_or(false)
// }

// curl 0.0.0.0:8000/api/user/v1/steadylearner
async fn get_hashed_user_info(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Ignore error value for a while because of the typing problem
    // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
    // Currently it returns error from gRPC server
    // Improve it when you become familiar with the Warp
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await.map(|client| client);

    let request = tonic::Request::new(UserRequest {
        id
    });

    // Ignore error value for a while because of the typing problem
    // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
    // Currently it returns error from gRPC server
    let response = client.unwrap().get_user(request).await.map(|response| response);

    println!("RESPONSE={:?}", response);

    let reply = match response {
        Ok(user) => {
            user
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            return Err(warp::reject::not_found())
        }
    };

    let UserReply {
        id: _,
        first_name,
        last_name,
        date_of_birth: _,
    } = &reply.into_inner();

    let full_name: String = format!("{} {}", first_name, last_name);
    let hashed_full_name = hash(full_name.as_bytes());
    // https://docs.rs/warp/0.1.20/warp/reply/index.html
    // Use JSON later
    Ok(warp::reply::html(hashed_full_name))
}

// It seems warp is similar to use gulp or functional programming?
#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // let api_user = path!("api" / "user" / "v1");

    // curl 0.0.0.0:8000/api/user/v1/steadylearner
    // curl 0.0.0.0:8000/user/steadylearner
    let get_user = warp::path("user")
        .and(warp::path::param::<String>())
        .and_then(get_hashed_user_info);

    let api = get_user;
    // let api = list_users.or(get_user).or(create_user).or(update_user).or(delete_user);

    let routes = api.with(warp::log("api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// 1. Prefix api/user/v1 instead of /user
// 2. Return JSON instead of html
// 3. More error handling if necessary.