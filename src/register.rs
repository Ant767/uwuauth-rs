use warp::Filter;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct RegisterRequest {
    username: String,
    password: String,
    handle: String,
    email: String,
}

pub fn register_endpoint() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let route = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .map(|data: RegisterRequest| {
            format!("User {} registered!", data.username)
        })
        .boxed();
    return route;
}