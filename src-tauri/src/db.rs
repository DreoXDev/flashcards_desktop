use std::{env, path::PathBuf};

use libsql::{Builder, Database};

use crate::error::{AppError, AppResult};

pub struct AppState {
    pub database: Database,
}

pub async fn init_database() -> AppResult<Database> {
    let database = match env::var("TURSO_DATABASE_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
    {
        Some(url) if url.starts_with("libsql://") || url.starts_with("https://") => {
            let token = env::var("TURSO_AUTH_TOKEN").map_err(|_| {
                AppError::Configuration(
                    "TURSO_AUTH_TOKEN is required for remote Turso URLs.".into(),
                )
            })?;
            Builder::new_remote(url, token).build().await?
        }
        Some(path_or_url) => {
            Builder::new_local(PathBuf::from(path_or_url))
                .build()
                .await?
        }
        None => Builder::new_local(default_database_path()?).build().await?,
    };

    run_migrations(&database).await?;
    Ok(database)
}

fn default_database_path() -> AppResult<PathBuf> {
    let mut path = dirs::data_local_dir().unwrap_or_else(std::env::temp_dir);
    path.push("flashcards_desktop");
    std::fs::create_dir_all(&path)?;
    path.push("flashcards_desktop.db");
    Ok(path)
}

pub(crate) async fn run_migrations(database: &Database) -> AppResult<()> {
    let conn = database.connect()?;
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS decks (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            subject TEXT,
            tags_json TEXT NOT NULL DEFAULT '[]',
            schema_version INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )
    .await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            id TEXT NOT NULL,
            deck_id TEXT NOT NULL,
            card_type TEXT NOT NULL,
            position INTEGER NOT NULL,
            question TEXT NOT NULL,
            answer TEXT,
            options_json TEXT,
            correct_option_ids_json TEXT,
            explanation TEXT,
            tags_json TEXT NOT NULL DEFAULT '[]',
            source TEXT,
            notes TEXT,
            PRIMARY KEY (deck_id, id),
            FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE
        )",
        (),
    )
    .await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cards_deck_position ON cards(deck_id, position)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deck_study_history (
            deck_id TEXT PRIMARY KEY,
            last_studied_at TEXT NOT NULL,
            last_known_count INTEGER NOT NULL,
            last_unknown_count INTEGER NOT NULL,
            last_unknown_card_ids_json TEXT NOT NULL DEFAULT '[]',
            FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE
        )",
        (),
    )
    .await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS active_study_sessions (
            id TEXT PRIMARY KEY,
            deck_id TEXT NOT NULL,
            session_mode TEXT NOT NULL,
            study_mode TEXT NOT NULL,
            card_ids_json TEXT NOT NULL,
            current_index INTEGER NOT NULL,
            states_json TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE
        )",
        (),
    )
    .await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_active_sessions_updated_at
         ON active_study_sessions(updated_at)",
        (),
    )
    .await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS favorite_decks (
            deck_id TEXT PRIMARY KEY,
            created_at TEXT NOT NULL,
            FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE
        )",
        (),
    )
    .await?;
    Ok(())
}
