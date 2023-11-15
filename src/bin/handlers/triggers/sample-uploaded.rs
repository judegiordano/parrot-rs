use anyhow::Result;
use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, LambdaEvent};
use mongoose::Model;
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
        let sample = sample_bucket.get_object(key).await?;
        let api = ElevenLabs::new()?;
        let name = "placeholder-name";
        let data = sample.body.collect().await?.to_vec();
        let cloned_voice = api.add_voice(&name, &data, None).await?;
        let voice_doc = Voice {
            name: name.to_string(),
            status: VoiceStatus::Active,
            eleven_labs_id: Some(cloned_voice.voice_id),
            ..Default::default()
        }
        .save()
        .await?;
        tracing::info!("cloned voice {:?}", voice_doc);
    }
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    logger::init()?;
    run(service_fn(handler)).await
}
