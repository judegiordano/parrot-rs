use anyhow::Result;
use aws_sdk_s3 as s3;
use s3::{
    operation::{
        delete_object::DeleteObjectOutput, get_object::GetObjectOutput,
        list_buckets::ListBucketsOutput, put_object::PutObjectOutput,
    },
    presigning::PresigningConfig,
    Client as AwsClient,
};
use std::time::Duration;

#[derive(Debug)]
pub struct Client {
    pub bucket: String,
    pub client: AwsClient,
}

impl Client {
    pub async fn new(bucket: &str) -> Self {
        let config = aws_config::load_from_env().await;
        let client = s3::Client::new(&config);
        Self {
            bucket: bucket.to_string(),
            client,
        }
    }

    pub async fn list_buckets(&self) -> Result<ListBucketsOutput> {
        Ok(self.client.list_buckets().send().await?)
    }

    pub async fn get_object(&self, key: String) -> Result<GetObjectOutput> {
        let Self { bucket, client } = self;
        let request = client.get_object().bucket(bucket).key(key);
        let output = request.send().await?;
        Ok(output)
    }

    pub async fn get_presigned_url(&self, key: &str) -> Result<String> {
        let Self { bucket, client } = self;
        // TODO: make this configurable
        let expires_in = Duration::from_secs(60);
        let presigned_request = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;
        let url = presigned_request.uri().to_string();
        Ok(url)
    }

    pub async fn put_presigned_url(&self, key: &str, expires_in: Duration) -> Result<String> {
        let Self { bucket, client } = self;
        let presigned_request = client
            .put_object()
            .bucket(bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;
        let url = presigned_request.uri().to_string();
        Ok(url)
    }

    pub async fn put_object(&self, key: &str, body: Vec<u8>) -> Result<PutObjectOutput> {
        let Self { bucket, client } = self;
        let builder = client
            .put_object()
            .bucket(bucket)
            .body(body.into())
            .key(key);
        let output = builder.send().await?;
        Ok(output)
    }

    pub async fn delete_object(&self, key: &str) -> Result<DeleteObjectOutput> {
        let Self { bucket, client } = self;
        let builder = client.delete_object().bucket(bucket).key(key);
        let output = builder.send().await?;
        Ok(output)
    }
}
