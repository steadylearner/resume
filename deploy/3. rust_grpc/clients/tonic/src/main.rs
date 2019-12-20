// https://docs.rs/chrono/0.4.9/chrono/
extern crate argon2;
extern crate rand;

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

// async fn login(
//     credentials: User,
//     db: Arc<Mutex<HashMap<String, User>>>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let users = db.lock().await;
//     match users.get(&credentials.username) {
//         None => Ok(StatusCode::BAD_REQUEST),
//         Some(user) => {
//             if verify(&user.password, credentials.password.as_bytes()) {
//                 Ok(StatusCode::OK)
//             } else {
//                 Ok(StatusCode::UNAUTHORIZED)
//             }
//         }
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hyper is used here. Should prefix http://
    let mut client = UserServiceClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(UserRequest {
        // id: "It works!".into(),
        id: "steadylearner".into(),
    });

    let response = client.get_user(request).await?;

    println!("RESPONSE={:?}", response);

    let UserReply {
        id: _,
        first_name,
        last_name,
        date_of_birth: _,
    } = &response.into_inner();

    let full_name: String = format!("{}{}", first_name, last_name);

    println!("{}", hash(full_name.as_bytes()));

    Ok(())
}
