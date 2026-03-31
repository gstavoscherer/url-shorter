use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::db;
use crate::error::AppError;
use crate::AppState;

pub async fn delete_url(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<StatusCode, AppError> {
    db::delete_url(&state.pool, &code)?;
    Ok(StatusCode::NO_CONTENT)
}
