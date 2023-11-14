use mongoose::{
    bson::{doc, DateTime},
    mongodb::{options::IndexOptions, results::CreateIndexesResult, IndexModel},
    types::MongooseError,
    Model,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Voice {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub eleven_labs_id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            id: Self::generate_nanoid(),
            name: std::string::String::default(),
            description: None,
            eleven_labs_id: std::string::String::default(),
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
