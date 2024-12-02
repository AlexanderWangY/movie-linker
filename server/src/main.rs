use core::panic;

use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    println!("[GET] hello_world()");
    "Hello world!"
}

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    if let Ok(addr) = listener.local_addr() {
        println!("Server running on http://{}", addr);
    } else {
        panic!("No address binded!");
    }

    axum::serve(listener, app).await.unwrap();
}
