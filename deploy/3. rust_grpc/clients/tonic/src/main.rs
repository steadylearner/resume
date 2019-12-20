// https://docs.rs/chrono/0.4.9/chrono/

pub mod user {
    tonic::include_proto!("user");
}

use user::{user_service_client::UserServiceClient, UserRequest};

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
    let user_date_of_birth = &response.into_inner().date_of_birth;
    println!("{}", user_date_of_birth);

    Ok(())
}


