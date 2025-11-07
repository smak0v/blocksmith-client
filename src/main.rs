mod transaction;
mod wallet;

use rand::seq::IteratorRandom;
use reqwest::Client;
use tokio::time;

use std::env;
use std::time::Duration;

use crate::transaction::{Transaction, TransactionRequest};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let wallets = wallet::generate_wallets(100);
    let nodes: Vec<String> = env::var("NODES")
        .expect("NODES environmental variable is not set")
        .split(',')
        .filter_map(|s| {
            if !s.trim().is_empty() {
                return Some(s.to_owned());
            }

            None
        })
        .collect();
    let client = Client::new();
    let mut rng = rand::rng();
    let mut nonce = 0;

    if nodes.is_empty() {
        eprintln!("Nodes list is empty");

        return;
    }

    loop {
        let random_wallet_1 = &wallets[(0..wallets.len() / 2).choose(&mut rng).unwrap()];
        let random_wallet_2 =
            &wallets[(wallets.len() / 2..wallets.len()).choose(&mut rng).unwrap()];
        let mut transaction = Transaction::new(
            random_wallet_1.public_key().to_string(),
            random_wallet_2.public_key().to_string(),
            (1..=100).choose(&mut rng).unwrap(),
            "Example transaction",
            nonce,
        );

        transaction.sign(random_wallet_1.secret_key());

        let request = TransactionRequest {
            transaction,
            public_key: random_wallet_1.public_key().to_string(),
        };
        let result = client
            .post(format!(
                "{}/transaction",
                nodes[(0..nodes.len()).choose(&mut rng).unwrap()]
            ))
            .json(&request)
            .send()
            .await;

        match result {
            Err(error) => eprintln!("Failed to send request: {:?}", error),
            Ok(response) => {
                println!("Request sent successfully!");
                println!("Request: {:#?}", request);
                println!("Response: {:#?}\n", response.text().await);
            }
        }

        nonce += 1;

        time::sleep(Duration::from_secs(5)).await;
    }
}
