use pretty_env_logger;
use warp::{
    Filter
};

use std::str;

use crate::{
    handlers::{
        user::{
            get_hashed_user_info,
        }
    },
    models::{
        user::{
            UserSuccess,
        }
    },
};

// $cargo test -- --nocapture if you want to use println! etc.

// Tonic gRPC server should be ready first.
// I expect tests similar to what used in this post.
// https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket
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