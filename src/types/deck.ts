export interface AppErrorPayload {
  code: string
  message: string
  details: string[]
}

export interface DeckSummary {
  id: string
  name: string
  description?: string | null
  subject?: string | null
  tags: string[]
  cardCount: number
  updatedAt: string
}

export interface FavoriteDeck {
  id: string
  name: string
  description?: string | null
  subject?: string | null
  tags: string[]
  cardCount: number
  favoritedAt: string
}

export interface DeckDetail {
  id: string
  name: string
  description?: string | null
  subject?: string | null
  tags: string[]
  cards: Flashcard[]
}

export interface Flashcard {
  id: string
  cardType: 'open' | 'closed'
  position: number
  question: string
  answer?: string | null
  options: ClosedCardOption[]
  correctOptionIds: string[]
  explanation?: string | null
  tags: string[]
  source?: string | null
  notes?: string | null
}

export interface ClosedCardOption {
  id: string
  text: string
}

export interface ImportResult {
  deckId: string
  deckName: string
  cardCount: number
  replaced: boolean
}

export type StudyMode = 'original' | 'random'
export type StudySessionMode = 'full-deck' | 'review-unknown'
export type StudyResult = 'known' | 'unknown' | null

export interface RecentDeck {
  id: string
  name: string
  description?: string | null
  subject?: string | null
  tags: string[]
  cardCount: number
  lastStudiedAt: string
  lastKnownCount: number
  lastUnknownCount: number
  lastUnknownCardIds: string[]
}

export interface SaveStudyHistoryRequest {
  deckId: string
  lastKnownCount: number
  lastUnknownCount: number
  lastUnknownCardIds: string[]
}

export interface CardStudyState {
  cardId: string
  result: StudyResult
  visited: boolean
  answerVisible: boolean
  selectedOptionIds: string[]
}

export interface ActiveStudySessionSummary {
  id: string
  deckId: string
  deckName: string
  sessionMode: StudySessionMode
  studyMode: StudyMode
  cardCount: number
  answeredCount: number
  knownCount: number
  unknownCount: number
  currentIndex: number
  updatedAt: string
}

export interface ActiveStudySessionDetail {
  id: string
  deckId: string
  sessionMode: StudySessionMode
  studyMode: StudyMode
  cardIds: string[]
  currentIndex: number
  statesJson: string
  updatedAt: string
}

export interface SaveActiveStudySessionRequest {
  id: string
  deckId: string
  sessionMode: StudySessionMode
  studyMode: StudyMode
  cardIds: string[]
  currentIndex: number
  statesJson: string
}
