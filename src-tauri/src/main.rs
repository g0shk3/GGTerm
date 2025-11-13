// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod ssh;
mod encryption;

use db::SSHSession;
use db::async_db;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, State};

// New async-friendly state using SQLx connection pool
pub struct DbState(pub SqlitePool);

#[tauri::command]
async fn get_sessions(db_state: State<'_, DbState>) -> Result<Vec<SSHSession>, String> {
    async_db::get_sessions(&db_state.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_session(
    db_state: State<'_, DbState>,
    session: SSHSession,
) -> Result<SSHSession, String> {
    async_db::save_session(&db_state.0, session)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_session(db_state: State<'_, DbState>, session_id: String) -> Result<(), String> {
    async_db::delete_session(&db_state.0, &session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn connect_ssh(
    db_state: State<'_, DbState>,
    app_handle: AppHandle,
    tab_id: String,
    session_id: String,
) -> Result<(), String> {
    println!("Connecting SSH for tab {} with session {}", tab_id, session_id);

    // Use async database call - no blocking!
    let session = async_db::get_session(&db_state.0, &session_id)
        .await
        .map_err(|e| e.to_string())?;

    println!("Found session: {}@{}:{}", session.username, session.host, session.port);

    // Spawn a tokio task for the long-running SSH connection
    tokio::spawn(async move {
        if let Err(e) = ssh::connect(&session, app_handle, tab_id).await {
            eprintln!("SSH connection task failed: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
async fn send_terminal_input(tab_id: String, data: String) -> Result<(), String> {
    ssh::send_input(&tab_id, &data).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn close_terminal(tab_id: String) -> Result<(), String> {
    ssh::close_connection(&tab_id).await;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let app_data_dir = handle.path().app_data_dir().expect("Failed to get app data dir");
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
            }
            let db_path = app_data_dir.join("ggterm.db");
            let db_url = format!("sqlite://{}", db_path.display());

            // Initialize async database with connection pool
            // This is a blocking operation, so we use block_on
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            let pool = runtime.block_on(async {
                async_db::init_db(&db_url)
                    .await
                    .expect("Failed to initialize database")
            });

            app.manage(DbState(pool));
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
