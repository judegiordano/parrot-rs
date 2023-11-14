use lambda_web::actix_web::{web, HttpResponse};
use mongoose::{
    bson::{doc, DateTime},
    types::ListOptions,
    Model,
};
use serde::Serialize;

use crate::{
    config::{self, Env, Stage},
    errors::{ApiResponse, AppResponse},
    models::record::Record,
};

#[derive(Serialize)]
struct PingMessage {
    stage: Stage,
    region: String,
}

pub async fn ping() -> AppResponse {
    let Env { stage, region, .. } = config::Env::new()?;
    Ok(HttpResponse::Ok().json(PingMessage { stage, region }))
}

pub async fn create_record() -> ApiResponse {
    let now = DateTime::now();
    let new_record = Record {
        payload: format!("request received: {now}"),
        ..Default::default()
    };
    Ok(HttpResponse::Created().json(new_record.save().await?))
}

pub async fn read_record(id: web::Path<String>) -> ApiResponse {
    let record = Record::read_by_id(&id).await?;
    Ok(HttpResponse::Ok().json(record))
}

pub async fn list_records() -> ApiResponse {
    let records = Record::list(
        None,
        Some(ListOptions {
            sort: Some(doc! { "created_at": -1 }),
            ..Default::default()
        }),
    )
    .await?;
    Ok(HttpResponse::Ok().json(records))
}
