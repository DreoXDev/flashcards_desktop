import { invoke } from '@tauri-apps/api/core'
import type {
  AppErrorPayload,
  DeckDetail,
  DeckSummary,
  ImportResult,
  RecentDeck,
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
  getDeck: (deckId: string) => invoke<DeckDetail>('get_deck', { deckId }),
  deleteDeck: (deckId: string) => invoke<void>('delete_deck', { deckId }),
  importDeckFromFile: (path: string, replaceExisting = false) =>
    invoke<ImportResult>('import_deck_from_file', { path, replaceExisting }),
  saveStudyHistory: (history: SaveStudyHistoryRequest) =>
    invoke<void>('save_study_history', { history }),
}
