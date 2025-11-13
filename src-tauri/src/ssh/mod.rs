use ssh2::Session;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{Emitter, Manager};
use crate::db::SSHSession;

#[derive(Debug, thiserror::Error)]
pub enum SSHError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct SSHConnection {
    session: Session,
    stream: TcpStream,
}

impl SSHConnection {
    pub fn new(config: &SSHSession) -> Result<Self, SSHError> {
        let tcp = TcpStream::connect(format!("{}:{}", config.host, config.port))
            .map_err(|e| SSHError::ConnectionFailed(e.to_string()))?;

        tcp.set_nonblocking(false)
            .map_err(|e| SSHError::ConnectionFailed(e.to_string()))?;

        let mut session = Session::new()
            .map_err(|e| SSHError::ConnectionFailed(e.to_string()))?;

        session.set_tcp_stream(tcp.try_clone().unwrap());
        session.handshake()
            .map_err(|e| SSHError::ConnectionFailed(e.to_string()))?;

        // Authentication
        if config.auth_type == "password" {
            if let Some(password) = &config.password {
                session.userauth_password(&config.username, password)
                    .map_err(|e| SSHError::AuthenticationFailed(e.to_string()))?;
            }
        } else if config.auth_type == "key" {
            if let Some(key_path) = &config.private_key {
                let expanded_path = shellexpand::tilde(key_path).to_string();
                session.userauth_pubkey_file(&config.username, None, std::path::Path::new(&expanded_path), None)
                    .map_err(|e| SSHError::AuthenticationFailed(e.to_string()))?;
            }
        }

        if !session.authenticated() {
            return Err(SSHError::AuthenticationFailed("Not authenticated".to_string()));
        }

        Ok(Self { session, stream: tcp })
    }

    pub fn execute_shell<F>(&mut self, tab_id: String, app_handle: tauri::AppHandle, mut on_disconnect: F)
    where
        F: FnMut() + Send + 'static,
    {
        let mut channel = match self.session.channel_session() {
            Ok(ch) => ch,
            Err(e) => {
                eprintln!("Failed to open channel: {}", e);
                return;
            }
        };

        if let Err(e) = channel.request_pty("xterm-256color", None, None) {
            eprintln!("Failed to request PTY: {}", e);
            return;
        }

        if let Err(e) = channel.shell() {
            eprintln!("Failed to start shell: {}", e);
            return;
        }

        let channel_arc = Arc::new(Mutex::new(channel));
        let channel_read = channel_arc.clone();
        let channel_write = channel_arc.clone();

        // Thread за четене на данни от SSH
        let read_tab_id = tab_id.clone();
        let read_handle = app_handle.clone();
        thread::spawn(move || {
            let mut buffer = [0u8; 8192];
            loop {
                let mut channel = channel_read.lock().unwrap();
                match channel.read(&mut buffer) {
                    Ok(0) => {
                        drop(channel);
                        let _ = read_handle.emit("connection-status", serde_json::json!({
                            "tab_id": read_tab_id,
                            "connected": false,
                        }));
                        break;
                    }
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                        drop(channel);
                        let _ = read_handle.emit("terminal-data", serde_json::json!({
                            "tab_id": read_tab_id,
                            "data": data,
                        }));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        drop(channel);
                        thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        drop(channel);
                        break;
                    }
                }
            }
            on_disconnect();
        });

        // Запазваме channel_write за по-късно използване
        ACTIVE_CHANNELS.lock().unwrap().insert(tab_id, channel_write);
    }
}

lazy_static::lazy_static! {
    static ref ACTIVE_CHANNELS: Mutex<HashMap<String, Arc<Mutex<ssh2::Channel>>>> = Mutex::new(HashMap::new());
}

pub fn send_input(tab_id: &str, data: &str) -> Result<(), SSHError> {
    let channels = ACTIVE_CHANNELS.lock().unwrap();
    if let Some(channel_arc) = channels.get(tab_id) {
        let mut channel = channel_arc.lock().unwrap();
        channel.write_all(data.as_bytes())
            .map_err(|e| SSHError::IoError(e))?;
        channel.flush()
            .map_err(|e| SSHError::IoError(e))?;
        Ok(())
    } else {
        Err(SSHError::ChannelError("Channel not found".to_string()))
    }
}

pub fn close_connection(tab_id: &str) {
    let mut channels = ACTIVE_CHANNELS.lock().unwrap();
    if let Some(channel_arc) = channels.remove(tab_id) {
        let mut channel = channel_arc.lock().unwrap();
        let _ = channel.close();
    }
}
