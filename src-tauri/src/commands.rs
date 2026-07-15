use std::fs;

use tauri::State;

use crate::{
    db::AppState,
    error::AppResult,
    models::{
        ActiveStudySessionDetail, ActiveStudySessionSummary, DeckDetail, DeckSummary, FavoriteDeck,
        ImportResult, RecentDeck, SaveActiveStudySessionRequest, SaveStudyHistoryRequest,
    },
    services,
};

#[tauri::command]
pub async fn list_decks(state: State<'_, AppState>) -> AppResult<Vec<DeckSummary>> {
    services::list_decks(&state.database).await
}

#[tauri::command]
pub async fn list_recent_decks(state: State<'_, AppState>) -> AppResult<Vec<RecentDeck>> {
    services::list_recent_decks(&state.database).await
}

#[tauri::command]
pub async fn list_active_study_sessions(
    state: State<'_, AppState>,
) -> AppResult<Vec<ActiveStudySessionSummary>> {
    services::list_active_study_sessions(&state.database).await
}

#[tauri::command]
pub async fn get_active_study_session(
    state: State<'_, AppState>,
    session_id: String,
) -> AppResult<ActiveStudySessionDetail> {
    services::get_active_study_session(&state.database, &session_id).await
}

#[tauri::command]
pub async fn list_favorite_decks(state: State<'_, AppState>) -> AppResult<Vec<FavoriteDeck>> {
    services::list_favorite_decks(&state.database).await
}

#[tauri::command]
pub async fn get_deck(state: State<'_, AppState>, deck_id: String) -> AppResult<DeckDetail> {
    services::get_deck(&state.database, &deck_id).await
}

#[tauri::command]
pub async fn delete_deck(state: State<'_, AppState>, deck_id: String) -> AppResult<()> {
    services::delete_deck(&state.database, &deck_id).await
}

#[tauri::command]
pub async fn import_deck_from_file(
    state: State<'_, AppState>,
    path: String,
    replace_existing: bool,
) -> AppResult<ImportResult> {
    let json = fs::read_to_string(path)?;
    services::import_deck_json(&state.database, &json, replace_existing).await
}

#[tauri::command]
pub async fn import_deck_from_json(
    state: State<'_, AppState>,
    json: String,
    replace_existing: bool,
) -> AppResult<ImportResult> {
    services::import_deck_json(&state.database, &json, replace_existing).await
}

#[tauri::command]
pub async fn save_study_history(
    state: State<'_, AppState>,
    history: SaveStudyHistoryRequest,
) -> AppResult<()> {
    services::save_study_history(&state.database, history).await
}

#[tauri::command]
pub async fn save_active_study_session(
    state: State<'_, AppState>,
    session: SaveActiveStudySessionRequest,
) -> AppResult<()> {
    services::save_active_study_session(&state.database, session).await
}

#[tauri::command]
pub async fn delete_active_study_session(
    state: State<'_, AppState>,
    session_id: String,
) -> AppResult<()> {
    services::delete_active_study_session(&state.database, &session_id).await
}

#[tauri::command]
pub async fn set_deck_favorite(
    state: State<'_, AppState>,
    deck_id: String,
    favorite: bool,
) -> AppResult<()> {
    services::set_deck_favorite(&state.database, &deck_id, favorite).await
}
