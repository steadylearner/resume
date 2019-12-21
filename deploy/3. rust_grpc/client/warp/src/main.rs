extern crate pretty_env_logger;
#[macro_use] extern crate log;

extern crate tonic;
mod user {
    tonic::include_proto!("user");
}
use user::{
    user_service_client::UserServiceClient,
    UserReply, UserRequest
};

extern crate argon2;
extern crate rand;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate warp;
use warp::{
    Filter
};

extern crate console;
use console::Style;

use std::str;

// This is where all your custom modules will be.
mod crypto;
mod models;
use self::{
    crypto::{
        hash::{
            hash_with_argon2,
        }
    },
    models::{
        user::{
            UserSuccess,
        }
    }
};

async fn get_hashed_user_info(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Ignore error value for a while because of the typing problem
    // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
    // Currently it prints error from gRPC server with println!("RESPONSE={:?}", response);
    // Improve it when you become familiar with the Warp
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    let request = tonic::Request::new(UserRequest {
        id
    });

    // Ignore error value for a while because of the typing problem
    let response = client.unwrap().get_user(request).await
        .map(|response| response);

    println!("RESPONSE={:#?}", response);

    // It is required.
    let reply = match response {
        Ok(user) => {
            user
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // Use JSON later and build it with serge with success: false ?
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
    let hashed_full_name = hash_with_argon2(full_name.as_bytes());

    let user_success = UserSuccess {
        full_name: hashed_full_name,
        success: true,
    };

    Ok(warp::reply::json(&user_success))
}

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // curl 0.0.0.0:8000/api/user/v1/steadylearner
    let get_user = path!("api" / "user" / "v1")
        .and(warp::path::param::<String>())
        .and_then(get_hashed_user_info);

    let api = get_user;
    // The complete form should be this with tests
    // let api = list_users.or(get_user).or(create_user).or(update_user).or(delete_user);

    let routes = api.with(warp::log("api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// Tonic gRPC server should be ready first.
// I expect tests similar to what used in this post.
// https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket
// https://docs.rs/warp/0.1.20/warp/test/struct.RequestBuilder.html
#[tokio::test]
async fn get_user() {
    let _ = pretty_env_logger::init();

    // Extract this duplicate code?
    let get_user = path!("api" / "user" / "v1")
        .and(warp::path::param::<String>())
        .and_then(get_hashed_user_info);

    let res = warp::test::request().path("/api/user/v1/steadylearner") // 1. Define path with datas
        .reply(&get_user).await; // 2. Use the exact payload you want to test and reply can be target, responder, reply_with.

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    // Make this function?
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap(); // res.body here is b"" utf8 bytes
    let UserSuccess {
        full_name: _,
        success
    } = serde_json::from_str(&body_str_from_byte).unwrap();
    assert_eq!(success, true);
}

// 1. Modulize and Organize the API(Separate the test file)
// 2. Make CRUD Rest API with tests, more error handling if necessary.