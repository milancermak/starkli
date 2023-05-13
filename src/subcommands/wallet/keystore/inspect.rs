use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use starknet::signers::SigningKey;

#[derive(Debug, Parser)]
pub struct Inspect {
    #[clap(help = "Path to the JSON keystore")]
    file: PathBuf,
}

impl Inspect {
    pub fn run(self) -> Result<()> {
        if !self.file.exists() {
            anyhow::bail!("keystore file not found");
        }

        let password = rpassword::prompt_password("Enter Password: ")?;

        let key = SigningKey::from_keystore(self.file, &password)?;
        println!("Public key: {:#064x}", key.verifying_key().scalar());

        Ok(())
    }
}
