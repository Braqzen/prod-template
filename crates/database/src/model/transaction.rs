use crate::{
    Database,
    entities::{
        prelude::Transactions,
        transactions::{ActiveModel, Column, Model},
    },
};
use eyre::{Result, eyre};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder,
    QuerySelect,
};

impl Database {
    pub async fn create_transaction(
        &self,
        id: String,
        signer: String,
        raw_transaction: String,
        status: String,
    ) -> Result<()> {
        let transaction = ActiveModel {
            id: Set(id),
            signer: Set(signer),
            raw_transaction: Set(raw_transaction),
            status: Set(status),
            ..Default::default()
        };

        transaction.insert(&self.connection).await?;
        Ok(())
    }

    pub async fn unprocessed_transactions(&self) -> Result<Vec<Model>> {
        // Arbitrary limit of 1000
        let transactions = Transactions::find()
            .filter(Column::Status.eq("Received"))
            .order_by(Column::CreatedAt, Order::Asc)
            .limit(1000)
            .all(&self.connection)
            .await?;

        Ok(transactions)
    }

    // TODO: BUG - seems to update created_at
    pub async fn update_transaction_status(&self, id: String, status: String) -> Result<()> {
        let transaction = Transactions::find_by_id(id)
            .one(&self.connection)
            .await?
            .ok_or_else(|| eyre!("Transaction not found"))?;

        let mut active_model: ActiveModel = transaction.into();
        active_model.status = Set(status);
        // TODO: update time
        // active_model.updated_at = Set(OffsetDateTime::now_utc().into());
        active_model.update(&self.connection).await?;

        Ok(())
    }

    pub async fn transaction(&self, id: String) -> Result<Model> {
        let transaction = Transactions::find_by_id(id)
            .one(&self.connection)
            .await?
            .ok_or_else(|| eyre!("Transaction not found"))?;

        Ok(transaction)
    }
}
