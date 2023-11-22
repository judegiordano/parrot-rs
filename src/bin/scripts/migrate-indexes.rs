use anyhow::Result;
use parrot_api::{
    logger,
    models::{output::Output, voice::Voice},
};

#[tokio::main]
pub async fn main() -> Result<()> {
    logger::init()?;
    let results = futures::try_join!(Voice::migrate(), Output::migrate())?;
    tracing::info!("{:#?}", results);
    Ok(())
}
