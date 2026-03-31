use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::error::AppError;
use crate::models::{ClickInfo, CreateUrlResponse, StatsResponse, UrlEntry};

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn init_pool(database_path: &str) -> DbPool {
    let manager = SqliteConnectionManager::file(database_path);
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create connection pool");

    let conn = pool.get().expect("Failed to get connection");
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .expect("Failed to set pragmas");

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT UNIQUE NOT NULL,
            original_url TEXT NOT NULL,
            redirect_type INTEGER NOT NULL DEFAULT 302,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            click_count INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS clicks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url_id INTEGER NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
            clicked_at TEXT NOT NULL DEFAULT (datetime('now')),
            referrer TEXT,
            user_agent TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_urls_code ON urls(code);
        CREATE INDEX IF NOT EXISTS idx_clicks_url_id ON clicks(url_id);",
    )
    .expect("Failed to run migrations");

    pool
}

pub fn insert_url(
    pool: &DbPool,
    code: &str,
    original_url: &str,
    redirect_type: u16,
    base_url: &str,
) -> Result<CreateUrlResponse, AppError> {
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO urls (code, original_url, redirect_type) VALUES (?1, ?2, ?3)",
        params![code, original_url, redirect_type],
    )
    .map_err(|e| match e {
        rusqlite::Error::SqliteFailure(err, _)
            if err.code == rusqlite::ErrorCode::ConstraintViolation =>
        {
            AppError::Conflict(format!("Code '{code}' is already taken"))
        }
        other => AppError::from(other),
    })?;

    let created_at: String = conn.query_row(
        "SELECT created_at FROM urls WHERE code = ?1",
        params![code],
        |row| row.get(0),
    )?;

    Ok(CreateUrlResponse {
        code: code.to_string(),
        short_url: format!("{base_url}/{code}"),
        created_at,
    })
}

pub fn code_exists(pool: &DbPool, code: &str) -> Result<bool, AppError> {
    let conn = pool.get()?;
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM urls WHERE code = ?1",
        params![code],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn get_url_by_code(pool: &DbPool, code: &str) -> Result<UrlEntry, AppError> {
    let conn = pool.get()?;
    conn.query_row(
        "SELECT id, code, original_url, redirect_type, created_at, click_count FROM urls WHERE code = ?1",
        params![code],
        |row| {
            Ok(UrlEntry {
                id: row.get(0)?,
                code: row.get(1)?,
                original_url: row.get(2)?,
                redirect_type: row.get(3)?,
                created_at: row.get(4)?,
                click_count: row.get(5)?,
            })
        },
    )
    .map_err(|_| AppError::NotFound(format!("URL with code '{code}' not found")))
}

pub fn record_click(
    pool: &DbPool,
    url_id: i64,
    referrer: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(), AppError> {
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO clicks (url_id, referrer, user_agent) VALUES (?1, ?2, ?3)",
        params![url_id, referrer, user_agent],
    )?;
    conn.execute(
        "UPDATE urls SET click_count = click_count + 1 WHERE id = ?1",
        params![url_id],
    )?;
    Ok(())
}

pub fn get_stats(pool: &DbPool, code: &str) -> Result<StatsResponse, AppError> {
    let entry = get_url_by_code(pool, code)?;

    let conn = pool.get()?;
    let last_clicked_at: Option<String> = conn
        .query_row(
            "SELECT clicked_at FROM clicks WHERE url_id = ?1 ORDER BY id DESC LIMIT 1",
            params![entry.id],
            |row| row.get(0),
        )
        .ok();

    let mut stmt = conn.prepare(
        "SELECT clicked_at, referrer, user_agent FROM clicks WHERE url_id = ?1 ORDER BY id DESC LIMIT 50",
    )?;
    let recent_clicks = stmt
        .query_map(params![entry.id], |row| {
            Ok(ClickInfo {
                clicked_at: row.get(0)?,
                referrer: row.get(1)?,
                user_agent: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(StatsResponse {
        code: entry.code,
        original_url: entry.original_url,
        redirect_type: entry.redirect_type,
        created_at: entry.created_at,
        click_count: entry.click_count,
        last_clicked_at,
        recent_clicks,
    })
}

pub fn delete_url(pool: &DbPool, code: &str) -> Result<(), AppError> {
    let conn = pool.get()?;
    let affected = conn.execute("DELETE FROM urls WHERE code = ?1", params![code])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!(
            "URL with code '{code}' not found"
        )));
    }
    Ok(())
}
