use mongoose::{
    bson::{doc, DateTime},
    mongodb::{options::IndexOptions, results::CreateIndexesResult, IndexModel},
    types::MongooseError,
    Model,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum VoiceStatus {
    Active,
    Draft,
    Deleted,
}

impl VoiceStatus {
    pub fn to_string(&self) -> String {
        match self {
            VoiceStatus::Active => "Active".to_string(),
            VoiceStatus::Draft => "Draft".to_string(),
            VoiceStatus::Deleted => "Deleted".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Voice {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub status: VoiceStatus,
    pub description: Option<String>,
    pub eleven_labs_id: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            name: std::string::String::default(),
            status: VoiceStatus::Draft,
            description: None,
            eleven_labs_id: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for Voice {}

impl Voice {
    pub async fn migrate() -> Result<CreateIndexesResult, MongooseError> {
        Self::create_indexes(&vec![
            IndexModel::builder()
                .keys(doc! { "name": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            IndexModel::builder()
                .keys(doc! { "eleven_labs_id": 1 })
                .build(),
            IndexModel::builder().keys(doc! { "status": 1 }).build(),
        ])
        .await
    }

    pub async fn active_voices_count() -> anyhow::Result<u64> {
        Ok(Self::count(Some(doc! { "status": VoiceStatus::Active.to_string() })).await?)
    }
}
