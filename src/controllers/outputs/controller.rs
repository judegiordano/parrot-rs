use lambda_web::actix_web::{web, HttpRequest, HttpResponse};
use mongoose::{bson::doc, Model};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    aws::sqs::{FifoMessage, FifoQueue},
    env::Config,
    errors::ApiResponse,
    helpers::authenticate,
    models::{
        output::Output,
        voice::{Voice, VoiceStatus},
    },
    types::CreateOutputFifoMessage,
};

#[derive(Deserialize, Serialize)]
pub struct OutputPayload {
    voice_id: String,
    text: String,
}

pub async fn create_output(req: HttpRequest, body: web::Json<OutputPayload>) -> ApiResponse {
    authenticate(req).await?;
    let voice = match Voice::read_by_id(&body.voice_id).await {
        Ok(voice) => voice,
        Err(_) => return Ok(HttpResponse::NotFound().json(json!({ "error": "no voice found" }))),
    };
    if voice.status != VoiceStatus::Active {
        return Ok(HttpResponse::BadRequest().json(json!({ "error": "voice is not active" })));
    }
    let output = Output {
        voice: voice.id,
        text: body.text.trim().to_string(),
        ..Default::default()
    };
    let output = output.save().await?;
    let config = Config::new()?;
    let sqs = FifoQueue::new(config.create_output_queue_url).await;
    // push to FIFO
    sqs.send_fifo_message::<CreateOutputFifoMessage>(FifoMessage {
        body: CreateOutputFifoMessage {
            output_id: output.id.to_string(),
        },
        group: output.voice.to_string(),
        deduplication_id: output.id.to_string(),
    })
    .await?;
    Ok(HttpResponse::Created().json(output))
}

#[derive(Deserialize, Serialize)]
pub struct SearchOutputTextPayload {
    text: String,
}

pub async fn search_outputs_text(
    req: HttpRequest,
    body: web::Json<SearchOutputTextPayload>,
) -> ApiResponse {
    authenticate(req).await?;
    let results = Output::search_text(&body.text).await?;
    Ok(HttpResponse::Created().json(results))
}
