use anyhow::Result;
use lambda_runtime::{run, service_fn, LambdaEvent};
use serde::{Deserialize, Serialize};
use sst_example::logger;

#[derive(Debug, Deserialize, Serialize)]
struct ExamplePayload {
    payload: String,
}

async fn handler(event: LambdaEvent<ExamplePayload>) -> Result<()> {
    tracing::info!("request received: {:?}", event);
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    logger::init()?;
    run(service_fn(handler)).await
}
