// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod ssh;

use db::{Database, SSHSession};
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    db: Database,
    connections: Mutex<std::collections::HashMap<String, ()>>,
}

#[tauri::command]
async fn get_sessions(state: State<'_, AppState>) -> Result<Vec<SSHSession>, String> {
    Ok(state.db.get_sessions())
}

#[tauri::command]
async fn save_session(
    state: State<'_, AppState>,
    session: SSHSession,
) -> Result<SSHSession, String> {
    state.db.save_session(session)
}

#[tauri::command]
async fn delete_session(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<(), String> {
    state.db.delete_session(&session_id)
}

#[tauri::command]
async fn connect_ssh(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
    tab_id: String,
    session_id: String,
) -> Result<(), String> {
    let session = state
        .db
        .get_session(&session_id)
        .ok_or("Session not found")?;

    let tab_id_clone = tab_id.clone();
    let app_handle_clone = app_handle.clone();

    // Стартираме SSH връзката в отделна нишка
    std::thread::spawn(move || {
        match ssh::SSHConnection::new(&session) {
            Ok(mut conn) => {
                // Известяваме за успешна връзка
                let _ = app_handle_clone.emit("connection-status", serde_json::json!({
                    "tab_id": tab_id_clone,
                    "connected": true,
                }));

                // Стартираме shell сесията
                conn.execute_shell(tab_id_clone.clone(), app_handle_clone.clone(), move || {
                    println!("SSH connection closed for tab: {}", tab_id_clone);
                });
            }
            Err(e) => {
                eprintln!("SSH connection error: {}", e);
                let _ = app_handle_clone.emit("connection-status", serde_json::json!({
                    "tab_id": tab_id_clone,
                    "connected": false,
                    "error": e.to_string(),
                }));
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn send_terminal_input(
    tab_id: String,
    data: String,
) -> Result<(), String> {
    ssh::send_input(&tab_id, &data).map_err(|e| e.to_string())
}

#[tauri::command]
async fn close_terminal(
    tab_id: String,
) -> Result<(), String> {
    ssh::close_connection(&tab_id);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            let app_state = AppState {
                db: Database::new(),
                connections: Mutex::new(std::collections::HashMap::new()),
            };
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_sessions,
            save_session,
            delete_session,
            connect_ssh,
            send_terminal_input,
            close_terminal,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
