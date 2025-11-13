use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::Utc;
use crate::db::SSHSession;
use crate::encryption::{encrypt_password, decrypt_password};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Not found")]
    NotFound,
}

/// Initialize the database pool and run migrations
pub async fn init_db(database_url: &str) -> Result<SqlitePool, DbError> {
    // Create database if it doesn't exist
    let pool = SqlitePool::connect(database_url).await?;

    // Run migrations
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            host TEXT NOT NULL,
            port INTEGER NOT NULL,
            username TEXT NOT NULL,
            auth_type TEXT NOT NULL,
            password TEXT,
            private_key TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

/// Get all sessions from the database
pub async fn get_sessions(pool: &SqlitePool) -> Result<Vec<SSHSession>, DbError> {
    let rows = sqlx::query("SELECT * FROM sessions ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    let mut sessions = Vec::new();
    for row in rows {
        let password: Option<String> = row.get("password");
        let decrypted_password = if let Some(enc_pass) = password {
            if enc_pass.is_empty() {
                None
            } else {
                Some(decrypt_password(&enc_pass).map_err(|e| DbError::Encryption(e.to_string()))?)
            }
        } else {
            None
        };

        sessions.push(SSHSession {
            id: row.get("id"),
            name: row.get("name"),
            host: row.get("host"),
            port: row.get::<i64, _>("port") as u16,
            username: row.get("username"),
            auth_type: row.get("auth_type"),
            password: decrypted_password,
            private_key: row.get("private_key"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(sessions)
}

/// Get a single session by ID
pub async fn get_session(pool: &SqlitePool, id: &str) -> Result<SSHSession, DbError> {
    let row = sqlx::query("SELECT * FROM sessions WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(DbError::NotFound)?;

    let password: Option<String> = row.get("password");
    let decrypted_password = if let Some(enc_pass) = password {
        if enc_pass.is_empty() {
            None
        } else {
            Some(decrypt_password(&enc_pass).map_err(|e| DbError::Encryption(e.to_string()))?)
        }
    } else {
        None
    };

    Ok(SSHSession {
        id: row.get("id"),
        name: row.get("name"),
        host: row.get("host"),
        port: row.get::<i64, _>("port") as u16,
        username: row.get("username"),
        auth_type: row.get("auth_type"),
        password: decrypted_password,
        private_key: row.get("private_key"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

/// Save or update a session
pub async fn save_session(pool: &SqlitePool, mut session: SSHSession) -> Result<SSHSession, DbError> {
    let now = Utc::now().to_rfc3339();

    // Encrypt password if present
    let encrypted_password = if let Some(password) = &session.password {
        if password.is_empty() {
            None
        } else {
            Some(encrypt_password(password).map_err(|e| DbError::Encryption(e.to_string()))?)
        }
    } else {
        None
    };

    if session.id.is_empty() {
        // Create new session
        session.id = Uuid::new_v4().to_string();
        session.created_at = now.clone();
        session.updated_at = now;

        sqlx::query(
            "INSERT INTO sessions (id, name, host, port, username, auth_type, password, private_key, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&session.id)
        .bind(&session.name)
        .bind(&session.host)
        .bind(session.port as i64)
        .bind(&session.username)
        .bind(&session.auth_type)
        .bind(&encrypted_password)
        .bind(&session.private_key)
        .bind(&session.created_at)
        .bind(&session.updated_at)
        .execute(pool)
        .await?;
    } else {
        // Update existing session
        session.updated_at = now;

        sqlx::query(
            "UPDATE sessions SET name = ?, host = ?, port = ?, username = ?, auth_type = ?,
             password = ?, private_key = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(&session.name)
        .bind(&session.host)
        .bind(session.port as i64)
        .bind(&session.username)
        .bind(&session.auth_type)
        .bind(&encrypted_password)
        .bind(&session.private_key)
        .bind(&session.updated_at)
        .bind(&session.id)
        .execute(pool)
        .await?;
    }

    // Return the session with decrypted password for the response
    Ok(session)
}

/// Delete a session by ID
pub async fn delete_session(pool: &SqlitePool, id: &str) -> Result<(), DbError> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
