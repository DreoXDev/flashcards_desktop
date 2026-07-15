# Development

## Prerequisites

- Node.js
- pnpm
- Rust
- Tauri system dependencies for Windows

## Install

```bash
pnpm install
```

## Environment

Copy `.env.example` if you want explicit database configuration.

```env
TURSO_DATABASE_URL=
TURSO_AUTH_TOKEN=
```

If `TURSO_DATABASE_URL` is empty, the app uses a local libSQL database at the OS local app-data path, under a `flashcards_desktop` directory. On Windows this is typically:

```text
%LOCALAPPDATA%\flashcards_desktop\flashcards_desktop.db
```

This keeps database writes outside `src-tauri`, so `pnpm tauri dev` does not rebuild every time the app saves study state.

For a remote Turso database, set both variables. Do not ship a privileged remote token in a multi-user desktop client.

## Run

```bash
pnpm tauri dev
```

## Checks

```bash
pnpm typecheck
pnpm build
cd src-tauri
cargo test
cargo check
```

## Build

```bash
pnpm tauri build
```

Expected Windows bundle names:

```text
src-tauri/target/release/bundle/msi/flashcards_desktop_0.1.0_x64_en-US.msi
src-tauri/target/release/bundle/nsis/flashcards_desktop_0.1.0_x64-setup.exe
```

## App Icon

The source app icon is:

```text
src-tauri/app-icon.png
```

Regenerate the platform icon set after changing it:

```bash
pnpm tauri icon src-tauri/app-icon.png
```

## Troubleshooting

If import fails, inspect the validation details shown in the app. If the database cannot initialize, verify `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN`, or clear them to use the local database.

If a saved session does not appear on the home page, confirm that the deck still exists. Active sessions and favorites are removed automatically when their deck is deleted.

If the app repeatedly closes and reopens during `pnpm tauri dev`, check that no database file is being written inside `src-tauri`. Older development builds used `src-tauri/flashcards_desktop.db`; that file can be deleted after moving any data you still need.
