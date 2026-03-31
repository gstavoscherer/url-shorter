use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect, Response};

use crate::db;
use crate::error::AppError;
use crate::AppState;

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let entry = db::get_url_by_code(&state.pool, &code)?;

    let referrer = headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    db::record_click(
        &state.pool,
        entry.id,
        referrer.as_deref(),
        user_agent.as_deref(),
    )?;

    let response = if entry.redirect_type == 301 {
        Redirect::permanent(&entry.original_url).into_response()
    } else {
        Redirect::temporary(&entry.original_url).into_response()
    };

    Ok(response)
}
