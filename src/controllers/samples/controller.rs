use std::time::Duration;

use lambda_web::actix_web::{web, HttpResponse};
use mongoose::{bson::doc, Model};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{aws::s3::Client, env, errors::ApiResponse, models::voice::Voice};

#[derive(Deserialize, Serialize)]
pub struct UploadSampleBody {
    pub voice_name: String,
    pub description: Option<String>,
}

pub async fn request_put_url(body: web::Json<UploadSampleBody>) -> ApiResponse {
    let config = env::Config::new()?;
    let s3 = Client::new(&config.samples_bucket_name).await;
    let name = slug::slugify(body.voice_name.to_string());
    let count = Voice::count(None).await?;
    if count >= 10 {
        return Ok(HttpResponse::BadRequest().json(json!({ "error": "10 voice limit reached" })));
    }
    match Voice::read(doc! { "name": &name }).await {
        Ok(_) => {
            return Ok(
                HttpResponse::BadRequest().json(json!({ "error": "voice with name is taken" }))
            )
        }
        Err(_) => (),
    };
    let description = body
        .description
        .as_ref()
        .map_or(None, |desc| Some(desc.to_string()));
    let voice = Voice {
        name,
        description,
        ..Default::default()
    }
    .save()
    .await?;
    let key = format!("{}.mp3", voice.id);
    let url = s3.put_presigned_url(&key, Duration::from_secs(120)).await?;
    Ok(HttpResponse::Ok().json(json!({ "url": url, "voice": voice })))
}
