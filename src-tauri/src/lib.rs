mod commands;
mod db;
mod error;
mod models;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database =
        tauri::async_runtime::block_on(db::init_database()).expect("failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(db::AppState { database })
        .invoke_handler(tauri::generate_handler![
            commands::list_decks,
            commands::list_recent_decks,
            commands::list_active_study_sessions,
            commands::get_active_study_session,
            commands::list_favorite_decks,
            commands::get_deck,
            commands::delete_deck,
            commands::import_deck_from_file,
            commands::import_deck_from_json,
            commands::save_study_history,
            commands::save_active_study_session,
            commands::delete_active_study_session,
            commands::set_deck_favorite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
