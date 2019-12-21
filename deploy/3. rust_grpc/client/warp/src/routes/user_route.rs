use warp::Filter;

pub fn get() -> warp::filters::BoxedFilter<(String, )> {
    path!("api" / "user" / "v1")
        .and(warp::path::param::<String>())
        .boxed()
}

// It is equal to use it in main.
// let get_user = path!("api" / "user" / "v1")
//     .and(warp::path::param::<String>())