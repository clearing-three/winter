mod db;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use tauri::{Manager, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_updater::UpdaterExt;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle();
            let pool = tauri::async_runtime::block_on(setup_database(app_handle));
            app_handle.manage(pool);

            spawn_update_checker(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_entry, list_entries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

#[tauri::command]
async fn create_entry(text: String, pool: State<'_, Pool<Sqlite>>) -> Result<(), AppError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(AppError::Validation(
            "Entry text cannot be empty".to_string(),
        ));
    }

    db::insert_entry(&pool, trimmed).await?;
    Ok(())
}

#[tauri::command]
async fn list_entries(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<db::Entry>, AppError> {
    let entries = db::get_all_entries(&pool).await?;
    Ok(entries)
}

async fn setup_database(_app_handle: &tauri::AppHandle) -> Pool<Sqlite> {
    #[cfg(debug_assertions)]
    let db_path = {
        let exe_dir = std::env::current_exe()
            .unwrap_or_else(|e| panic!("Failed to get executable path: {}", e))
            .parent()
            .unwrap_or_else(|| panic!("Failed to get parent directory of executable"))
            .to_path_buf();
        exe_dir.join("dev.db")
    };

    #[cfg(not(debug_assertions))]
    let db_path = {
        app_handle
            .path()
            .app_data_dir()
            .unwrap_or_else(|e| panic!("Failed to get app data directory: {}", e))
            .join("app.db")
    };

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|e| {
            panic!(
                "Failed to create database directory at {}: {}",
                parent.display(),
                e
            )
        });
    }

    let connection_string = format!("sqlite:{}?mode=rwc", db_path.display());
    let options = SqliteConnectOptions::from_str(&connection_string)
        .unwrap_or_else(|e| {
            panic!(
                "Failed to parse connection string for {}: {}",
                db_path.display(),
                e
            )
        })
        .journal_mode(SqliteJournalMode::Wal);
    let pool = SqlitePool::connect_with(options)
        .await
        .unwrap_or_else(|e| panic!("Failed to create pool for {}: {}", db_path.display(), e));
    sqlx::migrate!()
        .run(&pool)
        .await
        .unwrap_or_else(|e| panic!("Failed to run migrations: {}", e));
    pool
}

fn spawn_update_checker(handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        match handle.updater() {
            Ok(updater) => {
                if let Ok(Some(update)) = updater.check().await {
                    let answer = handle
                        .dialog()
                        .message(format!(
                            "Version {} is available. Install now?",
                            update.version
                        ))
                        .title("Update Available")
                        .kind(MessageDialogKind::Info)
                        .blocking_show();

                    if answer
                        && let Err(e) = update
                            .download_and_install(|_chunk_size, _content_length| {}, || {})
                            .await
                    {
                        eprintln!("Failed to install update: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to get updater: {}", e);
            }
        }
    });
}
