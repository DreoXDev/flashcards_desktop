# flashcards_desktop

Minimal desktop flashcards for focused study.

## Features

- Import JSON decks.
- List imported decks.
- Replace an existing deck by re-importing the same `deck.id`.
- Delete decks.
- Study in original or random order.
- Open cards with click or Space reveal.
- Closed cards with single-answer or multi-answer selection.
- Manual self-assessment with `I didn't know` / `I knew it`.
- Custom borderless Windows-style titlebar.

## Screenshots

Placeholder until the first packaged release.

## Tech Stack

- Tauri 2
- Rust
- Vue 3
- TypeScript
- Vite
- Pinia
- TailwindCSS
- shadcn-vue-compatible primitives
- Turso/libSQL

## Development

```bash
pnpm install
pnpm tauri dev
```

## Turso Configuration

By default the app uses a local libSQL database file in the working directory. To use Turso, set:

```env
TURSO_DATABASE_URL=
TURSO_AUTH_TOKEN=
```

Do not distribute a privileged remote database token inside a client app for a multi-user product.

## Deck Import Format

See [docs/DECK_FORMAT.md](docs/DECK_FORMAT.md).

## Example Decks

Example decks live in [examples](examples):

- `example-open-deck.json`
- `example-closed-deck.json`
- `example-mixed-deck.json`
- `architettura-demo.json`

## Build

```bash
pnpm build
pnpm tauri build
```

## Project Scope

This v1 is intentionally small: import, organize, and study flashcards.

## Non-goals

No spaced repetition, statistics, streaks, gamification, account system, full deck editor, sync workflow, or scoring.
