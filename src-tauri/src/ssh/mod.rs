use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use crate::db::SSHSession;
use russh::*;
use russh::client;
use russh_keys::*;
use tokio::sync::{mpsc, RwLock};
use async_trait::async_trait;
use std::io::Cursor;

#[derive(Debug, thiserror::Error)]
pub enum SSHError {
    #[error("Russh error: {0}")]
    Russh(#[from] russh::Error),
    #[error("Key error: {0}")]
    Key(#[from] russh_keys::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}

// Improved: Use RwLock instead of Mutex for better concurrent read performance
// Multiple tabs can read simultaneously without blocking each other
lazy_static::lazy_static! {
    static ref ACTIVE_SESSIONS: RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>> = RwLock::new(HashMap::new());
}

struct Client;

#[async_trait]
impl client::Handler for Client {
    type Error = SSHError;

    async fn check_server_key(
        self,
        _server_public_key: &key::PublicKey,
    ) -> Result<(Self, bool), Self::Error> {
        Ok((self, true)) // In a real app, you should check the key
    }
}

pub async fn connect(
    config: &SSHSession,
    app_handle: AppHandle,
    tab_id: String,
) -> Result<(), SSHError> {
    // Increased buffer from 100 to 10000 to handle high-throughput SSH sessions
    // This prevents data loss when commands produce rapid output
    let (tx, mut rx) = mpsc::channel(10000);
    ACTIVE_SESSIONS.write().await.insert(tab_id.clone(), tx);

    let client_config = Arc::new(client::Config::default());
    let client_handler = Client;

    let addr = format!("{}:{}", config.host, config.port);
    let mut session = client::connect(client_config, addr, client_handler).await?;

    let auth_result = if config.auth_type == "password" {
        session.authenticate_password(&config.username, config.password.as_deref().unwrap_or("")).await?
    } else {
        let key_path = config.private_key.as_deref().unwrap_or("");
        let expanded_path = shellexpand::tilde(key_path).to_string();
        let key_pair = load_secret_key(expanded_path, None)?;
        session.authenticate_publickey(&config.username, Arc::new(key_pair)).await?
    };

    if !auth_result {
        return Err(SSHError::ConnectionFailed("Authentication failed".to_string()));
    }

    let mut channel = session.channel_open_session().await?;
    channel.request_pty(false, "xterm-256color", 80, 24, 0, 0, &[]).await?;
    channel.request_shell(false).await?;

    let _ = app_handle.emit("connection-status", serde_json::json!({
        "tab_id": tab_id,
        "connected": true,
    }));

    // Main loop for handling input/output
    loop {
        tokio::select! {
            Some(data) = rx.recv() => {
                // Use Cursor to wrap data as AsyncRead
                let cursor = Cursor::new(data);
                channel.data(cursor).await.map_err(|e| SSHError::Russh(e))?;
            }
            result = channel.wait() => {
                match result {
                    Some(msg) => {
                        match msg {
                            ChannelMsg::Data { ref data } => {
                                // Data is handled by the Handler trait
                                let data_str = String::from_utf8_lossy(data).to_string();
                                let _ = app_handle.emit("terminal-data", serde_json::json!({
                                    "tab_id": tab_id,
                                    "data": data_str,
                                }));
                            }
                            ChannelMsg::Eof => {
                                break;
                            }
                            ChannelMsg::Close => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    None => break,
                }
            }
        }
    }

    let _ = app_handle.emit("connection-status", serde_json::json!({
        "tab_id": tab_id,
        "connected": false,
    }));
    close_connection(&tab_id).await;
    Ok(())
}

pub async fn send_input(tab_id: &str, data: &str) -> Result<(), SSHError> {
    // Use read lock for faster lookup - doesn't block other readers
    let tx_opt = ACTIVE_SESSIONS.read().await.get(tab_id).cloned();
    if let Some(tx) = tx_opt {
        tx.send(data.as_bytes().to_vec()).await.map_err(|_| SSHError::ConnectionFailed("Channel closed".to_string()))?;
    }
    Ok(())
}

pub async fn close_connection(tab_id: &str) {
    // Use write lock only when modifying
    ACTIVE_SESSIONS.write().await.remove(tab_id);
}