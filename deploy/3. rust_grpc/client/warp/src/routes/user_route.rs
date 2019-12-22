use warp::Filter;

// extract this part path!("api" / "user" / "v1")
// Just the path segment "/api/user/v1"
// let todos = path!("api" / "user" / "v1")

fn path_prefix() -> warp::filters::BoxedFilter<()> {
    path!("api" / "user" / "v1")
        .boxed()
}

pub fn list() -> warp::filters::BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}

// It is equal to use it in main.
// let get_user = path!("api" / "user" / "v1")
//     .and(warp::path::param::<String>())
pub fn get() -> warp::filters::BoxedFilter<(String, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .boxed()
}

pub fn delete() -> warp::filters::BoxedFilter<(String, )> {
    // Move it to function later if necessary.
    // pub fn exact(
    //     name: &'static str,
    //     value: &'static str
    // ) -> impl Filter<Extract = (), Error = Rejection> + Copy
    let admin_only = warp::header::exact("authorization", "steadylearner");

    warp::delete()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .and(admin_only.clone())
        .boxed()
}