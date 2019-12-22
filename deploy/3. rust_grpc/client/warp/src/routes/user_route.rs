use warp::Filter;

// extract this part path!("api" / "user" / "v1")
// Just the path segment "/api/user/v1"
// let todos = path!("api" / "user" / "v1")

fn path_prefix() -> warp::filters::BoxedFilter<()> {
    path!("api" / "user" / "v1")
        .boxed()
}

pub fn list() -> warp::filters::BoxedFilter<()> {
    path_prefix()
        .and(warp::path::end())
        .boxed()
}

pub fn get() -> warp::filters::BoxedFilter<(String, )> {
    path_prefix()
        .and(warp::path::param::<String>())
        .boxed()
}

// pub fn list() -> warp::filters::BoxedFilter<()> {
//     path!("api" / "user" / "v1")
//         .and(warp::path::end())
//         .boxed()
// }

// pub fn get() -> warp::filters::BoxedFilter<(String, )> {
//     path!("api" / "user" / "v1")
//         .and(warp::path::param::<String>())
//         .boxed()
// }

// It is equal to use it in main.
// let get_user = path!("api" / "user" / "v1")
//     .and(warp::path::param::<String>())