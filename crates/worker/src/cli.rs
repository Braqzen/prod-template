use eyre::{Result, eyre};

pub struct Cli {
    pub database_url: Option<String>,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        // Skip binary name
        let mut args = std::env::args().skip(1);

        let mut database_url: Option<String> = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--database-url" | "-d" => {
                    if let Some(val) = args.next() {
                        database_url = Some(val);
                    } else {
                        return Err(eyre!("--database-url requires a value"));
                    }
                }
                _ => {}
            }
        }

        Ok(Self { database_url })
    }
}
