use crate::server::models::{Bucket, BucketList, CreateBucketRequest};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
};

use std::fs;
use std::path;

use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    errors: Vec<String>,
    error_code: String,
    message: String,
}

pub async fn list_buckets() -> Json<BucketList> {
    let data = BucketList {
        items: vec!["bucket_one".to_string(), "bucket_two".to_string()],
    };
    Json(data)
}

pub async fn get_bucket(Path(bucket_name): Path<String>) -> Json<Bucket> {
    let data = Bucket {
        kind: "bucket".to_string(),
        name: bucket_name,
    };

    Json(data)
}

pub async fn create_bucket(Json(payload): Json<CreateBucketRequest>) -> impl IntoResponse {
    let path = format!("buckets/{}", &payload.bucket_name);
    let bucket_path = path::Path::new(&path);

    match fs::create_dir_all(bucket_path) {
        Ok(_) => {
            println!("things");
            (
                StatusCode::CREATED,
                Json(Bucket {
                    kind: "bucket".to_string(),
                    name: payload.bucket_name,
                }),
            )
                .into_response()
        }

        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                errors: Vec::new(),
                error_code: StatusCode::CONFLICT.as_str().to_string(),
                message: "Bucket already exists.".into(),
            }),
        )
            .into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                errors: Vec::new(),
                error_code: StatusCode::INTERNAL_SERVER_ERROR.as_str().to_string(),
                message: format!("Failed to create bucket: {}", e),
            }),
        )
            .into_response(),
    }
}
