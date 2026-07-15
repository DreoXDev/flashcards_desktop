<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { BookOpen, FilePlus2, RotateCcw, Trash2 } from 'lucide-vue-next'
import BaseButton from '@/components/ui/BaseButton.vue'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'
import { useDeckLibraryStore } from '@/stores/deckLibrary'
import type { DeckSummary, RecentDeck } from '@/types/deck'

const library = useDeckLibraryStore()
const router = useRouter()
const deckPendingDelete = ref<DeckSummary | RecentDeck | null>(null)

const hasDecks = computed(() => library.decks.length > 0)

onMounted(() => {
  void library.loadDecks()
})

function openDeck(deckId: string) {
  void router.push({ name: 'deck-detail', params: { deckId } })
}

function openDeckFromKeyboard(event: KeyboardEvent, deckId: string) {
  if (event.key !== 'Enter' && event.code !== 'Space') return
  event.preventDefault()
  openDeck(deckId)
}

function reviewUnknown(event: MouseEvent, deck: RecentDeck) {
  event.stopPropagation()
  if (deck.lastUnknownCardIds.length === 0) return
  void router.push({
    name: 'study',
    params: { deckId: deck.id },
    query: {
      mode: 'original',
      session: 'review-unknown',
      cards: deck.lastUnknownCardIds.join(','),
    },
  })
}

async function confirmDelete() {
  if (!deckPendingDelete.value) return
  await library.deleteDeck(deckPendingDelete.value)
  deckPendingDelete.value = null
}
</script>

<template>
  <section class="h-full overflow-auto px-8 py-8">
    <div class="mx-auto flex w-full max-w-6xl flex-col gap-8">
      <div class="flex items-start justify-between gap-6">
        <div>
          <h1 class="text-3xl font-bold tracking-normal">Flashcards</h1>
          <p class="mt-2 text-sm text-muted-foreground">Study without distractions.</p>
        </div>
        <BaseButton variant="primary" :disabled="library.importing" @click="library.importDeck">
          <FilePlus2 class="h-4 w-4" />
          {{ library.importing ? 'Importing...' : 'Import deck' }}
        </BaseButton>
      </div>

      <div v-if="library.loading" class="rounded-lg border border-border bg-card p-8 text-sm text-muted-foreground">
        Loading decks...
      </div>

      <div
        v-else-if="!hasDecks"
        class="flex min-h-[360px] flex-col items-center justify-center rounded-lg border border-dashed border-border bg-card/60 p-8 text-center"
      >
        <BookOpen class="h-10 w-10 text-primary" />
        <h2 class="mt-5 text-xl font-semibold">No decks yet</h2>
        <p class="mt-2 max-w-sm text-sm leading-6 text-muted-foreground">
          Import a JSON deck to start studying.
        </p>
        <BaseButton class="mt-6" variant="primary" @click="library.importDeck">
          <FilePlus2 class="h-4 w-4" />
          Import deck
        </BaseButton>
      </div>

      <template v-else>
        <section v-if="library.recentDecks.length > 0" class="flex flex-col gap-3">
          <h2 class="text-sm font-bold uppercase tracking-wider text-muted-foreground">Recent decks</h2>
          <div class="grid grid-cols-1 gap-3 md:grid-cols-2 xl:grid-cols-3">
            <article
              v-for="deck in library.recentDecks"
              :key="deck.id"
              class="group flex min-h-40 cursor-pointer flex-col rounded-lg border border-border bg-card p-5 transition-colors hover:border-primary/45 hover:bg-card/90"
              role="button"
              tabindex="0"
              @click="openDeck(deck.id)"
              @keydown="openDeckFromKeyboard($event, deck.id)"
            >
              <div class="flex items-start justify-between gap-3">
                <h3 class="min-w-0 text-base font-bold group-hover:text-primary">{{ deck.name }}</h3>
                <button
                  class="rounded-md p-2 text-muted-foreground hover:bg-muted hover:text-foreground"
                  :aria-label="`Delete ${deck.name}`"
                  title="Delete deck"
                  @click.stop="deckPendingDelete = deck"
                >
                  <Trash2 class="h-4 w-4" />
                </button>
              </div>
              <div class="mt-4 grid grid-cols-3 gap-2 text-xs">
                <div>
                  <p class="text-muted-foreground">Cards</p>
                  <p class="mt-1 font-semibold">{{ deck.cardCount }}</p>
                </div>
                <div>
                  <p class="text-muted-foreground">Known</p>
                  <p class="mt-1 font-semibold">{{ deck.lastKnownCount }}</p>
                </div>
                <div>
                  <p class="text-muted-foreground">Unknown</p>
                  <p class="mt-1 font-semibold text-primary">{{ deck.lastUnknownCount }}</p>
                </div>
              </div>
              <div class="mt-auto flex items-end justify-between gap-3 pt-4">
                <p class="text-[11px] text-muted-foreground">
                  {{ new Date(deck.lastStudiedAt).toLocaleString() }}
                </p>
                <BaseButton
                  v-if="deck.lastUnknownCardIds.length > 0"
                  variant="secondary"
                  size="sm"
                  @click="reviewUnknown($event, deck)"
                >
                  <RotateCcw class="h-3.5 w-3.5" />
                  Review unknown
                </BaseButton>
              </div>
            </article>
          </div>
        </section>

        <section class="flex flex-col gap-3">
          <h2 class="text-sm font-bold uppercase tracking-wider text-muted-foreground">All decks</h2>
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
        <article
          v-for="deck in library.decks"
          :key="deck.id"
          class="group flex min-h-56 cursor-pointer flex-col rounded-lg border border-border bg-card p-5 transition-colors hover:border-primary/45 hover:bg-card/90"
          role="button"
          tabindex="0"
          @click="openDeck(deck.id)"
          @keydown="openDeckFromKeyboard($event, deck.id)"
        >
          <div class="flex items-start justify-between gap-3">
            <h3 class="min-w-0 flex-1 text-lg font-bold text-foreground group-hover:text-primary">
              {{ deck.name }}
            </h3>
            <button
              class="rounded-md p-2 text-muted-foreground hover:bg-muted hover:text-foreground"
              :aria-label="`Delete ${deck.name}`"
              title="Delete deck"
              @click.stop="deckPendingDelete = deck"
            >
              <Trash2 class="h-4 w-4" />
            </button>
          </div>

          <p v-if="deck.description" class="mt-3 line-clamp-3 text-sm leading-6 text-muted-foreground">
            {{ deck.description }}
          </p>
          <p v-else class="mt-3 text-sm leading-6 text-muted-foreground">No description.</p>

          <div class="mt-auto flex items-end justify-between gap-4 pt-6">
            <div class="min-w-0">
              <p class="text-xs uppercase tracking-wider text-muted-foreground">
                {{ deck.subject || 'General' }}
              </p>
              <p class="mt-1 text-sm font-semibold">{{ deck.cardCount }} cards</p>
            </div>
            <div class="flex max-w-36 flex-wrap justify-end gap-1">
              <span
                v-for="tag in deck.tags.slice(0, 3)"
                :key="tag"
                class="rounded-sm border border-border bg-muted px-2 py-1 text-[10px] text-muted-foreground"
              >
                {{ tag }}
              </span>
            </div>
          </div>
        </article>
      </div>
        </section>
      </template>

      <div v-if="library.error" class="rounded-lg border border-destructive/40 bg-destructive/10 p-4">
        <p class="text-sm font-semibold">{{ library.error.message }}</p>
        <ul class="mt-2 space-y-1 text-xs text-muted-foreground">
          <li v-for="detail in library.error.details" :key="detail">{{ detail }}</li>
        </ul>
      </div>
    </div>
    <ConfirmDialog
      :open="deckPendingDelete !== null"
      title="Delete deck?"
      :description="`This will permanently delete '${deckPendingDelete?.name ?? ''}' and all of its cards.`"
      confirm-label="Delete"
      @cancel="deckPendingDelete = null"
      @confirm="confirmDelete"
    />
  </section>
</template>
