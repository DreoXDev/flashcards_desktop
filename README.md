# flashcards_desktop

Minimal desktop flashcards for focused study.

## Features

- Import JSON decks.
- List imported decks.
- Continue unfinished study sessions after leaving or restarting the app.
- Keep favorite decks near the top of the home page.
- Show recently studied decks.
- Reopen a recent deck or review the unknown cards from the last session.
- Replace an existing deck by re-importing the same `deck.id`.
- Delete decks after confirmation.
- Study in original or random order.
- Navigate freely through a study session with the question sidebar.
- Open cards with bidirectional click or Space flip.
- Open cards use Check / X icon actions for self-assessment and then move to the next unanswered card.
- Closed cards support single-answer and multi-answer selection.
- Closed cards automatically mark known / unknown after answer checking.
- Session summary with known / unknown counts.
- Temporary review sessions for unknown cards.
- Custom borderless Windows-style titlebar.
- Custom flashcard app icon.

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
- Tailwind UI primitives inspired by shadcn-vue
- Turso/libSQL

## Development

```bash
pnpm install
pnpm tauri dev
```

## Turso Configuration

By default the app uses a local libSQL database file named `flashcards_desktop.db` in the OS local app-data directory. On Windows this is typically `%LOCALAPPDATA%\flashcards_desktop\flashcards_desktop.db`. To use Turso, set:

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

The Windows bundles are emitted under:

- `src-tauri/target/release/bundle/msi/flashcards_desktop_0.1.0_x64_en-US.msi`
- `src-tauri/target/release/bundle/nsis/flashcards_desktop_0.1.0_x64-setup.exe`

## Project Scope

This v1 is intentionally small: import, organize, and study flashcards. It stores deck data, unfinished study sessions, favorite decks, and the latest lightweight study history needed for recent decks and unknown-card review.

## Non-goals

No spaced repetition, long-term analytics, streaks, gamification, account system, full deck editor, sync workflow, or scoring.
