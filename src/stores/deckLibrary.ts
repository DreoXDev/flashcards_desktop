import { defineStore } from 'pinia'
import { open } from '@tauri-apps/plugin-dialog'
import { toast } from 'vue-sonner'
import { deckApi, normalizeError } from '@/lib/api'
import type { AppErrorPayload, DeckSummary, RecentDeck } from '@/types/deck'

export const useDeckLibraryStore = defineStore('deckLibrary', {
  state: () => ({
    decks: [] as DeckSummary[],
    recentDecks: [] as RecentDeck[],
    loading: false,
    importing: false,
    error: null as AppErrorPayload | null,
  }),
  actions: {
    async loadDecks() {
      this.loading = true
      this.error = null
      try {
        this.decks = await deckApi.listDecks()
        this.recentDecks = await deckApi.listRecentDecks()
      } catch (error) {
        this.error = normalizeError(error)
        toast.error(this.error.message)
      } finally {
        this.loading = false
      }
    },
    async importDeck() {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'JSON deck', extensions: ['json'] }],
      })
      if (typeof selected !== 'string') return

      this.importing = true
      try {
        const result = await deckApi.importDeckFromFile(selected, false)
        toast.success('Deck imported', {
          description: `${result.cardCount} cards added`,
        })
        await this.loadDecks()
      } catch (error) {
        const payload = normalizeError(error)
        if (payload.code === 'DECK_CONFLICT') {
          const replace = window.confirm(
            'A deck with this ID already exists.\n\nReplace existing deck?'
          )
          if (replace) {
            const result = await deckApi.importDeckFromFile(selected, true)
            toast.success('Deck replaced', {
              description: `${result.cardCount} cards imported`,
            })
            await this.loadDecks()
          }
        } else {
          this.error = payload
          toast.error(payload.message, {
            description: payload.details.slice(0, 2).join('\n'),
          })
        }
      } finally {
        this.importing = false
      }
    },
    async deleteDeck(deck: DeckSummary | RecentDeck) {
      try {
        await deckApi.deleteDeck(deck.id)
        toast.success('Deck deleted')
        await this.loadDecks()
      } catch (error) {
        const payload = normalizeError(error)
        toast.error(payload.message)
      }
    },
  },
})
