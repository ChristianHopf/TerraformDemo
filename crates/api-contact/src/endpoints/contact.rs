use super::AppState;
use proto::*;
use server::*;

#[utoipa::path(
    post, path = "/api/contact/submit", request_body = ContactFormRequest,
    responses(
        (status = 200, description = "ok", body = ContactFormResponse)
    )
)]
pub async fn submit(
    Extension(state): Extension<Arc<AppState>>,
    Json(body): Json<ContactFormRequest>,
) -> impl IntoResponse {
    let state = state.clone();
    let entry = match ContactFormEntry::new(&body) {
        Ok(x) => x,
        Err(e) => return err400(&e.to_string()).into_response(),
    };
    match entry.notify(&state.email_config) {
        Ok(result) => Json(ContactFormResponse { result }).into_response(),
        Err(e) => err500(&e.to_string()).into_response()
    }
}
