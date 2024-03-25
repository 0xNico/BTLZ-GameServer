use actix_web::{web, HttpResponse};
use serde::Serialize;
use crate::AppState;
use std::sync::Arc;
use std::time::Instant;

// Updated response structure to include response time
#[derive(Serialize)]
struct RpcConnectionResponse {
    connected: bool,
    message: String,
    response_time_ms: u128, // Response time in milliseconds
}

// Function to test RPC connection and report back with response time
pub async fn test_rpc_connection(app_state: web::Data<AppState>) -> HttpResponse {
    let rpc_client = app_state.solana_client.clone();

    // Start measuring time
    let start = Instant::now();

    // Use web::block to offload the blocking operation
    let result = web::block(move || rpc_client.get_version()).await;

    // Stop measuring time
    let duration = start.elapsed();

    match result {
        Ok(version_info) => HttpResponse::Ok().json(RpcConnectionResponse {
            connected: true,
            message: format!("Connected to Solana DevNet. Version: {:?}", version_info),
            response_time_ms: duration.as_millis(), // Report response time
        }),
        Err(e) => HttpResponse::InternalServerError().json(RpcConnectionResponse {
            connected: false,
            message: format!("Failed to connect to Solana DevNet. Error: {:?}", e),
            response_time_ms: duration.as_millis(), // Report response time even in case of failure
        }),
    }
}
