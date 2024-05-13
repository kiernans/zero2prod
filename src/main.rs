use std::net::TcpListener;

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:0")?;
    run(listener)?.await
}
