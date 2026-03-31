use axum::extract::State;
use axum::Json;

use crate::db;
use crate::error::AppError;
use crate::models::{CreateUrlRequest, CreateUrlResponse};
use crate::shortcode;
use crate::AppState;

pub async fn create_url(
    State(state): State<AppState>,
    Json(payload): Json<CreateUrlRequest>,
) -> Result<Json<CreateUrlResponse>, AppError> {
    if payload.url.is_empty() {
        return Err(AppError::BadRequest("URL is required".to_string()));
    }

    if !payload.url.starts_with("http://") && !payload.url.starts_with("https://") {
        return Err(AppError::BadRequest(
            "URL must start with http:// or https://".to_string(),
        ));
    }

    let redirect_type = payload.redirect_type.unwrap_or(302);
    if redirect_type != 301 && redirect_type != 302 {
        return Err(AppError::BadRequest(
            "redirect_type must be 301 or 302".to_string(),
        ));
    }

    let code = match &payload.custom_alias {
        Some(alias) => {
            shortcode::validate_alias(alias)?;
            alias.clone()
        }
        None => {
            let mut code = shortcode::generate(state.config.code_length);
            let mut retries = 0;
            while db::code_exists(&state.pool, &code)? && retries < 3 {
                code = shortcode::generate(state.config.code_length);
                retries += 1;
            }
            if db::code_exists(&state.pool, &code)? {
                return Err(AppError::Internal(
                    "Failed to generate unique code".to_string(),
                ));
            }
            code
        }
    };

    let response = db::insert_url(
        &state.pool,
        &code,
        &payload.url,
        redirect_type,
        &state.config.base_url,
    )?;

    Ok(Json(response))
}
