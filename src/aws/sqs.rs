use anyhow::Result;
use aws_sdk_sqs as sqs;
use serde::{Deserialize, Serialize};
use sqs::{operation::send_message::SendMessageOutput, Client as AwsClient};

pub struct FifoQueue {
    pub queue_url: String,
    pub client: AwsClient,
}

pub struct FifoMessage<T: Serialize + for<'a> Deserialize<'a>> {
    pub body: T,
    pub group: String,
    pub deduplication_id: String,
}

impl FifoQueue {
    pub async fn new(queue_url: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = sqs::Client::new(&config);
        Self { queue_url, client }
    }

    pub async fn send_fifo_message<T: Serialize + for<'a> Deserialize<'a>>(
        &self,
        message: FifoMessage<T>,
    ) -> Result<SendMessageOutput> {
        let Self { queue_url, client } = self;
        let body = serde_json::to_string(&message.body)?;
        let response = client
            .send_message()
            .queue_url(queue_url)
            .message_body(&body)
            .message_group_id(&message.group)
            .message_deduplication_id(&message.deduplication_id)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn receive_fifo_message<T: for<'a> Deserialize<'a>>(&self) -> Result<Vec<T>> {
        let Self { queue_url, client } = self;
        let output = client.receive_message().queue_url(queue_url).send().await?;
        let mut results = vec![];
        for message in output.messages.unwrap_or_default() {
            if let Some(body) = message.body() {
                let data = serde_json::from_str::<T>(body)?;
                results.push(data);
            }
        }
        Ok(results)
    }
}
