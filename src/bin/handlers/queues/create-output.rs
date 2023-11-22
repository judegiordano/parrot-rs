use anyhow::Result;
use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, LambdaEvent};
use mongoose::{bson::doc, Model};
use parrot_api::{
    aws::s3::Client,
    eleven_labs::ElevenLabs,
    env::Config,
    logger,
    models::output::Output,
    models::{output::OutputStatus, voice::Voice},
    types::CreateOutputFifoMessage,
};

pub async fn handler(event: LambdaEvent<SqsEvent>) -> Result<()> {
    let messages = event.payload.records;
    let config = Config::new()?;
    let voice_api = ElevenLabs::new()?;
    let outputs_bucket = Client::new(&config.outputs_bucket_name).await;
    for message in messages {
        let body = message.body.unwrap();
        let data = serde_json::from_str::<CreateOutputFifoMessage>(&body)?;
        let output = Output::read_by_id(&data.output_id).await?;
        let voice = Voice::read_by_id(&output.voice).await?;
        if voice.eleven_labs_id.is_none() {
            anyhow::bail!("no eleven labs id supplied");
        };
        let bytes = voice_api
            .text_to_speech(&voice.eleven_labs_id.unwrap(), &output.text)
            .await?;
        let updated = Output::update(
            doc! {
                "_id": output.id
            },
            doc! {
                "status": OutputStatus::Done.to_string()
            },
        )
        .await?;
        outputs_bucket
            .put_object(&updated.id, bytes.to_vec())
            .await?;
        // TODO: send server side event of process complete
        tracing::info!("OUTPUT: {:?}", updated);
    }
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), lambda_http::Error> {
    logger::init()?;
    run(service_fn(handler)).await
}
