use std::collections::{HashMap, HashSet};

use chrono::Utc;
use libsql::{params, Database};

use crate::{
    error::{AppError, AppResult},
    models::{
        ActiveStudySessionDetail, ActiveStudySessionSummary, DeckDefinition, DeckDetail,
        DeckImport, DeckSummary, FavoriteDeck, Flashcard, FlashcardDefinition, ImportResult,
        RecentDeck, SaveActiveStudySessionRequest, SaveStudyHistoryRequest,
    },
};

pub async fn list_decks(database: &Database) -> AppResult<Vec<DeckSummary>> {
    let conn = database.connect()?;
    let mut rows = conn
        .query(
            "SELECT d.id, d.name, d.description, d.subject, d.tags_json, d.updated_at,
                    COUNT(c.id) AS card_count
             FROM decks d
             LEFT JOIN cards c ON c.deck_id = d.id
             GROUP BY d.id
             ORDER BY d.updated_at DESC, d.name ASC",
            (),
        )
        .await?;
    let mut decks = Vec::new();
    while let Some(row) = rows.next().await? {
        let tags_json: String = row.get(4)?;
        decks.push(DeckSummary {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            subject: row.get(3)?,
            tags: parse_json_vec(&tags_json),
            updated_at: row.get(5)?,
            card_count: row.get(6)?,
        });
    }
    Ok(decks)
}

pub async fn list_recent_decks(database: &Database) -> AppResult<Vec<RecentDeck>> {
    let conn = database.connect()?;
    let mut rows = conn
        .query(
            "SELECT d.id, d.name, d.description, d.subject, d.tags_json,
                    COUNT(c.id) AS card_count,
                    h.last_studied_at, h.last_known_count, h.last_unknown_count,
                    h.last_unknown_card_ids_json
             FROM deck_study_history h
             JOIN decks d ON d.id = h.deck_id
             LEFT JOIN cards c ON c.deck_id = d.id
             GROUP BY d.id
             ORDER BY h.last_studied_at DESC
             LIMIT 5",
            (),
        )
        .await?;
    let mut decks = Vec::new();
    while let Some(row) = rows.next().await? {
        let tags_json: String = row.get(4)?;
        let unknown_json: String = row.get(9)?;
        decks.push(RecentDeck {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            subject: row.get(3)?,
            tags: parse_json_vec(&tags_json),
            card_count: row.get(5)?,
            last_studied_at: row.get(6)?,
            last_known_count: row.get(7)?,
            last_unknown_count: row.get(8)?,
            last_unknown_card_ids: parse_json_vec(&unknown_json),
        });
    }
    Ok(decks)
}

pub async fn list_active_study_sessions(
    database: &Database,
) -> AppResult<Vec<ActiveStudySessionSummary>> {
    let conn = database.connect()?;
    let mut rows = conn
        .query(
            "SELECT s.id, s.deck_id, d.name, s.session_mode, s.study_mode,
                    s.card_ids_json, s.current_index, s.states_json, s.updated_at
             FROM active_study_sessions s
             JOIN decks d ON d.id = s.deck_id
             ORDER BY s.updated_at DESC
             LIMIT 8",
            (),
        )
        .await?;
    let mut sessions = Vec::new();
    while let Some(row) = rows.next().await? {
        let card_ids_json: String = row.get(5)?;
        let states_json: String = row.get(7)?;
        let card_ids = parse_json_vec(&card_ids_json);
        let (answered_count, known_count, unknown_count) = summarize_states_json(&states_json);
        sessions.push(ActiveStudySessionSummary {
            id: row.get(0)?,
            deck_id: row.get(1)?,
            deck_name: row.get(2)?,
            session_mode: row.get(3)?,
            study_mode: row.get(4)?,
            card_count: card_ids.len() as i64,
            answered_count,
            known_count,
            unknown_count,
            current_index: row.get(6)?,
            updated_at: row.get(8)?,
        });
    }
    Ok(sessions)
}

pub async fn get_active_study_session(
    database: &Database,
    session_id: &str,
) -> AppResult<ActiveStudySessionDetail> {
    let conn = database.connect()?;
    let mut rows = conn
        .query(
            "SELECT id, deck_id, session_mode, study_mode, card_ids_json,
                    current_index, states_json, updated_at
             FROM active_study_sessions
             WHERE id = ?1",
            params![session_id],
        )
        .await?;
    let Some(row) = rows.next().await? else {
        return Err(AppError::NotFound(session_id.to_string()));
    };
    let card_ids_json: String = row.get(4)?;
    Ok(ActiveStudySessionDetail {
        id: row.get(0)?,
        deck_id: row.get(1)?,
        session_mode: row.get(2)?,
        study_mode: row.get(3)?,
        card_ids: parse_json_vec(&card_ids_json),
        current_index: row.get(5)?,
        states_json: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

pub async fn list_favorite_decks(database: &Database) -> AppResult<Vec<FavoriteDeck>> {
    let conn = database.connect()?;
    let mut rows = conn
        .query(
            "SELECT d.id, d.name, d.description, d.subject, d.tags_json,
                    COUNT(c.id) AS card_count, f.created_at
             FROM favorite_decks f
             JOIN decks d ON d.id = f.deck_id
             LEFT JOIN cards c ON c.deck_id = d.id
             GROUP BY d.id
             ORDER BY f.created_at DESC",
            (),
        )
        .await?;
    let mut decks = Vec::new();
    while let Some(row) = rows.next().await? {
        let tags_json: String = row.get(4)?;
        decks.push(FavoriteDeck {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            subject: row.get(3)?,
            tags: parse_json_vec(&tags_json),
            card_count: row.get(5)?,
            favorited_at: row.get(6)?,
        });
    }
    Ok(decks)
}

pub async fn get_deck(database: &Database, deck_id: &str) -> AppResult<DeckDetail> {
    let conn = database.connect()?;
    let mut deck_rows = conn
        .query(
            "SELECT id, name, description, subject, tags_json FROM decks WHERE id = ?1",
            params![deck_id],
        )
        .await?;

    let Some(deck_row) = deck_rows.next().await? else {
        return Err(AppError::NotFound(deck_id.to_string()));
    };

    let tags_json: String = deck_row.get(4)?;
    let mut card_rows = conn
        .query(
            "SELECT id, card_type, position, question, answer, options_json,
                    correct_option_ids_json, explanation, tags_json, source, notes
             FROM cards
             WHERE deck_id = ?1
             ORDER BY position ASC",
            params![deck_id],
        )
        .await?;

    let mut cards = Vec::new();
    while let Some(row) = card_rows.next().await? {
        let options_json: Option<String> = row.get(5)?;
        let correct_option_ids_json: Option<String> = row.get(6)?;
        let card_tags_json: String = row.get(8)?;
        cards.push(Flashcard {
            id: row.get(0)?,
            card_type: row.get(1)?,
            position: row.get(2)?,
            question: row.get(3)?,
            answer: row.get(4)?,
            options: options_json
                .as_deref()
                .and_then(|v| serde_json::from_str(v).ok())
                .unwrap_or_default(),
            correct_option_ids: correct_option_ids_json
                .as_deref()
                .map(parse_json_vec)
                .unwrap_or_default(),
            explanation: row.get(7)?,
            tags: parse_json_vec(&card_tags_json),
            source: row.get(9)?,
            notes: row.get(10)?,
        });
    }

    Ok(DeckDetail {
        id: deck_row.get(0)?,
        name: deck_row.get(1)?,
        description: deck_row.get(2)?,
        subject: deck_row.get(3)?,
        tags: parse_json_vec(&tags_json),
        cards,
    })
}

pub async fn delete_deck(database: &Database, deck_id: &str) -> AppResult<()> {
    let conn = database.connect()?;
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;
    let changed = conn
        .execute("DELETE FROM decks WHERE id = ?1", params![deck_id])
        .await?;
    if changed == 0 {
        return Err(AppError::NotFound(deck_id.to_string()));
    }
    Ok(())
}

pub async fn save_study_history(
    database: &Database,
    history: SaveStudyHistoryRequest,
) -> AppResult<()> {
    let conn = database.connect()?;
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;
    let now = Utc::now().to_rfc3339();
    let unknown_json = serde_json::to_string(&history.last_unknown_card_ids)?;
    conn.execute(
        "INSERT INTO deck_study_history
            (deck_id, last_studied_at, last_known_count, last_unknown_count, last_unknown_card_ids_json)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(deck_id) DO UPDATE SET
            last_studied_at = excluded.last_studied_at,
            last_known_count = excluded.last_known_count,
            last_unknown_count = excluded.last_unknown_count,
            last_unknown_card_ids_json = excluded.last_unknown_card_ids_json",
        params![
            history.deck_id,
            now,
            history.last_known_count,
            history.last_unknown_count,
            unknown_json,
        ],
    )
    .await?;
    Ok(())
}

pub async fn save_active_study_session(
    database: &Database,
    session: SaveActiveStudySessionRequest,
) -> AppResult<()> {
    let conn = database.connect()?;
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;
    let now = Utc::now().to_rfc3339();
    let card_ids_json = serde_json::to_string(&session.card_ids)?;
    let mut rows = conn
        .query(
            "SELECT created_at FROM active_study_sessions WHERE id = ?1",
            params![session.id.as_str()],
        )
        .await?;
    let created_at = match rows.next().await? {
        Some(row) => row.get::<String>(0)?,
        None => now.clone(),
    };

    conn.execute(
        "INSERT INTO active_study_sessions
            (id, deck_id, session_mode, study_mode, card_ids_json, current_index,
             states_json, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(id) DO UPDATE SET
            deck_id = excluded.deck_id,
            session_mode = excluded.session_mode,
            study_mode = excluded.study_mode,
            card_ids_json = excluded.card_ids_json,
            current_index = excluded.current_index,
            states_json = excluded.states_json,
            updated_at = excluded.updated_at",
        params![
            session.id,
            session.deck_id,
            session.session_mode,
            session.study_mode,
            card_ids_json,
            session.current_index,
            session.states_json,
            created_at,
            now,
        ],
    )
    .await?;
    Ok(())
}

pub async fn delete_active_study_session(database: &Database, session_id: &str) -> AppResult<()> {
    let conn = database.connect()?;
    conn.execute(
        "DELETE FROM active_study_sessions WHERE id = ?1",
        params![session_id],
    )
    .await?;
    Ok(())
}

pub async fn set_deck_favorite(
    database: &Database,
    deck_id: &str,
    favorite: bool,
) -> AppResult<()> {
    let conn = database.connect()?;
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;
    if favorite {
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO favorite_decks (deck_id, created_at)
             VALUES (?1, ?2)
             ON CONFLICT(deck_id) DO NOTHING",
            params![deck_id, now],
        )
        .await?;
    } else {
        conn.execute(
            "DELETE FROM favorite_decks WHERE deck_id = ?1",
            params![deck_id],
        )
        .await?;
    }
    Ok(())
}

pub async fn import_deck_json(
    database: &Database,
    json: &str,
    replace_existing: bool,
) -> AppResult<ImportResult> {
    let import: DeckImport = serde_json::from_str(json)?;
    validate_import(&import)?;

    let conn = database.connect()?;
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;

    let existing = deck_exists(database, &import.deck.id).await?;
    if existing && !replace_existing {
        return Err(AppError::Conflict(import.deck.id));
    }

    conn.execute("BEGIN IMMEDIATE TRANSACTION", ()).await?;
    let result = insert_deck_in_transaction(&conn, &import, existing).await;
    match result {
        Ok(()) => {
            conn.execute("COMMIT", ()).await?;
            Ok(ImportResult {
                deck_id: import.deck.id,
                deck_name: import.deck.name,
                card_count: import.deck.cards.len(),
                replaced: existing,
            })
        }
        Err(err) => {
            let _ = conn.execute("ROLLBACK", ()).await;
            Err(err)
        }
    }
}

async fn deck_exists(database: &Database, deck_id: &str) -> AppResult<bool> {
    let conn = database.connect()?;
    let mut rows = conn
        .query(
            "SELECT 1 FROM decks WHERE id = ?1 LIMIT 1",
            params![deck_id],
        )
        .await?;
    Ok(rows.next().await?.is_some())
}

async fn insert_deck_in_transaction(
    conn: &libsql::Connection,
    import: &DeckImport,
    existing: bool,
) -> AppResult<()> {
    if existing {
        conn.execute(
            "DELETE FROM decks WHERE id = ?1",
            params![import.deck.id.as_str()],
        )
        .await?;
    }

    let now = Utc::now().to_rfc3339();
    let tags_json = serde_json::to_string(&import.deck.tags)?;
    conn.execute(
        "INSERT INTO decks
            (id, name, description, subject, tags_json, schema_version, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            import.deck.id.as_str(),
            import.deck.name.as_str(),
            import.deck.description.as_deref(),
            import.deck.subject.as_deref(),
            tags_json,
            import.schema_version,
            now.as_str(),
            now.as_str()
        ],
    )
    .await?;

    for (position, card) in import.deck.cards.iter().enumerate() {
        insert_card(conn, &import.deck.id, position as i64, card).await?;
    }

    Ok(())
}

async fn insert_card(
    conn: &libsql::Connection,
    deck_id: &str,
    position: i64,
    card: &FlashcardDefinition,
) -> AppResult<()> {
    let normalized = normalize_card(card)?;
    conn.execute(
        "INSERT INTO cards
            (id, deck_id, card_type, position, question, answer, options_json,
             correct_option_ids_json, explanation, tags_json, source, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            normalized.id,
            deck_id,
            normalized.card_type,
            position,
            normalized.question,
            normalized.answer,
            normalized.options_json,
            normalized.correct_option_ids_json,
            normalized.explanation,
            normalized.tags_json,
            normalized.source,
            normalized.notes,
        ],
    )
    .await?;
    Ok(())
}

struct NormalizedCard {
    id: String,
    card_type: String,
    question: String,
    answer: Option<String>,
    options_json: Option<String>,
    correct_option_ids_json: Option<String>,
    explanation: Option<String>,
    tags_json: String,
    source: Option<String>,
    notes: Option<String>,
}

fn normalize_card(card: &FlashcardDefinition) -> AppResult<NormalizedCard> {
    match card {
        FlashcardDefinition::Open {
            id,
            question,
            answer,
            tags,
            source,
            notes,
        } => Ok(NormalizedCard {
            id: id.trim().to_string(),
            card_type: "open".into(),
            question: question.trim().to_string(),
            answer: Some(answer.trim().to_string()),
            options_json: None,
            correct_option_ids_json: None,
            explanation: None,
            tags_json: serde_json::to_string(tags)?,
            source: source.clone(),
            notes: notes.clone(),
        }),
        FlashcardDefinition::Closed {
            id,
            question,
            options,
            correct_option_ids,
            explanation,
            tags,
            source,
            notes,
        } => Ok(NormalizedCard {
            id: id.trim().to_string(),
            card_type: "closed".into(),
            question: question.trim().to_string(),
            answer: None,
            options_json: Some(serde_json::to_string(options)?),
            correct_option_ids_json: Some(serde_json::to_string(correct_option_ids)?),
            explanation: explanation.clone(),
            tags_json: serde_json::to_string(tags)?,
            source: source.clone(),
            notes: notes.clone(),
        }),
    }
}

fn validate_import(import: &DeckImport) -> AppResult<()> {
    let mut errors = Vec::new();
    if import.schema_version != 1 {
        errors.push(format!(
            "schemaVersion: unsupported version {}. Only version 1 is supported.",
            import.schema_version
        ));
    }
    validate_deck(&import.deck, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(AppError::Validation(errors))
    }
}

fn validate_deck(deck: &DeckDefinition, errors: &mut Vec<String>) {
    if !is_valid_id(&deck.id) {
        errors.push("deck.id: use lowercase letters, numbers, and hyphens.".into());
    }
    if deck.name.trim().is_empty() {
        errors.push("deck.name is required.".into());
    }
    if deck.cards.is_empty() {
        errors.push("deck.cards must contain at least one card.".into());
    }

    let mut card_positions_by_id: HashMap<&str, usize> = HashMap::new();
    for (index, card) in deck.cards.iter().enumerate() {
        let id = card_id(card);
        if id.trim().is_empty() {
            errors.push(format!("cards[{index}].id is required."));
        } else if let Some(first_index) = card_positions_by_id.insert(id, index) {
            errors.push(format!(
                "cards[{index}].id duplicates cards[{first_index}].id (\"{id}\")."
            ));
        } else if !is_valid_id(id) {
            errors.push(format!(
                "cards[{index}].id: use lowercase letters, numbers, and hyphens."
            ));
        }
        validate_card(card, index, errors);
    }
}

fn validate_card(card: &FlashcardDefinition, index: usize, errors: &mut Vec<String>) {
    match card {
        FlashcardDefinition::Open {
            question, answer, ..
        } => {
            if question.trim().is_empty() {
                errors.push(format!("cards[{index}].question is required."));
            }
            if answer.trim().is_empty() {
                errors.push(format!("cards[{index}].answer is required."));
            }
        }
        FlashcardDefinition::Closed {
            question,
            options,
            correct_option_ids,
            ..
        } => {
            if question.trim().is_empty() {
                errors.push(format!("cards[{index}].question is required."));
            }
            if options.len() < 2 {
                errors.push(format!(
                    "cards[{index}].options must contain at least two options."
                ));
            }
            if correct_option_ids.is_empty() {
                errors.push(format!(
                    "cards[{index}].correctOptionIds must contain at least one option id."
                ));
            }

            let mut option_ids = HashSet::new();
            for (option_index, option) in options.iter().enumerate() {
                if option.id.trim().is_empty() {
                    errors.push(format!(
                        "cards[{index}].options[{option_index}].id is required."
                    ));
                }
                if option.text.trim().is_empty() {
                    errors.push(format!(
                        "cards[{index}].options[{option_index}].text is required."
                    ));
                }
                if !option_ids.insert(option.id.as_str()) {
                    errors.push(format!(
                        "cards[{index}].options[{option_index}].id duplicates another option id (\"{}\").",
                        option.id
                    ));
                }
            }

            for (correct_index, correct_id) in correct_option_ids.iter().enumerate() {
                if !option_ids.contains(correct_id.as_str()) {
                    errors.push(format!(
                        "cards[{index}].correctOptionIds[{correct_index}]: option \"{correct_id}\" does not exist in cards[{index}].options."
                    ));
                }
            }
        }
    }
}

fn card_id(card: &FlashcardDefinition) -> &str {
    match card {
        FlashcardDefinition::Open { id, .. } => id,
        FlashcardDefinition::Closed { id, .. } => id,
    }
}

fn is_valid_id(id: &str) -> bool {
    let trimmed = id.trim();
    !trimmed.is_empty()
        && trimmed
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !trimmed.starts_with('-')
        && !trimmed.ends_with('-')
}

fn parse_json_vec(json: &str) -> Vec<String> {
    serde_json::from_str(json).unwrap_or_default()
}

fn summarize_states_json(states_json: &str) -> (i64, i64, i64) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(states_json) else {
        return (0, 0, 0);
    };
    let Some(object) = value.as_object() else {
        return (0, 0, 0);
    };
    let mut answered = 0;
    let mut known = 0;
    let mut unknown = 0;
    for state in object.values() {
        match state.get("result").and_then(|result| result.as_str()) {
            Some("known") => {
                answered += 1;
                known += 1;
            }
            Some("unknown") => {
                answered += 1;
                unknown += 1;
            }
            _ => {}
        }
    }
    (answered, known, unknown)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::run_migrations;
    use libsql::Builder;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn valid_open_json() -> &'static str {
        r#"{
          "schemaVersion": 1,
          "deck": {
            "id": "archi-demo",
            "name": "Archi demo",
            "cards": [
              {"id": "card-001", "type": "open", "question": "Q?", "answer": "A"}
            ]
          }
        }"#
    }

    fn assert_validation_error(json: &str, needle: &str) {
        let import: DeckImport = serde_json::from_str(json).unwrap();
        let err = validate_import(&import).unwrap_err();
        let AppError::Validation(details) = err else {
            panic!("expected validation error");
        };
        assert!(
            details.iter().any(|detail| detail.contains(needle)),
            "{details:?}"
        );
    }

    async fn test_database() -> Database {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("flashcards-desktop-test-{unique}.db"));
        let database = Builder::new_local(path).build().await.unwrap();
        run_migrations(&database).await.unwrap();
        database
    }

    #[test]
    fn valid_open_deck_passes() {
        let import: DeckImport = serde_json::from_str(valid_open_json()).unwrap();
        validate_import(&import).unwrap();
    }

    #[test]
    fn valid_closed_deck_passes() {
        let json = r#"{
          "schemaVersion": 1,
          "deck": {
            "id": "closed-demo",
            "name": "Closed demo",
            "cards": [
              {
                "id": "card-001",
                "type": "closed",
                "question": "Which?",
                "options": [{"id":"a","text":"A"},{"id":"b","text":"B"}],
                "correctOptionIds": ["a"]
              }
            ]
          }
        }"#;
        let import: DeckImport = serde_json::from_str(json).unwrap();
        validate_import(&import).unwrap();
    }

    #[test]
    fn invalid_schema_version_fails() {
        assert_validation_error(
            &valid_open_json().replace("\"schemaVersion\": 1", "\"schemaVersion\": 2"),
            "unsupported version",
        );
    }

    #[test]
    fn missing_answer_fails() {
        assert_validation_error(
            &valid_open_json().replace("\"answer\": \"A\"", "\"answer\": \"\""),
            "answer is required",
        );
    }

    #[test]
    fn duplicate_card_ids_fail() {
        let json = valid_open_json().replace(
            r#"{"id": "card-001", "type": "open", "question": "Q?", "answer": "A"}"#,
            r#"{"id": "card-001", "type": "open", "question": "Q?", "answer": "A"},
              {"id": "card-001", "type": "open", "question": "Q2?", "answer": "A2"}"#,
        );
        assert_validation_error(&json, "duplicates");
    }

    #[test]
    fn duplicate_option_ids_fail() {
        let json = r#"{
          "schemaVersion": 1,
          "deck": {"id":"closed-demo","name":"Closed demo","cards":[{
            "id":"card-001","type":"closed","question":"Q?",
            "options":[{"id":"a","text":"A"},{"id":"a","text":"B"}],
            "correctOptionIds":["a"]
          }]}
        }"#;
        assert_validation_error(json, "duplicates another option id");
    }

    #[test]
    fn missing_correct_option_fails() {
        let json = r#"{
          "schemaVersion": 1,
          "deck": {"id":"closed-demo","name":"Closed demo","cards":[{
            "id":"card-001","type":"closed","question":"Q?",
            "options":[{"id":"a","text":"A"},{"id":"b","text":"B"}],
            "correctOptionIds":[]
          }]}
        }"#;
        assert_validation_error(json, "correctOptionIds must contain");
    }

    #[test]
    fn unknown_correct_option_fails() {
        let json = r#"{
          "schemaVersion": 1,
          "deck": {"id":"closed-demo","name":"Closed demo","cards":[{
            "id":"card-001","type":"closed","question":"Q?",
            "options":[{"id":"a","text":"A"},{"id":"b","text":"B"}],
            "correctOptionIds":["d"]
          }]}
        }"#;
        assert_validation_error(json, "does not exist");
    }

    #[test]
    fn empty_cards_array_fails() {
        let json = r#"{"schemaVersion":1,"deck":{"id":"empty-demo","name":"Empty","cards":[]}}"#;
        assert_validation_error(json, "at least one card");
    }

    #[tokio::test]
    async fn import_lists_and_loads_deck() {
        let database = test_database().await;
        let result = import_deck_json(&database, valid_open_json(), false)
            .await
            .unwrap();
        assert_eq!(result.deck_id, "archi-demo");
        assert_eq!(result.card_count, 1);

        let decks = list_decks(&database).await.unwrap();
        assert_eq!(decks.len(), 1);
        assert_eq!(decks[0].card_count, 1);

        let detail = get_deck(&database, "archi-demo").await.unwrap();
        assert_eq!(detail.cards.len(), 1);
        assert_eq!(detail.cards[0].position, 0);
    }

    #[tokio::test]
    async fn import_replace_deck_is_transactional_shape() {
        let database = test_database().await;
        import_deck_json(&database, valid_open_json(), false)
            .await
            .unwrap();

        let conflict = import_deck_json(&database, valid_open_json(), false)
            .await
            .unwrap_err();
        assert!(matches!(conflict, AppError::Conflict(_)));

        let replacement = valid_open_json()
            .replace(
                "\"name\": \"Archi demo\"",
                "\"name\": \"Archi replacement\"",
            )
            .replace(
                r#"{"id": "card-001", "type": "open", "question": "Q?", "answer": "A"}"#,
                r#"{"id": "card-002", "type": "open", "question": "Q2?", "answer": "A2"}"#,
            );
        let result = import_deck_json(&database, &replacement, true)
            .await
            .unwrap();
        assert!(result.replaced);

        let detail = get_deck(&database, "archi-demo").await.unwrap();
        assert_eq!(detail.name, "Archi replacement");
        assert_eq!(detail.cards[0].id, "card-002");
    }

    #[tokio::test]
    async fn save_and_list_recent_decks() {
        let database = test_database().await;
        import_deck_json(&database, valid_open_json(), false)
            .await
            .unwrap();
        save_study_history(
            &database,
            SaveStudyHistoryRequest {
                deck_id: "archi-demo".into(),
                last_known_count: 0,
                last_unknown_count: 1,
                last_unknown_card_ids: vec!["card-001".into()],
            },
        )
        .await
        .unwrap();

        let recent = list_recent_decks(&database).await.unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].last_unknown_card_ids, vec!["card-001"]);
    }

    #[tokio::test]
    async fn save_list_and_load_active_session() {
        let database = test_database().await;
        import_deck_json(&database, valid_open_json(), false)
            .await
            .unwrap();
        save_active_study_session(
            &database,
            SaveActiveStudySessionRequest {
                id: "archi-demo__full-deck__full".into(),
                deck_id: "archi-demo".into(),
                session_mode: "full-deck".into(),
                study_mode: "original".into(),
                card_ids: vec!["card-001".into()],
                current_index: 0,
                states_json: r#"{"card-001":{"cardId":"card-001","result":"known","visited":true,"answerVisible":true,"selectedOptionIds":[]}}"#.into(),
            },
        )
        .await
        .unwrap();

        let sessions = list_active_study_sessions(&database).await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].answered_count, 1);
        assert_eq!(sessions[0].known_count, 1);

        let detail = get_active_study_session(&database, "archi-demo__full-deck__full")
            .await
            .unwrap();
        assert_eq!(detail.card_ids, vec!["card-001"]);

        delete_active_study_session(&database, "archi-demo__full-deck__full")
            .await
            .unwrap();
        assert!(list_active_study_sessions(&database)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn favorite_decks_can_be_toggled() {
        let database = test_database().await;
        import_deck_json(&database, valid_open_json(), false)
            .await
            .unwrap();

        set_deck_favorite(&database, "archi-demo", true)
            .await
            .unwrap();
        let favorites = list_favorite_decks(&database).await.unwrap();
        assert_eq!(favorites.len(), 1);
        assert_eq!(favorites[0].id, "archi-demo");

        set_deck_favorite(&database, "archi-demo", false)
            .await
            .unwrap();
        assert!(list_favorite_decks(&database).await.unwrap().is_empty());
    }
}
