use pasm::server;

/// Main function that runs pasm as a server
#[tokio::main]
pub async fn main() {
    server::run().await;
}
