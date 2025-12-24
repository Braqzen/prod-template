mod cli;
mod client;
mod config;
mod environment;
mod method;

use {
    alloy::{
        eips::eip2718::Encodable2718,
        network::{EthereumWallet, TransactionBuilder},
        primitives::{Address, Bytes, TxKind, U256, hex},
        rpc::types::{TransactionInput, TransactionRequest},
    },
    cli::Cli,
    client::Client,
    config::Config,
    dotenvy::dotenv,
    environment::Environment,
    eyre::{Result, eyre},
    method::Method,
    rand::Rng,
    std::time::Duration,
    tokio::time::sleep,
    tracing::{error, info},
    tracing_subscriber::EnvFilter,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Load arguments if any are provided
    let cli = match Cli::parse() {
        Ok(cli) => cli,
        Err(error) => {
            error!(%error, "Failed to parse CLI arguments");
            return Err(error);
        }
    };

    // Load environment variables from .env file
    dotenv().ok();

    // Load environment variables
    let environment = match Environment::new(&cli) {
        Ok(environment) => environment,
        Err(error) => {
            error!(%error, "Failed to parse environment variables");
            return Err(error);
        }
    };

    // Parse environment variables
    let config = match Config::new(&environment).await {
        Ok(config) => config,
        Err(error) => {
            error!(%error, "Failed to parse config");
            return Err(error);
        }
    };

    // Create client to send requests to the RPC endpoint
    let client = Client::new(&environment.rpc);

    info!("Starting spammer");

    loop {
        // Pick a random signer
        let index = rand::rng().random_range(0..config.signers.len());

        // Create transaction
        let transaction = match create_transaction(&config.signers[index]).await {
            Ok(transaction) => transaction,
            Err(error) => {
                error!(%error, "Failed to create transaction");
                continue;
            }
        };

        // Pick a random method
        let method = rand::rng().random_range(0..100);
        let method = match method {
            0..=74 => Method::Raw,      // 75%
            75..=89 => Method::Bundle,  // 15%
            90..=99 => Method::Private, // 10%
            _ => unreachable!(),
        };

        info!(transaction, method = method.as_str(), "Sending transaction");

        match client.send_tx(&transaction, &method).await {
            Ok(response) => info!(response, "Sent transaction"),
            Err(error) => error!(%error, "Failed to send transaction"),
        };

        // Random delay to prevent real spam and simulate submission
        sleep(Duration::from_millis(rand::rng().random_range(100..1000))).await;
    }
}

/// Creates transaction to send to the RPC endpoint
async fn create_transaction(signer: &EthereumWallet) -> Result<String> {
    let input = {
        let random_data = rand::rng()
            .sample_iter(&rand::distr::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>()
            .into_bytes();
        let data = Bytes::from(random_data);

        TransactionInput {
            data: Some(data.clone()),
            input: Some(data),
        }
    };

    // Transactions aren't meant to be valid, only parseable in rpc/worker
    let tx = TransactionRequest {
        nonce: Some(0),
        value: Some(U256::from(0)),
        gas: Some(25_000),
        to: Some(TxKind::Call(Address::ZERO)),
        max_fee_per_gas: Some(7e9 as u128),
        max_priority_fee_per_gas: Some(7e9 as u128),
        chain_id: Some(1),
        transaction_type: Some(2),
        input,
        ..Default::default()
    };

    let signed = tx
        .build(&signer)
        .await
        .map_err(|e| eyre!("Failed to sign transaction: {e}"))?;

    let mut encoded = Vec::new();
    Encodable2718::encode_2718(&signed, &mut encoded);
    let tx_hex = format!("0x{}", hex::encode(&encoded));

    Ok(tx_hex)
}
