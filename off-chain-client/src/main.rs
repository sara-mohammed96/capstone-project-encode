use solana_client::rpc_client::RpcClient;
use solana_client::rpc_response::RpcVoteAccountInfo;
use solana_sdk::commitment_config::CommitmentConfig;
use std::cmp::Reverse;
use axum::{routing::get, Router, response::IntoResponse, Json};
use tower_http::cors::{CorsLayer, AllowMethods, AllowHeaders, Any};
use axum::http::{Method, header::HeaderName};  // Import Method and HeaderName from axum's http module
use serde::Serialize;

const RPC_URL: &str = "https://api.devnet.solana.com"; // Devnet RPC URL

// Custom type extending RpcVoteAccountInfo
#[derive(Serialize, Debug)]
struct RpcVoteAccountInfoExtended {
    vote_account_info: RpcVoteAccountInfo,
    root_distance: u64,
    vote_distance: u64,
    points: u64,
}

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

fn fetch_validators() -> Result<Vec<RpcVoteAccountInfoExtended>, Box<dyn std::error::Error>> {
    
    let rpc_client =
        RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::finalized());

//    let mock = RpcVoteAccountInfo {
//        vote_pubkey: String::from("testvotepubkey"),
//        node_pubkey: String::from("testnodepubkey"),
//        activated_stake: 1000,
//        commission: 4,
//        epoch_vote_account: true,
//        epoch_credits: vec![(10, 10, 10)],
//        last_vote: 347898301,
//        root_slot: 347898300,
//    };

    let current_vote_accounts = rpc_client.get_vote_accounts()?.current;
//    current_vote_accounts.push(mock);

    let latest_slot = rpc_client.get_slot()?;

    let mut vote_accounts: Vec<RpcVoteAccountInfoExtended> = Vec::new();

    for item in current_vote_accounts.iter() {
        let vote_account_info_with_distance = RpcVoteAccountInfoExtended {
            vote_account_info: item.clone(),
            root_distance: latest_slot - item.root_slot,
            vote_distance: latest_slot - item.last_vote,
            points: 0,
        };

        vote_accounts.push(vote_account_info_with_distance);
    }

    let root_distance_median =
        vote_account_root_distance_median(&mut vote_accounts).unwrap();
    let vote_distance_median =
        vote_account_vote_distance_median(&mut vote_accounts).unwrap();
    let root_distance_average =
        vote_account_root_distance_average(&mut vote_accounts).unwrap();
    let vote_distance_average =
        vote_account_vote_distance_average(&mut vote_accounts).unwrap();

    for item in vote_accounts.iter_mut() {
        let root_median_performance = root_distance_median - item.root_distance as f64;

        if root_median_performance >= root_distance_median / 2.0 {
            item.points += 2;
        } else if root_median_performance < root_distance_median / 2.0 && root_median_performance >= 0.0 {
            item.points += 1;
        }

        let vote_median_performance = vote_distance_median - item.vote_distance as f64;

        if vote_median_performance >= vote_distance_median / 2.0 {
            item.points += 2;
        } else if vote_median_performance < vote_distance_median / 2.0 && vote_median_performance >= 0.0 {
            item.points += 1;
        }

        let root_average_performance = root_distance_average - item.root_distance as f64;

        if root_average_performance >= root_distance_average / 2.0 {
            item.points += 2;
        } else if root_average_performance < root_distance_average / 2.0 && root_average_performance >= 0.0 {
            item.points += 1;
        }

        let vote_average_performance = vote_distance_average - item.vote_distance as f64;

        if vote_average_performance >= vote_distance_average / 2.0 {
            item.points += 2;
        } else if vote_average_performance < vote_distance_average / 2.0 && vote_average_performance >= 0.0 {
            item.points += 1;
        }

    }

    vote_accounts.sort_by_key(|item| Reverse(item.points));

    let mut top_vote_accounts: Vec<RpcVoteAccountInfoExtended> = Vec::new();
    let max_points: u64 = vote_accounts[0].points;

    for item in vote_accounts.into_iter() {
        if item.points == max_points {
            top_vote_accounts.push(item);
            continue;
        } else {
            break;
        }
    }

    top_vote_accounts.sort_by_key(|item| item.vote_account_info.activated_stake);

    for item in top_vote_accounts.iter() {
        println!("Vote Pubkey: {:?} \t Node Pubkey: {:?} \t Activated Stake: {:?} \t Comission: {:?} \t Epoch Vote Account: {:?} \t Epoch Credits: {:?} \t Last Vote: {:?} \t Root Slot: {:?} \t Root Distance: {:?} \t Vote Distance: {:?} \t Points: {:?}", item.vote_account_info.vote_pubkey, item.vote_account_info.node_pubkey, item.vote_account_info.activated_stake, item.vote_account_info.commission, item.vote_account_info.epoch_vote_account, item.vote_account_info.epoch_credits, item.vote_account_info.last_vote, item.vote_account_info.root_slot, item.root_distance, item.vote_distance, item.points);
    }

    println!("Root median: {:?}", root_distance_median);
    println!("Root average: {:?}", root_distance_average);
    println!("Vote median: {:?}", vote_distance_median);
    println!("Vote average: {:?}", vote_distance_average);

    Ok(top_vote_accounts)
}

//#[derive(Serialize, Debug)]
//struct Validator {
//    id: usize,
//    name: String,
//    uptime: String,
//    fees: String,
//    location: String,
//}

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

//fn fetch_validators() -> Result<Vec<Validator>, Box<dyn std::error::Error>> {
//    // Solana RPC client
//    let rpc_client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());
//
//    // Fetch vote accounts
//    let vote_accounts = rpc_client.get_vote_accounts()?.current;
//
//    // Map vote accounts to Validator struct
//    let validators: Vec<Validator> = vote_accounts
//        .iter()
//        .enumerate()
//        .map(|(index, vote_account)| Validator {
//            id: index + 1,
//            name: format!("Validator {}", vote_account.vote_pubkey),
//            uptime: format!("{:.1}%", vote_account.epoch_vote_account),
//            fees: format!("{}%", vote_account.commission),
//            location: "Unknown".to_string(), // Placeholder
//        })
//        .collect();
//
//    Ok(validators)
//}

fn vote_account_root_distance_median(values: &mut Vec<RpcVoteAccountInfoExtended>) -> Option<f64> {
    // Return None if the list is empty
    if values.is_empty() {
        return None;
    }

    // Sort the values by root_distance
    values.sort_by_key(|item| item.root_distance);

    let len = values.len();
    let mid = len / 2;

    if len % 2 == 0 {
        // Even-length: calculate the average of the two middle values
        Some((values[mid - 1].root_distance as f64 + values[mid].root_distance as f64) / 2.0)
    } else {
        // Odd-length: return the middle value
        Some(values[mid].root_distance as f64)
    }
}

fn vote_account_vote_distance_median(values: &mut Vec<RpcVoteAccountInfoExtended>) -> Option<f64> {
    // Return None if the list is empty
    if values.is_empty() {
        return None;
    }

    // Sort the values by vote_distance
    values.sort_by_key(|item| item.vote_distance);

    let len = values.len();
    let mid = len / 2;

    if len % 2 == 0 {
        // Even-length: calculate the average of the two middle values
        Some((values[mid - 1].vote_distance as f64 + values[mid].vote_distance as f64) / 2.0)
    } else {
        // Odd-length: return the middle value
        Some(values[mid].vote_distance as f64)
    }
}

fn vote_account_root_distance_average(values: &mut Vec<RpcVoteAccountInfoExtended>) -> Option<f64> {
    // Return None if the list is empty
    if values.is_empty() {
        return None;
    }

    let mut sum = 0.0;
    let length = values.len() as f64;
    println!("Length in root distance fn: {:?}", length);

    for item in values.iter() {
        sum += item.root_distance as f64;
    }
    println!("Sum in root distance fn: {:?}", sum);

    Some(sum/length)
}

fn vote_account_vote_distance_average(values: &mut Vec<RpcVoteAccountInfoExtended>) -> Option<f64> {
    // Return None if the list is empty
    if values.is_empty() {
        return None;
    }

    let mut sum = 0.0;
    let length = values.len() as f64;
    println!("Length in vote distance fn: {:?}", length);

    for item in values.iter() {
        sum += item.vote_distance as f64;
    }
    println!("Sum in vote distance fn: {:?}", sum);

    Some(sum/length)
}
