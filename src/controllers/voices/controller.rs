use lambda_web::actix_web::{web, HttpRequest, HttpResponse};
use mongoose::{bson::doc, Model};
use serde_json::json;

use crate::{
    aws::s3::Client,
    eleven_labs::ElevenLabs,
    env::Config,
    errors::ApiResponse,
    helpers::authenticate,
    models::voice::{Voice, VoiceStatus},
};

pub async fn list_voices(req: HttpRequest) -> ApiResponse {
    authenticate(req).await?;
    let voices = Voice::list(None, None).await?;
    Ok(HttpResponse::Ok().json(voices))
}

pub async fn get_voice_by_id(req: HttpRequest, voice_id: web::Path<String>) -> ApiResponse {
    authenticate(req).await?;
    let voice = Voice::read_by_id(&voice_id).await?;
    Ok(HttpResponse::Ok().json(voice))
}

pub async fn delete_voice(req: HttpRequest, voice_id: web::Path<String>) -> ApiResponse {
    authenticate(req).await?;
    let voice = Voice::read_by_id(&voice_id).await?;
    if voice.status.to_string() != VoiceStatus::Active.to_string() {
        return Ok(
            HttpResponse::InternalServerError().json(json!({ "error": "voice is not active" }))
        );
    }
    let eleven_labs_id = match voice.eleven_labs_id {
        Some(id) => id,
        None => {
            return Ok(HttpResponse::InternalServerError()
                .json(json!({ "error": "voice has no id attached" })))
        }
    };
    // delete voice from eleven labs
    let eleven_labs = ElevenLabs::new()?;
    let deleted = match eleven_labs.delete_voice(&eleven_labs_id).await {
        Ok(status) => status.as_u16(),
        Err(err) => {
            return Ok(HttpResponse::InternalServerError().json(json!({ "error": err.to_string() })))
        }
    };
    if deleted != 200 {
        return Ok(
            HttpResponse::InternalServerError().json(json!({ "error": "error deleting voice" }))
        );
    }
    // remove object from s3
    {
        let config = Config::new()?;
        let samples_bucket = Client::new(&config.samples_bucket_name).await;
        samples_bucket.delete_object(&voice_id).await?;
    }
    let eleven_labs_id: Option<String> = None;
    let voice = Voice::update(
        doc! { "_id": voice_id.to_string() },
        doc! {
            "status": VoiceStatus::Deleted.to_string(),
            "eleven_labs_id": eleven_labs_id
        },
    )
    .await?;
    Ok(HttpResponse::Ok().json(voice))
}
