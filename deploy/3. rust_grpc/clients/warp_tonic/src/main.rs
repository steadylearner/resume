extern crate pretty_env_logger;
#[macro_use] extern crate log;

extern crate argon2;
extern crate rand;
extern crate tonic;
extern crate serde;
extern crate serde_json;
use serde_json::json;

#[macro_use]
extern crate warp;
use warp::{
    // http::StatusCode,
    Filter
};
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
    // Currently it prints error from gRPC server with println!("RESPONSE={:?}", response);
    // Improve it when you become familiar with the Warp
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await.map(|client| client);

    let request = tonic::Request::new(UserRequest {
        id
    });

    // Ignore error value for a while because of the typing problem
    let response = client.unwrap().get_user(request).await.map(|response| response);

    println!("RESPONSE={:?}", response);

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
    let hashed_full_name = hash(full_name.as_bytes());
    // https://docs.rs/warp/0.1.20/warp/reply/index.html
    // Ok(warp::reply::html(hashed_full_name))

    // https://github.com/serde-rs/json#constructing-json-values
    // let user_success = json!({
    //     "full_name": &hashed_full_name,
    //     "success": true,
    // });

    Ok(warp::reply::json(&user_success))
}

// It seems warp is similar to use gulp or functional programming?
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
    // let api = list_users.or(get_user).or(create_user).or(update_user).or(delete_user);

    let routes = api.with(warp::log("api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// 1. Make JSON with module instead of json!, Modulize and Organize the API, Make CRUD Rest API with tests
// 2. More error handling if necessary.

// Server should be ready first.
// I expect tests similar to what used in this post.
// https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket
// https://docs.rs/warp/0.1.20/warp/test/struct.RequestBuilder.html
#[tokio::test]
async fn get_user() {
    // Use it or not?
    let _ = pretty_env_logger::init();

    // get api/user/v1/steadylearner ==> return JSON with success: true
    // https://github.com/seanmonstar/warp/blob/master/tests/redirect.rs
    let get_user = path!("api" / "user" / "v1")
        .and(warp::path::param::<String>())
        .and_then(get_hashed_user_info);

    let res = warp::test::request().path("/api/user/v1/steadylearner") // 1. Define path with datas
        .reply(&get_user).await; // 2. Use the exact payload you want to test and reply can be target, responder, reply_with.

    assert_eq!(res.status(), 200, "Should return 200 OK.");

    // assert_eq!(res.body(), "true", "Should return JSON with {{ 'success': true }}");
}