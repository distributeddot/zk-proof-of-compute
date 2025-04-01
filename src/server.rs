use actix_web::{post, web, App, HttpServer, Responder};
use crate::{zkvm, verifier, coordinator, incentives};

#[post("/prove")]
async fn prove(info: web::Json<ProveRequest>) -> impl Responder {
    let input = coordinator::next_task();
    let proof = zkvm::prove_computation(input);
    let valid = verifier::verify_proof(&proof);

    if valid {
        incentives::reward_prover(&info.prover_id, 100);
        format!("Proof accepted for task {}", input)
    } else {
        format!("Invalid proof for task {}", input)
    }
}

#[derive(serde::Deserialize)]
pub struct ProveRequest {
    pub prover_id: String,
}

pub async fn run() {
    HttpServer::new(|| App::new().service(prove))
        .bind("127.0.0.1:8080")
        .expect("Failed to bind server")
        .run()
        .await
        .unwrap();
}