use mongoose::{
    bson::{doc, DateTime},
    mongodb::{results::CreateIndexesResult, IndexModel},
    types::MongooseError,
    Model,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum OutputStatus {
    Pending,
    Done,
}

impl OutputStatus {
    pub fn to_string(&self) -> String {
        match self {
            OutputStatus::Pending => "Pending".to_string(),
            OutputStatus::Done => "Done".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Output {
    #[serde(rename = "_id")]
    pub id: String,
    pub voice: String,
    pub text: String,
    pub status: OutputStatus,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            voice: std::string::String::default(),
            text: std::string::String::default(),
            status: OutputStatus::Pending,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Model for Output {}

impl Output {
    pub async fn migrate() -> Result<CreateIndexesResult, MongooseError> {
        Self::create_indexes(&vec![IndexModel::builder()
            .keys(doc! { "voice": 1 })
            .build()])
        .await
    }
}
