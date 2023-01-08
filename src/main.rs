use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let listener = TcpListener::bind("127.0.0.1")?;
    run(listener)?.await
}