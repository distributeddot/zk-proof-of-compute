mod zkvm;
mod verifier;
mod coordinator;
mod incentives;
mod server;

#[tokio::main]
async fn main() {
    println!("Starting zk-powered proof-of-compute chain emulator...");
    server::run().await;
}