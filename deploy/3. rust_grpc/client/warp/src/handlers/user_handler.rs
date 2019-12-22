use tonic;
use warp;

use crate::{
    models::{
        user::{
            UserSuccess,
            UserSuccessList,
        }
    },
    user::{
        user_service_client::UserServiceClient,
        UserReply, UserRequest,
        Empty,
        Users,
    },
    crypto::{
        hash::{
            hash_with_argon2,
        }
    },
};

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    let request = tonic::Request::new(Empty {});

    let response = client.unwrap().list_users(request).await
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
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };

    let Users { users } = &reply.into_inner();
    let user_success_list: Vec<UserSuccess> = users.iter().map(|x| {
        let UserReply {
            id: _,
            first_name,
            last_name,
            date_of_birth: _,
        } = x;

        let full_name: String = format!("{} {}", first_name, last_name);
        let hashed_full_name = hash_with_argon2(full_name.as_bytes());

        let user_success = UserSuccess {
            full_name: hashed_full_name,
        };
        user_success
    }).collect();

    // println!("{:#?}", &user_success_list);

    let user_success_list_json = UserSuccessList {
        users: user_success_list,
    };

    Ok(warp::reply::json(&user_success_list_json))
}

// Ignore error value for a while because of the typing problem
// https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
// Currently it prints error from gRPC server with println!("RESPONSE={:?}", response);
// Improve it when you become familiar with the Warp
pub async fn get(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    let request = tonic::Request::new(UserRequest {
        id
    });

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
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
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

    let user_success_json = UserSuccess {
        full_name: hashed_full_name,
    };

    Ok(warp::reply::json(&user_success_json))
}
