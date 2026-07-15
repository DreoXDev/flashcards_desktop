# Architecture

The app keeps persistence and validation in Rust while Vue handles rendering and study-session state.

```text
Vue UI
  -> Tauri invoke
  -> Rust commands
  -> services
  -> libSQL/Turso database
```

## Frontend

Vue renders the library, unfinished sessions, favorite decks, recently completed decks, deck detail, and study flow. Pinia stores the deck library list, active session summaries, favorite deck list, recent deck list, loading state, import state, and user-visible errors.

`StudyView` owns the active session state. It keeps a single per-card state model:

```ts
type StudyResult = 'known' | 'unknown' | null

interface CardStudyState {
  cardId: string
  result: StudyResult
  visited: boolean
  answerVisible: boolean
  selectedOptionIds: string[]
}
```

This state supports non-linear navigation, answer locking for closed cards, bidirectional open-card flipping, and temporary unknown-card review sessions. Active session progress is saved to Rust while the user studies, so sessions can be resumed after leaving the view or restarting the app.

## Backend

Rust owns file import, JSON parsing, validation, normalization, CRUD, and database access. Commands are intentionally small:

- `list_decks`
- `list_recent_decks`
- `list_active_study_sessions`
- `get_active_study_session`
- `list_favorite_decks`
- `get_deck`
- `delete_deck`
- `import_deck_from_file`
- `import_deck_from_json`
- `save_study_history`
- `save_active_study_session`
- `delete_active_study_session`
- `set_deck_favorite`

## Import Pipeline

```text
select file
-> read JSON
-> parse
-> validate
-> check existing deck id
-> optionally replace
-> insert deck and cards in one transaction
```

## Database

The default development database is local `flashcards_desktop.db` under the OS local app-data directory, inside a `flashcards_desktop` folder. If `TURSO_DATABASE_URL` is set to a remote Turso URL, `TURSO_AUTH_TOKEN` is required and the same database layer is used.

The local database is deliberately outside `src-tauri` because Tauri dev watches that directory and restarts the app when files inside it change.

`decks` stores deck metadata. `cards` stores card content and `position`, which preserves the original JSON order. Random sessions shuffle a copy in frontend memory and never change `position`.

`deck_study_history` stores one lightweight recent-history row per deck:

- `deck_id`
- `last_studied_at`
- `last_known_count`
- `last_unknown_count`
- `last_unknown_card_ids_json`

The history table is used only for the home page recent deck section and quick unknown-card review. It is deleted automatically when a deck is deleted because it references `decks(id)` with `ON DELETE CASCADE`.

`active_study_sessions` stores unfinished sessions:

- `id`
- `deck_id`
- `session_mode`
- `study_mode`
- `card_ids_json`
- `current_index`
- `states_json`
- `created_at`
- `updated_at`

The frontend saves the same per-card state it uses to render the current session. Completed sessions are removed from this table and summarized into `deck_study_history`.

`favorite_decks` stores deck ids pinned by the user. It is also cascade-deleted with the deck.

## Study Flow

Full-deck sessions use all cards in the deck. Review sessions use a temporary subset of card ids, usually the unknown ids saved from the last completed session. Missing card ids are ignored by the frontend when creating the review subset.

A session is complete when every card in the active session has `result !== null`. The summary writes the latest study history to Rust through `save_study_history`.

Open cards:

- click or Space flips question and answer in both directions;
- Check marks known;
- X marks unknown;
- Check or X advances to the next unanswered card when possible;
- the result can be changed while revisiting the card.

Closed cards:

- selected answers are locked for the current session;
- correct answers mark known;
- incorrect answers mark unknown;
- correct options are shown green and wrong selections red.
