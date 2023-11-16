use anyhow::Result;
use parrot_api::{logger, models::voice::Voice};

#[tokio::main]
pub async fn main() -> Result<()> {
    logger::init()?;
    let results = futures::try_join!(Voice::migrate())?;
    tracing::info!("{:#?}", results);
    Ok(())
}
