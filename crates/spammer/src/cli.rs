use eyre::{Result, eyre};

pub struct Cli {
    pub private_keys: Option<String>,
    pub rpc: Option<String>,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        // Skip binary name
        let mut args = std::env::args().skip(1);

        let mut private_keys: Option<String> = None;
        let mut rpc: Option<String> = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--private-keys" | "-pks" => {
                    if let Some(val) = args.next() {
                        private_keys = Some(val);
                    } else {
                        return Err(eyre!("--private-keys requires a value"));
                    }
                }
                "--rpc" | "-r" => {
                    if let Some(val) = args.next() {
                        rpc = Some(val);
                    } else {
                        return Err(eyre!("--rpc requires a value"));
                    }
                }
                _ => {}
            }
        }

        Ok(Self { private_keys, rpc })
    }
}
