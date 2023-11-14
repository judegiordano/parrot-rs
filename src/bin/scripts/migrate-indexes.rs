use anyhow::Result;
use parrot_api::{logger, models::record::Record};

#[tokio::main]
pub async fn main() -> Result<()> {
    logger::init()?;
    let results = futures::try_join!(Record::migrate())?;
    tracing::info!("{:#?}", results);
    Ok(())
}
