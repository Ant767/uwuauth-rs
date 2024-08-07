use register::register_endpoint;

mod register;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = register_endpoint();

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}