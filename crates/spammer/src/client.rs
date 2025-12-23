use {
    crate::method::Method,
    eyre::{Result, eyre},
    reqwest::Client as ReqwestClient,
    serde_json::json,
};

pub struct Client {
    client: ReqwestClient,
    rpc: String,
}

impl Client {
    pub fn new(rpc: &str) -> Self {
        Self {
            client: ReqwestClient::new(),
            rpc: rpc.to_string(),
        }
    }

    pub async fn send_tx(&self, tx_hex: &str, method: &Method) -> Result<String> {
        let body = match method {
            Method::Raw => json!({
                "jsonrpc": "2.0",
                "method": "eth_sendRawTransaction",
                "params": [tx_hex],
                "id": 1
            }),
            Method::Bundle => json!({
                "jsonrpc": "2.0",
                "method": "eth_sendBundle",
                "params": [{
                    "txs": [tx_hex],
                }],
                "id": 1
            }),
            Method::Private => json!({
                "jsonrpc": "2.0",
                "method": "eth_sendPrivateTransaction",
                "params": [tx_hex],
                "id": 1
            }),
        };

        let response = self.client.post(&self.rpc).json(&body).send().await;

        match response {
            Ok(response) => {
                let status = response.status();
                let body = response.text().await?;
                if !status.is_success() {
                    return Err(eyre!("{status}: {body}"));
                }
                Ok(body)
            }
            Err(error) => Err(eyre!("Failed to send transaction: {error}")),
        }
    }
}
