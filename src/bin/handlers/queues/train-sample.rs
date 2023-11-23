use anyhow::Result;
use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, LambdaEvent};
use mongoose::{bson::doc, Model};
use parrot_api::{
    aws::s3::Client,
    eleven_labs::ElevenLabs,
    env::Config,
    logger,
    models::voice::{Voice, VoiceStatus},
    types::TrainSampleFifoMessage,
};

pub async fn handler(event: LambdaEvent<SqsEvent>) -> Result<()> {
    let messages = event.payload.records;
    let config = Config::new()?;
    let sample_bucket = Client::new(&config.samples_bucket_name).await;
    let eleven_labs = ElevenLabs::new()?;
    for message in messages {
        let body = message.body.unwrap();
        let data = serde_json::from_str::<TrainSampleFifoMessage>(&body)?;
        let voice = Voice::read_by_id(&data.voice_id).await?;
        if voice.status == VoiceStatus::Active {
            tracing::info!("voice is already active: {:?}", voice);
            return Ok(());
        }
        let key = format!("{}.mp3", data.voice_id);
        // get sample from s3
        let sample = sample_bucket.get_object(key).await?;
        let data = sample.body.collect().await?.to_vec();
        // clone voice from eleven labs
        let cloned_voice = eleven_labs
            .add_voice(&voice.name, &data, voice.description.as_deref())
            .await?;
        // update voice status
        let updated_voice = Voice::update(
            doc! { "_id": voice.id },
            doc! {
                "status": VoiceStatus::Active.to_string(),
                "eleven_labs_id": Some(cloned_voice.voice_id),
            },
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
