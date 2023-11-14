use anyhow::Result;
use parrot_api::{env::Config, logger};

#[tokio::main]
pub async fn main() -> Result<()> {
    logger::init()?;
    let config = Config::new()?;
    Ok(())
}
