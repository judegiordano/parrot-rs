use anyhow::Result;
use bytes::Bytes;
use reqwest::{
    header::{self, HeaderMap},
    multipart::{self, Form},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::env::Config;

pub struct ElevenLabs {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub detail: ErrorMessage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddVoiceResponse {
    pub voice_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VoicesResponse {
    pub voices: Vec<Voice>,
}

impl ElevenLabs {
    // Internal Methods
    fn headers(&self) -> Result<HeaderMap> {
        let mut headers = header::HeaderMap::new();
        let key = &self.api_key;
        headers.insert("xi-api-key", key.parse()?);
        Ok(headers)
    }
    fn base_url() -> String {
        "https://api.elevenlabs.io/v1".to_string()
    }
    async fn get<T: for<'a> Deserialize<'a> + Serialize>(&self, url: &str) -> Result<T> {
        let base_url = Self::base_url();
        let headers = self.headers()?;
        let client = reqwest::Client::builder();
        let client = client.default_headers(headers).build()?;
        let url = format!("{base_url}/{url}");
        let response = match client.get(url).send().await {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("error connecting to server: {:?}", err.to_string());
                anyhow::bail!(err)
            }
        };
        let data = response.json::<serde_json::Value>().await?;
        let serialized = serde_json::to_string(&data)?;
        match serde_json::from_str::<ErrorResponse>(&serialized) {
            Ok(err) => {
                tracing::error!("{:?}", err);
                anyhow::bail!("{:?}", err.detail.message)
            }
            Err(_) => Ok(serde_json::from_str::<T>(&serialized)?),
        }
    }

    async fn post_form<T: for<'a> Deserialize<'a> + Serialize>(
        &self,
        url: &str,
        form: Form,
    ) -> Result<T> {
        let base_url = Self::base_url();
        let mut headers = self.headers()?;
        headers.insert("Content-Type", "multipart/form-data".parse()?);
        let client = reqwest::Client::builder();
        let client = client.default_headers(headers).build()?;
        let url = format!("{base_url}/{url}");
        let response = match client.post(url).multipart(form).send().await {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("error connecting to server: {:?}", err.to_string());
                anyhow::bail!(err)
            }
        };
        let data = response.json::<serde_json::Value>().await?;
        let serialized = serde_json::to_string(&data)?;
        match serde_json::from_str::<ErrorResponse>(&serialized) {
            Ok(err) => {
                tracing::error!("{:?}", err);
                anyhow::bail!("{:?}", err.detail.message)
            }
            Err(_) => Ok(serde_json::from_str::<T>(&serialized)?),
        }
    }

    // Public Api Methods
    pub fn new() -> Result<Self> {
        let Config {
            eleven_labs_api_key: api_key,
            ..
        } = Config::new()?;
        Ok(Self { api_key })
    }

    pub async fn get_voices(&self) -> Result<Vec<Voice>> {
        let response = self.get::<VoicesResponse>("voices").await?;
        Ok(response.voices)
    }

    pub async fn get_voice(&self, voice_id: &str) -> Result<Voice> {
        let response = self.get::<Voice>(&format!("voices/{voice_id}")).await?;
        Ok(response)
    }

    // 11mb max file size
    pub async fn add_voice(
        &self,
        voice_name: &str,
        data: &Vec<u8>,
        description: Option<&str>,
    ) -> Result<AddVoiceResponse> {
        let file_name = slug::slugify(voice_name);
        let part = multipart::Part::stream(data.to_owned())
            .file_name(format!("{file_name}.mp3"))
            .mime_str("audio/mpeg")?;
        let form = multipart::Form::new()
            .text("name", voice_name.to_string())
            .text(
                "description",
                description.map_or(String::new(), std::string::ToString::to_string),
            )
            .part("files", part);
        let response = self
            .post_form::<AddVoiceResponse>("voices/add", form)
            .await?;
        Ok(response)
    }

    pub async fn text_to_speech(&self, voice_id: &str, text: &str) -> Result<Bytes> {
        let base_url = Self::base_url();
        let headers = self.headers()?;
        let optimizations = "optimize_streaming_latency=3";
        let url = format!("{base_url}/text-to-speech/{voice_id}/stream?{optimizations}");
        let client = reqwest::Client::builder();
        let client = client.default_headers(headers).build()?;
        let payload = json!({
            "text": text,
            "model_id": "eleven_monolingual_v1",
            "voice_settings": {
              "stability": 0,
              "similarity_boost": 0,
              "style": 0.5,
              "use_speaker_boost": true
            }
        });
        let response = client.post(url).json(&payload).send().await?;
        Ok(response.bytes().await?)
    }
}
