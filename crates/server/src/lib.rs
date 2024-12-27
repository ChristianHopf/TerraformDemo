pub use anyhow::{bail, Context};
pub use axum::extract::*;
pub use axum::http::header::CONTENT_TYPE;
pub use axum::http::StatusCode;
pub use axum::response::*;
pub use axum::routing;
pub use axum::Json;
pub use axum::Router;
pub use serde::{Deserialize, Serialize};
pub use serde_json::Value;
pub use std::net::SocketAddr;
pub use std::sync::Arc;
pub use tower_http::cors::{Any, CorsLayer};
pub use tower_http::classify::*;
pub use tower_http::trace::*;
pub use tracing::*;
pub use utoipa::{IntoParams, OpenApi, ToSchema};

// monitoring
pub use lazy_static::lazy_static;
pub use prometheus::{opts, register_gauge, register_int_gauge};
pub use prometheus::{Encoder, Gauge, GaugeVec, IntGauge, Opts, Registry, TextEncoder};

#[derive(Serialize, ToSchema)]
struct HttpErrMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    message: String,
}

pub fn err500(message: &str) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(HttpErrMessage {
            code: Some(500),
            error: Some("Server Error".to_string()),
            message: message.to_string(),
        }),
    )
}
pub fn err400(message: &str) -> impl IntoResponse {
    (
        StatusCode::BAD_REQUEST,
        Json(HttpErrMessage {
            code: Some(400),
            error: Some("Bad Request".to_string()),
            message: message.to_string(),
        }),
    )
}

pub fn axum_cors_any() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

pub fn axum_trace_default() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default())
        .on_request(DefaultOnRequest::default())
        .on_response(DefaultOnResponse::new().level(Level::INFO).include_headers(true))
        .on_failure(DefaultOnFailure::default())
}

pub async fn axum_serve(listen: &str, app: Router) {
    let listener = tokio::net::TcpListener::bind(listen).await.unwrap();
    tracing::info!("Listening on {}", listen);
    axum::serve(listener, app).await.unwrap();
}

pub mod logging {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{prelude::*, EnvFilter};

    pub fn start(defaults: &str) {
        let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(defaults));
        let is_terminal = atty::is(atty::Stream::Stderr);
        let subscriber = tracing_subscriber::fmt::fmt()
            .with_env_filter(env_filter)
            .with_ansi(is_terminal)
            .finish();
        _ = subscriber.with(ErrorLayer::default()).try_init();
    }
}
