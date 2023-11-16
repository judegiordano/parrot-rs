use mongoose::{
    bson::{doc, DateTime},
    mongodb::{options::IndexOptions, results::CreateIndexesResult, IndexModel},
    types::MongooseError,
    Model,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum VoiceStatus {
    Active,
    Draft,
    Deleted,
}

impl VoiceStatus {
    pub fn to_string(&self) -> String {
        match self {
            VoiceStatus::Active => "active".to_string(),
            VoiceStatus::Draft => "draft".to_string(),
            VoiceStatus::Deleted => "deleted".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Voice {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub status: String,
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
            status: VoiceStatus::Draft.to_string(),
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
        ])
        .await
    }
}
