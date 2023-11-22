use lambda_web::actix_web::{web, HttpRequest, HttpResponse};
use mongoose::{bson::doc, Model};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    errors::ApiResponse,
    helpers::authenticate,
    models::{
        output::Output,
        voice::{Voice, VoiceStatus},
    },
};

#[derive(Deserialize, Serialize)]
pub struct OutputPayload {
    voice_name: String,
    text: String,
}

pub async fn create_output(req: HttpRequest, body: web::Json<OutputPayload>) -> ApiResponse {
    authenticate(req).await?;
    let name = slug::slugify(body.voice_name.to_string());
    let voice = match Voice::read(doc! { "name": name }).await {
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
    Ok(HttpResponse::Created().json(output))
}
