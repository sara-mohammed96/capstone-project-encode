use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
};

use axum::{routing::get, Router, response::IntoResponse, Json};
use tower_http::cors::{CorsLayer, AllowMethods, AllowHeaders, Any};
use axum::http::{Method, header::HeaderName};  // Import Method and HeaderName from axum's http module
use serde::Serialize;

const RPC_URL: &str = "http://127.0.0.1:8899";

#[tokio::main]
async fn main() {
     // Define CORS settings
    let cors = CorsLayer::new()
        .allow_origin(Any)  // Allow all origins (you can restrict this to your frontend's origin)
        .allow_methods(AllowMethods::from(vec![Method::GET]))  // Allow only GET requests
        .allow_headers(AllowHeaders::from(vec![HeaderName::from_static("content-type")]));  // Allow only "Content-Type" header 

    let app = Router::new()
        .route("/validators", get(get_validators))
        .layer(cors);

    println!("Server running on port: http://127.0.0.1:3030");

    axum::Server::bind(&"127.0.0.1:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Debug)]
struct Validator {
    id: usize,
    name: String,
    uptime: String,
    fees: String,
    location: String,
}

async fn get_validators() -> impl IntoResponse {
    // Simulate fetching validators from Solana or any data source
    match fetch_validators() {
        Ok(validators) => {
            println!("Fetched Validators: {:?}", validators);

            Json(validators).into_response()
        }
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", err),
        )
            .into_response(),
    }
}

fn fetch_validators() -> Result<Vec<Validator>, Box<dyn std::error::Error>> {
    // Solana RPC client
    let rpc_client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());

    // Fetch vote accounts
    let vote_accounts = rpc_client.get_vote_accounts()?.current;

    // Map vote accounts to Validator struct
    let validators: Vec<Validator> = vote_accounts
        .iter()
        .enumerate()
        .map(|(index, vote_account)| Validator {
            id: index + 1,
            name: format!("Validator {}", vote_account.vote_pubkey),
            uptime: format!("{:.1}%", vote_account.epoch_vote_account),
            fees: format!("{}%", vote_account.commission),
            location: "Unknown".to_string(), // Placeholder
        })
        .collect();

    Ok(validators)
}