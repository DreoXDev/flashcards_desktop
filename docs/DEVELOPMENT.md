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

If `TURSO_DATABASE_URL` is empty, the app uses a local libSQL database file named `flashcards_desktop.db`.

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

## Troubleshooting

If import fails, inspect the validation details shown in the app. If the database cannot initialize, verify `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN`, or clear them to use the local database.
