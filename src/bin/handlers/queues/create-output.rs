use anyhow::Result;
use mongoose::{bson::doc, Model};
use parrot_api::{
    aws::{s3::Client, sqs::FifoQueue},
    eleven_labs::ElevenLabs,
    env::Config,
    logger,
    models::output::Output,
    models::{output::OutputStatus, voice::Voice},
    types::CreateOutputFifoMessage,
};

#[allow(unused_variables)]
#[tokio::main]
pub async fn main() -> Result<()> {
    logger::init()?;
    let config = Config::new()?;
    let voice_api = ElevenLabs::new()?;
    let sqs = FifoQueue::new(config.create_output_queue_url).await;
    let outputs_bucket = Client::new(&config.outputs_bucket_name).await;
    let messages = sqs
        .receive_fifo_message::<CreateOutputFifoMessage>()
        .await?;
    for message in messages {
        let output = Output::read_by_id(&message.output_id).await?;
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
