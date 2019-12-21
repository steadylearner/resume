use tonic;
use warp;

use crate::{
    models::{
        user::{
            UserSuccess,
        }
    },
    user::{
        user_service_client::UserServiceClient,
        UserReply, UserRequest
    },
    crypto::{
        hash::{
            hash_with_argon2,
        }
    },
};

pub async fn get(id: String) -> Result<impl warp::Reply, warp::Rejection> {
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
            // Use JSON later and build it with serde with success: false ?
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