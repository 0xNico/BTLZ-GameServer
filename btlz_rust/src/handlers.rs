use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::zai_functions::create_player;
use crate::AppState;
use std::sync::Arc;

// Assuming you have this struct to parse the request body
#[derive(Deserialize)]
struct CreatePlayerRequest {
    signer_pubkey: String, // In practice, you might accept a Base58 string
    active_class: u64,
    active_weapon: u64,
}

// Response structure for simplicity
#[derive(Serialize)]
struct CreatePlayerResponse {
    transaction: String, // Simplified for demonstration; adjust as needed
}

// The handler function
pub async fn create_player_handler(
    app_state: web::Data<Arc<AppState>>,
    req: web::Json<CreatePlayerRequest>,
) -> HttpResponse {
    let signer_pubkey = req.signer_pubkey.parse().expect("Invalid Pubkey"); // Handle errors properly in real code

    match create_player(app_state.into_inner(), signer_pubkey, req.active_class, req.active_weapon).await {
        Ok(transaction) => {
            // For the purpose of this example, we're simplifying the transaction representation.
            // You would serialize it appropriately depending on your needs.
            HttpResponse::Ok().json(CreatePlayerResponse { transaction: format!("{:?}", transaction) })
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
