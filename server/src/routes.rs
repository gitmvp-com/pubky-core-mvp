//! HTTP routes for storage operations
//!
//! Provides PUT/GET/DELETE endpoints for key-value storage.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, put},
    Json, Router,
};
use bytes::Bytes;
use pubky_common::PublicKey;
use serde_json::json;
use std::sync::Arc;

use crate::storage::Storage;

/// Application state containing shared storage
type AppState = Arc<Storage>;

/// Custom error type for route handlers
#[derive(Debug)]
enum ApiError {
    InvalidPublicKey(String),
    NotFound,
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::InvalidPublicKey(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

/// Create the storage routes
pub fn storage_routes() -> Router<AppState> {
    Router::new()
        .route("/*path", put(put_data))
        .route("/*path", get(get_data))
        .route("/*path", delete(delete_data))
}

/// PUT /{public_key}/{path}
/// Store data at the specified path for a public key
async fn put_data(
    State(storage): State<AppState>,
    Path((public_key_str, path)): Path<(String, String)>,
    body: Bytes,
) -> Result<StatusCode, ApiError> {
    tracing::debug!("PUT /{}/{}", public_key_str, path);

    let public_key = PublicKey::from_z32(&public_key_str)
        .map_err(|e| ApiError::InvalidPublicKey(e.to_string()))?;

    storage.put(public_key, path, body.to_vec());

    Ok(StatusCode::CREATED)
}

/// GET /{public_key}/{path}
/// Retrieve data from the specified path or list if path ends with /
async fn get_data(
    State(storage): State<AppState>,
    Path((public_key_str, path)): Path<(String, String)>,
) -> Result<Response, ApiError> {
    tracing::debug!("GET /{}/{}", public_key_str, path);

    let public_key = PublicKey::from_z32(&public_key_str)
        .map_err(|e| ApiError::InvalidPublicKey(e.to_string()))?;

    // If path ends with /, list all keys with that prefix
    if path.ends_with('/') {
        let keys = storage.list(&public_key, &path);
        return Ok(Json(json!({
            "keys": keys,
            "count": keys.len()
        }))
        .into_response());
    }

    // Otherwise, get the value
    match storage.get(&public_key, &path) {
        Some(data) => Ok(data.into_response()),
        None => Err(ApiError::NotFound),
    }
}

/// DELETE /{public_key}/{path}
/// Delete data at the specified path
async fn delete_data(
    State(storage): State<AppState>,
    Path((public_key_str, path)): Path<(String, String)>,
) -> Result<StatusCode, ApiError> {
    tracing::debug!("DELETE /{}/{}", public_key_str, path);

    let public_key = PublicKey::from_z32(&public_key_str)
        .map_err(|e| ApiError::InvalidPublicKey(e.to_string()))?;

    if storage.delete(&public_key, &path) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}
