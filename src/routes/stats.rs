use axum::extract::{Path, State};
use axum::Json;

use crate::db;
use crate::error::AppError;
use crate::models::StatsResponse;
use crate::AppState;

pub async fn get_stats(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<StatsResponse>, AppError> {
    let stats = db::get_stats(&state.pool, &code)?;
    Ok(Json(stats))
}
