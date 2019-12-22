use pretty_env_logger;
use warp::Filter;

use std::str;

use crate::{
    crypto::hash::verify_with_argon2, handlers::user_handler, models::user::UserSuccess,
    routes::user_route,
};

// $cargo test -- --nocapture if you want to use println! etc.

// I think that there are async problems when test various functions.
// When test a function and comment all others it passes.
// Is this the problem of this crate or tokio::test?
// failures:

// ---- tests::user::tests::get_user stdout ----
// thread 'tests::user::tests::get_user' panicked at 'called `Result::unwrap()` on an `Err` value: SetLoggerError(())', src/libcore/result.rs:1189:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

// Should read more documenation and ask for the author if necessary.

// or test just one function each time.
// For example, $cargo test list_users

// Tonic gRPC server should be ready first.(It is the client of tonic_server/src/service.rs)
// I expect tests similar to what used in this post.
// https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Refer to curl commands in main.rs
    #[tokio::test]
    async fn list_users() {
        let _ = pretty_env_logger::init();

        let list_users = user_route::list()
            .and_then(user_handler::list);

        let res = warp::test::request()
            .method("GET")
            .path("/api/user/v1") // 1. Define path with datas
            .reply(&list_users) // 2. Use the exact payload you want to test and reply can be target, responder, reply_with.
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK.");
        println!("{:#?}", res.body());
    }

    // Error handling with this crate is not easy for me currently.

    // 1. get_user in database
    //      -> Should return 200 OK
    //      -> The full_name made from its first_name and last name should be equal to the hashed value
    //         when verified with verify_with_argon2
    // 2. get_user not in database
    //      -> fail with 404 'Not Found'(This is what I expect)
    // (When I use CURL or Browser, it returns 405 'HTTP method not allowed')
    //      -> Should solve this problem with the author of the crate or there is a problem with my code?

    #[tokio::test]
    async fn get_user() {
        let _ = pretty_env_logger::init();

        // id: "steadylearner",
        // first_name: "steady",
        // last_name: "learner",
        // full_name: "steady learner"

        let get_user = user_route::get()
            .and_then(user_handler::get);

        // 1.
        let res = warp::test::request()
            .method("GET")
            .path("/api/user/v1/steadylearner")
            .reply(&get_user)
            .await;

        assert_eq!(res.status(), 200, "Should return 200 'OK'");
        let body_str_from_byte = str::from_utf8(&res.body()).unwrap(); // res.body here is b"" utf8 bytes
        let UserSuccess { full_name } = serde_json::from_str(&body_str_from_byte).unwrap();

        assert!(verify_with_argon2(&full_name, "steady learner".as_bytes()));

        // 2.
        let res = warp::test::request()
            .method("GET")
            .path("/api/user/v1/d631208e-57ce-4838-9368-db401519ebb8")
            .reply(&get_user)
            .await;

        assert_eq!(res.status(), 404, "Should return 404 'Not Found'");
    }

    // The main tests here are 5 and 6.

    // 1. Delete iser without method "DELETE"
    //      -> fail with ""
    // 2. Delete user without header
    //      -> fail with 'Missing request header 'authorization''
    // 3. Delete user with incorrect header key for example 'without-authorizaiton'
    //      -> fail with 'Missing request header 'authorization''
    // 4. Delete user without correct header value such as 'not-allowed'
    //      -> fail with 'Invalid request header 'authorization''
    // 5. Delete user with correct header and when there is already data in database.
    //      -> success with 'Remove the user with id f2bd8139-5044-4526-89b8-1981d6220b4.'
    // 6. Delete user with correct header but no data in database
    //      -> fail with 'Fail to delete the user with id f2bd8139-5044-4526-89b8-1981d6220b4.' from gRPC server

    #[tokio::test]
    async fn delete_user() {
        let _ = pretty_env_logger::init();

        let delete_user = user_route::delete()
            .and_then(user_handler::delete);

        // You should verify this exist in your database.
        // For example, $SELECT * FROM users;
        // Or refer to list_users api. But it will be more complicated than this.
        let target = "898067d8-5787-4939-bb3e-20025ae88d4e";
        // If it doesn't exist, it will fail at 5.
        // else all tests will pass.

        // 1.
        let res = warp::test::request()
            .header("authorization", "steadylearner")
            .path(&format!("/api/user/v1/{}", &target))
            .reply(&delete_user)
            .await;

        assert_eq!(res.status(), 405, "Should return 405 'HTTP method not allowed.'");
        let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
        assert_eq!(body_str_from_byte, "HTTP method not allowed");

        // 2.
        let res = warp::test::request()
            .method("DELETE")
            .path(&format!("/api/user/v1/{}", &target))
            .reply(&delete_user)
            .await;

        assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
        let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
        assert_eq!(body_str_from_byte, "Missing request header 'authorization'");

        // 3.
        let res = warp::test::request()
            .method("DELETE")
            .header("without-authorization", "steadylearner")
            .path(&format!("/api/user/v1/{}", &target))
            .reply(&delete_user)
            .await;

        assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
        let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
        assert_eq!(body_str_from_byte, "Missing request header 'authorization'");

        // 4.
        let res = warp::test::request()
            .method("DELETE")
            .header("authorization", "not-allowed")
            .path(&format!("/api/user/v1/{}", &target))
            .reply(&delete_user)
            .await;

        assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
        let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
        assert_eq!(body_str_from_byte, "Invalid request header 'authorization'");

        // 5.
        let res = warp::test::request()
            .method("DELETE")
            .header("authorization", "steadylearner")
            .path(&format!("/api/user/v1/{}", &target))
            .reply(&delete_user)
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK.");
        let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
        assert_eq!(&body_str_from_byte, &format!("Remove the user with id {}.", &target));

        // 6.
        let res = warp::test::request()
            .method("DELETE")
            .header("authorization", "steadylearner")
            .path(&format!("/api/user/v1/{}", &target))
            .reply(&delete_user)
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK.");
        let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
        assert_eq!(&body_str_from_byte, &format!("Fail to delete the user with id {}.", &target));

        // No records in Postgresql when 6. passes.
        // \c grpc;
        // $SELECT * FROM users;
    }
}
