mod entities;
mod model;

pub use entities::transactions::Model as TransactionModel;
use eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database as SeaDatabase, DatabaseConnection};
use sqlx::postgres::PgListener;

const NOTIFY_CHANNEL: &str = "transaction";

pub struct Database {
    url: String,
    connection: DatabaseConnection,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self> {
        let connection = SeaDatabase::connect(url).await?;

        // Automatically apply all migrations
        Migrator::up(&connection, None).await?;

        Ok(Self {
            connection,
            url: url.to_string(),
        })
    }

    pub async fn notify(&self) -> Result<()> {
        self.connection
            .execute_unprepared(&format!("NOTIFY {};", NOTIFY_CHANNEL))
            .await?;
        Ok(())
    }

    pub async fn listener(&self) -> Result<PgListener> {
        let mut listener = PgListener::connect(&self.url).await?;
        listener.listen(NOTIFY_CHANNEL).await?;

        Ok(listener)
    }
}
