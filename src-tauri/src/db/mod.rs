use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHSession {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_type: String,
    pub password: Option<String>,
    pub private_key: Option<String>,
    pub group: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct Database {
    sessions: Mutex<HashMap<String, SSHSession>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn save_session(&self, mut session: SSHSession) -> Result<SSHSession, String> {
        println!("save_session called with: {:?}", session);
        let mut sessions = self.sessions.lock().unwrap();

        if session.id.is_empty() {
            session.id = Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();
            session.created_at = now.clone();
            session.updated_at = now;
            println!("Created new session with id: {}", session.id);
        } else {
            session.updated_at = chrono::Utc::now().to_rfc3339();
            println!("Updated existing session: {}", session.id);
        }

        sessions.insert(session.id.clone(), session.clone());
        println!("Total sessions in DB: {}", sessions.len());
        Ok(session)
    }

    pub fn get_sessions(&self) -> Vec<SSHSession> {
        let sessions = self.sessions.lock().unwrap();
        let result: Vec<SSHSession> = sessions.values().cloned().collect();
        println!("get_sessions called, returning {} sessions", result.len());
        result
    }

    pub fn get_session(&self, id: &str) -> Option<SSHSession> {
        let sessions = self.sessions.lock().unwrap();
        let result = sessions.get(id).cloned();
        println!("get_session called for id: {}, found: {}", id, result.is_some());
        result
    }

    pub fn delete_session(&self, id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(id);
        Ok(())
    }
}

// За по-напреднала версия с SQLite:
// use tauri_plugin_sql::{Migration, MigrationKind};
//
// pub fn get_migrations() -> Vec<Migration> {
//     vec![
//         Migration {
//             version: 1,
//             description: "create_sessions_table",
//             sql: "CREATE TABLE sessions (
//                 id TEXT PRIMARY KEY,
//                 name TEXT NOT NULL,
//                 host TEXT NOT NULL,
//                 port INTEGER NOT NULL,
//                 username TEXT NOT NULL,
//                 auth_type TEXT NOT NULL,
//                 password TEXT,
//                 private_key TEXT,
//                 group_name TEXT,
//                 created_at TEXT NOT NULL,
//                 updated_at TEXT NOT NULL
//             )",
//             kind: MigrationKind::Up,
//         }
//     ]
// }
