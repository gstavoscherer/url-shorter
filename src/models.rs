use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UrlEntry {
    pub id: i64,
    pub code: String,
    pub original_url: String,
    pub redirect_type: u16,
    pub created_at: String,
    pub click_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
    pub custom_alias: Option<String>,
    pub redirect_type: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub code: String,
    pub short_url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub code: String,
    pub original_url: String,
    pub redirect_type: u16,
    pub created_at: String,
    pub click_count: i64,
    pub last_clicked_at: Option<String>,
    pub recent_clicks: Vec<ClickInfo>,
}

#[derive(Debug, Serialize)]
pub struct ClickInfo {
    pub clicked_at: String,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}
