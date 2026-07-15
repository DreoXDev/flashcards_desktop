import { invoke } from '@tauri-apps/api/core'
import type {
  AppErrorPayload,
  ActiveStudySessionDetail,
  ActiveStudySessionSummary,
  DeckDetail,
  DeckSummary,
  FavoriteDeck,
  ImportResult,
  RecentDeck,
  SaveActiveStudySessionRequest,
  SaveStudyHistoryRequest,
} from '@/types/deck'

export function normalizeError(error: unknown): AppErrorPayload {
  if (typeof error === 'object' && error !== null && 'code' in error) {
    return error as AppErrorPayload
  }
  return {
    code: 'UNEXPECTED_ERROR',
    message: 'Something went wrong.',
    details: [String(error)],
  }
}

export const deckApi = {
  listDecks: () => invoke<DeckSummary[]>('list_decks'),
  listRecentDecks: () => invoke<RecentDeck[]>('list_recent_decks'),
  listActiveStudySessions: () =>
    invoke<ActiveStudySessionSummary[]>('list_active_study_sessions'),
  getActiveStudySession: (sessionId: string) =>
    invoke<ActiveStudySessionDetail>('get_active_study_session', { sessionId }),
  listFavoriteDecks: () => invoke<FavoriteDeck[]>('list_favorite_decks'),
  getDeck: (deckId: string) => invoke<DeckDetail>('get_deck', { deckId }),
  deleteDeck: (deckId: string) => invoke<void>('delete_deck', { deckId }),
  importDeckFromFile: (path: string, replaceExisting = false) =>
    invoke<ImportResult>('import_deck_from_file', { path, replaceExisting }),
  saveStudyHistory: (history: SaveStudyHistoryRequest) =>
    invoke<void>('save_study_history', { history }),
  saveActiveStudySession: (session: SaveActiveStudySessionRequest) =>
    invoke<void>('save_active_study_session', { session }),
  deleteActiveStudySession: (sessionId: string) =>
    invoke<void>('delete_active_study_session', { sessionId }),
  setDeckFavorite: (deckId: string, favorite: boolean) =>
    invoke<void>('set_deck_favorite', { deckId, favorite }),
}
