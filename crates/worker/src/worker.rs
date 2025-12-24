use database::{Database, TransactionModel};
use eyre::Result;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

pub struct Worker {
    database: Database,
}

impl Worker {
    pub async fn new(database_url: &str) -> Result<Self> {
        let database = Database::new(database_url).await?;
        Ok(Self { database })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting worker");

        let mut listener = self.database.listener().await?;

        loop {
            // Wait for notification or 1 second fallback
            tokio::select! {
                _ = listener.recv() => {}
                _ = sleep(Duration::from_secs(1)) => {}
            }

            // Query DB for requests
            let transactions = match self.database.unprocessed_transactions().await {
                Ok(transactions) => transactions,
                Err(error) => {
                    warn!(%error, "Error getting unprocessed transactions");
                    continue;
                }
            };

            if transactions.is_empty() {
                continue;
            }

            // Process transactions
            for transaction in transactions {
                info!(
                    hash = transaction.id,
                    signer = transaction.signer,
                    "Processing transaction"
                );
                match self.process_transaction(transaction.clone()).await {
                    Ok(status) => {
                        info!(
                            hash = transaction.id,
                            signer = transaction.signer,
                            status,
                            "Transaction processed"
                        );
                    }
                    Err(error) => {
                        warn!(
                            %error,
                            hash = transaction.id,
                            signer = transaction.signer,
                            "Error processing transaction"
                        );
                    }
                }
            }
        }
    }

    async fn process_transaction(&self, transaction: TransactionModel) -> Result<String> {
        let status = match rand::rng().random_range(0..10) < 9 {
            true => "Success",
            false => "Failed",
        };
        match self
            .database
            .update_transaction_status(transaction.id.clone(), status.into())
            .await
        {
            Ok(()) => Ok(status.to_string()),
            Err(error) => Err(error),
        }
    }
}
