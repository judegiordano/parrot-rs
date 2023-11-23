use anyhow::Result;
use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, LambdaEvent};
use mongoose::{bson::doc, Model};
use parrot_api::{
    aws::sqs::{FifoMessage, FifoQueue},
    env::Config,
    logger,
    models::voice::{Voice, VoiceStatus},
    types::TrainSampleFifoMessage,
};

async fn handler(event: LambdaEvent<S3Event>) -> Result<()> {
    let config = Config::new()?;
    let sqs = FifoQueue::new(config.train_voice_queue_url).await;
    for record in event.payload.records {
        let key = match &record.s3.object.key {
            Some(key) => key,
            None => anyhow::bail!("no key exists on object"),
        };
        let split = key.split(".mp3").collect::<Vec<_>>();
        let voice_id = match split.first() {
            Some(str) => *str,
            None => anyhow::bail!("missing file name on split key"),
        };
        // push to FIFO for training
        sqs.send_fifo_message::<TrainSampleFifoMessage>(FifoMessage {
            body: TrainSampleFifoMessage {
                voice_id: voice_id.to_string(),
            },
            group: voice_id.to_string(),
            deduplication_id: voice_id.to_string(),
        })
        .await?;
        let updated_voice = Voice::update(
            doc! { "_id": voice_id },
            doc! { "status": VoiceStatus::Training.to_string(), },
        )
        .await?;
        tracing::info!("VOICE {:?}", updated_voice);
    }
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    logger::init()?;
    run(service_fn(handler)).await
}
