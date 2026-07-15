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

Vue renders the library, deck detail, and study flow. Pinia stores the deck library list, loading state, import state, and user-visible errors. The study session is local to `StudyView` because v1 does not persist session progress.

## Backend

Rust owns file import, JSON parsing, validation, normalization, CRUD, and database access. Commands are intentionally small:

- `list_decks`
- `get_deck`
- `delete_deck`
- `import_deck_from_file`
- `import_deck_from_json`

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

The default development database is local `piano-codex-flashcards.db` in the working directory. If `TURSO_DATABASE_URL` is set to a remote Turso URL, `TURSO_AUTH_TOKEN` is required and the same database layer is used.

`decks` stores deck metadata. `cards` stores card content and `position`, which preserves the original JSON order. Random sessions shuffle a copy in frontend memory and never change `position`.
