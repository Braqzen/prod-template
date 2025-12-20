mod cli;
mod environment;
mod worker;

use crate::{cli::Cli, worker::Worker};
use dotenvy::dotenv;
use environment::Environment;
use eyre::Result;
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Check for any CLI arguments to prioritize
    let args = match Cli::parse() {
        Ok(args) => args,
        Err(error) => {
            error!(%error, "Failed to parse CLI arguments");
            return Err(error);
        }
    };

    // Load environment variables from .env file
    dotenv().ok();

    // Parse environment variables and prioritize cli
    let environment = match Environment::new(&args) {
        Ok(environment) => environment,
        Err(error) => {
            error!(%error, "Failed to parse environment variables");
            return Err(error);
        }
    };

    let worker = match Worker::new(&environment.database_url).await {
        Ok(worker) => worker,
        Err(error) => {
            error!(%error, "Failed to create worker");
            return Err(error);
        }
    };

    match worker.run().await {
        Ok(()) => Ok(()),
        Err(error) => {
            error!(%error, "Worker crashed");
            return Err(error);
        }
    }
}
