use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port.");
    run(listener)?.await
}
