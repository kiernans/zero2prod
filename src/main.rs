use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if can't read config
    let configuration = get_configuration().expect("Failed to read configuration.yaml");
    // Get port from configuration
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
