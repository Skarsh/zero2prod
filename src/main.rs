//! main.rs

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);

    println!("App running on {}", address);

    run(listener)?.await
}
