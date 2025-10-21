//! Pubky MVP Server
//!
//! A simple HTTP server providing key-value storage with public key addressing.

mod routes;
mod storage;

use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use storage::Storage;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pubky_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create shared storage
    let storage = Arc::new(Storage::new());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the application router
    let app = Router::new()
        .route("/", get(|| async { "Pubky MVP Server" }))
        .nest("/:public_key", routes::storage_routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(storage);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("Server listening on http://127.0.0.1:3000");
    tracing::info!("Example: PUT http://127.0.0.1:3000/<public_key>/my-app/data.txt");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
