extern crate pretty_env_logger;
#[macro_use] extern crate log;

extern crate tonic;
mod user {
    tonic::include_proto!("user");
}

extern crate argon2;
extern crate rand;

extern crate serde;
extern crate serde_json;

// cargo doc -p warp --open
#[macro_use]
extern crate warp;
use warp::{
    Filter
};

extern crate console;
use console::Style;

// This is where all your custom modules(folders) will be.
mod crypto;
mod models;
// Files in handlers/ are what implements "Result<impl warp::Reply, warp::Rejection>"
// It will be similar to controllers in Express.
mod routes;
mod handlers;
use self::{
    routes::{
        user_route,
    },
    handlers::{
        user_handler
    },

};

// It will only work with $cargo test
#[cfg(test)] mod tests;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // curl 0.0.0.0:8000/api/user/v1/steadylearner
    let get_user = user_route::get()
        .and_then(user_handler::get);

    let api = get_user;
    // The complete form should be this with tests
    // let api = list_users.or(get_user).or(create_user).or(update_user).or(delete_user);

    let routes = api.with(warp::log("api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// 1. Read the documentation more.
// 2. More error handling and make CRUD Rest API with tests.
// 3. Read the documentation more.