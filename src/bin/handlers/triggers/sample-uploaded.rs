use anyhow::Result;
use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, LambdaEvent};
use mongoose::{bson::doc, Model};
use parrot_api::{
    aws::s3::Client,
    eleven_labs::ElevenLabs,
    logger,
    models::voice::{Voice, VoiceStatus},
};

async fn handler(event: LambdaEvent<S3Event>) -> Result<()> {
    for record in event.payload.records {
        let key = record.s3.object.key.unwrap();
        let bucket = record.s3.bucket.name.unwrap();
        let sample_bucket = Client::new(&bucket).await;
        let sample = sample_bucket.get_object(key.to_string()).await?;
        let api = ElevenLabs::new()?;
        let voice = Voice::read_by_id(&key).await?;
        let data = sample.body.collect().await?.to_vec();
        let cloned_voice = api.add_voice(&voice.name, &data, None).await?;
        let updated_voice = Voice::update(
            doc! { "_id": voice.id },
            doc! {
                "status": VoiceStatus::Active.to_string(),
                "eleven_labs_id": Some(cloned_voice.voice_id),
            },
        )
        .await?;
        tracing::info!("cloned voice {:?}", updated_voice);
    }
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    logger::init()?;
    run(service_fn(handler)).await
}
