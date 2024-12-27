mod contact;

use proto::*;
use server::*;

pub struct AppState {
    pub email_config: EmailConfig,
}

pub async fn run(listen_addr: &str, email_config: EmailConfig) -> anyhow::Result<()> {
    let shared_state = Arc::new(AppState { email_config });
    use utoipa::Path;

    let app = Router::new()
        .route(&contact::__path_submit::path(), post(contact::submit))
        .layer(DefaultBodyLimit::disable())
        .layer(Extension(shared_state))
        .layer(axum_trace_default())
        .layer(axum_cors_any());
    Ok(axum_serve(listen_addr, app).await)
}
