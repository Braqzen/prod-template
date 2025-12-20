use crate::api::{
    middleware::{JsonValidationLayer, MethodValidationLayer},
    not_found, request,
};
use axum::{Router, routing::post, serve};
use database::Database;
use eyre::Result;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing::info;

pub struct Server {
    socket: SocketAddr,
    database: Arc<Database>,
}

#[derive(Clone)]
pub struct State {
    pub database: Arc<Database>,
}

impl State {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

impl Server {
    pub async fn new(socket: SocketAddr, database_url: &str) -> Result<Self> {
        let database = Arc::new(Database::new(database_url).await?);
        Ok(Self { socket, database })
    }

    pub async fn run(self) -> Result<()> {
        let state = Arc::new(State::new(self.database.clone()));
        let listener = TcpListener::bind(self.socket).await?;

        let middleware = ServiceBuilder::new()
            .layer(JsonValidationLayer)
            .layer(MethodValidationLayer);

        let app = Router::new()
            .route("/", post(request))
            .fallback(not_found)
            .with_state(state.clone())
            .layer(middleware);

        info!(socket = self.socket.to_string(), "Starting router");

        serve(listener, app).await?;

        Ok(())
    }
}
