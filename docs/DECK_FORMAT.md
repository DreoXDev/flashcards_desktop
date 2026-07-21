# Deck Format

flashcards_desktop imports versioned JSON decks. The format is meant to be easy to read, easy to validate, stable in Git, and friendly to generation by an AI coding agent.

## Schema Version

Only `schemaVersion: 1` is supported.

## Root Shape

```json
{
  "schemaVersion": 1,
  "deck": {
    "id": "archi-datapath",
    "name": "Architettura - Datapath",
    "description": "Flashcard sul datapath MIPS.",
    "subject": "Architettura degli Elaboratori",
    "tags": ["architettura", "mips"],
    "cards": [
      {
        "id": "datapath-alu-001",
        "type": "open",
        "question": "Qual e il ruolo della ALU nel datapath?",
        "answer": "La ALU esegue operazioni aritmetiche e logiche."
      }
    ]
  }
}
```

## Deck Fields

Required: `id`, `name`, `cards`.

Optional: `description`, `subject`, `tags`.

`cards` must contain at least one card.

## Open Card

```json
{
  "id": "datapath-alu-001",
  "type": "open",
  "question": "Qual e il ruolo della ALU nel datapath?",
  "answer": "La ALU esegue operazioni aritmetiche e logiche.",
  "tags": ["alu"],
  "source": "03_notes/circuiti/09_alu_intro.md",
  "notes": "Optional private note"
}
```

Required: `id`, `type`, `question`, `answer`.

Optional: `tags`, `source`, `notes`.

Open-card answers can use simple Markdown for cleaner display:

- `**bold**`
- `*italic*`
- `` `inline code` ``
- bullet lists with `- item`
- links like `[label](https://example.com)`

## Closed Card

```json
{
  "id": "datapath-mux-001",
  "type": "closed",
  "question": "Quale componente seleziona uno tra piu ingressi?",
  "options": [
    { "id": "a", "text": "Multiplexer" },
    { "id": "b", "text": "Register file" }
  ],
  "correctOptionIds": ["a"],
  "explanation": "Il multiplexer seleziona un ingresso in base ai segnali di controllo."
}
```

Required: `id`, `type`, `question`, `options`, `correctOptionIds`.

Optional: `explanation`, `tags`, `source`, `notes`.

`correctOptionIds` can contain more than one option id.

Use `explanation` to explain why the correct answer is correct. It is shown after the user answers, especially useful when they choose a wrong option.

## Validation Rules

The Rust backend validates imports before writing to the database:

- `schemaVersion` must be `1`.
- `deck.id` must be present and valid.
- `deck.name` must not be empty.
- `cards` must contain at least one card.
- Card ids must be unique within the deck.
- Open cards need non-empty `question` and `answer`.
- Closed cards need a non-empty `question`.
- Closed cards need at least two options.
- Closed option ids must be unique.
- Closed cards need at least one `correctOptionIds` entry.
- Every correct option id must exist in `options`.

## ID Rules

Use stable lowercase kebab-case ids:

- `deck.id`: `archi-datapath`
- `card.id`: `datapath-alu-001`
- `option.id`: `a`, `b`, `c`, `d`

Do not make ids depend only on position in the file. The app stores original order separately.

## Common Errors

Invalid:

```json
{ "schemaVersion": 2 }
```

Invalid:

```json
{
  "id": "bad-card",
  "type": "closed",
  "question": "Q",
  "options": [{ "id": "a", "text": "A" }],
  "correctOptionIds": ["b"]
}
```

The closed card has fewer than two options and points to a missing correct option.

## Generating Decks With An AI Coding Agent

- Output valid JSON only, with no Markdown around it.
- Do not invent facts not present in the source.
- Put one main idea in each card.
- Write clear, non-ambiguous questions.
- Keep open answers concise but complete.
- Use plausible distractors for closed cards.
- Avoid absurd distractors.
- Keep all ids unique.
- Add `source` when a source file is known.
- Split large topics into multiple decks.
