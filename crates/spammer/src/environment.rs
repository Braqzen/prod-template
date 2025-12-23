use {
    crate::cli::Cli,
    eyre::{Result, eyre},
    std::env::var,
};

pub struct Environment {
    pub private_keys: String,
    pub rpc: String,
}

impl Environment {
    pub fn new(cli: &Cli) -> Result<Self> {
        let private_keys = cli
            .private_keys
            .clone()
            .or_else(|| var("PRIVATE_KEYS").ok())
            .ok_or_else(|| eyre!("PRIVATE_KEYS must be set"))?;
        let rpc = cli
            .rpc
            .clone()
            .or_else(|| var("RPC").ok())
            .ok_or_else(|| eyre!("RPC must be set"))?;

        Ok(Self { private_keys, rpc })
    }
}
