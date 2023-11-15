use anyhow::Result;
use mongoose::Model;
use parrot_api::{
    aws::{s3::Client, sqs::FifoQueue},
    eleven_labs::ElevenLabs,
    env::Config,
    logger,
    models::voice::Voice,
    types::CloneVoiceFifoMessage,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    logger::init()?;
    let config = Config::new()?;
    let voice_api = ElevenLabs::new()?;
    let sqs = FifoQueue::new(config.clone_voice_queue_url).await;
    let s3 = Client::new("parrot-api").await;
    let messages = sqs.receive_fifo_message::<CloneVoiceFifoMessage>().await?;
    for message in messages {
        let voice = Voice::read_by_id(&message.voice_id).await?;
    }
    Ok(())
}
