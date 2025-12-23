use {
    crate::environment::Environment,
    alloy::{network::EthereumWallet, signers::local::PrivateKeySigner},
    eyre::Result,
    std::str::FromStr,
};

pub struct Config {
    pub signers: Vec<EthereumWallet>,
}

impl Config {
    pub async fn new(environment: &Environment) -> Result<Self> {
        let signers = environment
            .private_keys
            .split(',')
            .map(|key| PrivateKeySigner::from_str(key).expect("Failed to parse private key"))
            .map(|signer| EthereumWallet::new(signer))
            .collect::<Vec<_>>();

        Ok(Self { signers })
    }
}
