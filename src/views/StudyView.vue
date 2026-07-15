<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowLeft, Check, ChevronLeft, ChevronRight, Circle, List, RotateCcw, X } from 'lucide-vue-next'
import BaseButton from '@/components/ui/BaseButton.vue'
import OpenFlashcard from '@/components/flashcard/OpenFlashcard.vue'
import ClosedFlashcard from '@/components/flashcard/ClosedFlashcard.vue'
import { deckApi, normalizeError } from '@/lib/api'
import { shuffled } from '@/lib/shuffle'
import type {
  AppErrorPayload,
  CardStudyState,
  DeckDetail,
  Flashcard,
  StudyMode,
  StudyResult,
  StudySessionMode,
} from '@/types/deck'

const props = defineProps<{
  deckId: string
  mode: StudyMode
  sessionMode: StudySessionMode
  cardIds: string[]
  resumeSessionId: string | null
}>()

const router = useRouter()
const deck = ref<DeckDetail | null>(null)
const cards = ref<Flashcard[]>([])
const currentIndex = ref(0)
const states = ref<Record<string, CardStudyState>>({})
const loading = ref(true)
const error = ref<AppErrorPayload | null>(null)
const sidebarOpen = ref(true)
const summaryOpen = ref(false)
const savingSummary = ref(false)
const activeSessionId = ref('')
const saveTimer = ref<number | null>(null)
const saveReady = ref(false)

const currentCard = computed(() => cards.value[currentIndex.value] ?? null)
const currentState = computed(() => (currentCard.value ? states.value[currentCard.value.id] : null))
const answeredCount = computed(() => Object.values(states.value).filter(state => state.result !== null).length)
const knownCount = computed(() => Object.values(states.value).filter(state => state.result === 'known').length)
const unknownCards = computed(() => cards.value.filter(card => states.value[card.id]?.result === 'unknown'))
const unknownCount = computed(() => unknownCards.value.length)
const isSessionComplete = computed(() => cards.value.length > 0 && answeredCount.value === cards.value.length)
const progressLabel = computed(() =>
  cards.value.length === 0 ? '0 / 0' : `${currentIndex.value + 1} / ${cards.value.length}`
)
const answeredLabel = computed(() => `${answeredCount.value} / ${cards.value.length} answered`)
const progressPercent = computed(() =>
  cards.value.length === 0 ? 0 : (answeredCount.value / cards.value.length) * 100
)

async function load() {
  loading.value = true
  error.value = null
  summaryOpen.value = false
  try {
    deck.value = await deckApi.getDeck(props.deckId)

    if (props.resumeSessionId) {
      const saved = await deckApi.getActiveStudySession(props.resumeSessionId)
      activeSessionId.value = saved.id
      const restoredCards = saved.cardIds
            .map(cardId => deck.value?.cards.find(card => card.id === cardId))
            .filter((card): card is Flashcard => Boolean(card))
      cards.value = restoredCards
      currentIndex.value = Math.min(saved.currentIndex, Math.max(restoredCards.length - 1, 0))
      states.value = restoreStates(saved.statesJson, restoredCards)
      if (cards.value[currentIndex.value]) states.value[cards.value[currentIndex.value].id].visited = true
      saveReady.value = true
      return
    }

    const sourceCards =
      props.sessionMode === 'review-unknown'
        ? props.cardIds
            .map(cardId => deck.value?.cards.find(card => card.id === cardId))
            .filter((card): card is Flashcard => Boolean(card))
        : [...deck.value.cards]

    cards.value = props.mode === 'random' && props.sessionMode === 'full-deck' ? shuffled(sourceCards) : sourceCards
    activeSessionId.value = createSessionId(deck.value.id, props.sessionMode, cards.value)
    currentIndex.value = 0
    states.value = Object.fromEntries(
      cards.value.map(card => [
        card.id,
        {
          cardId: card.id,
          result: null,
          visited: false,
          answerVisible: false,
          selectedOptionIds: [],
        },
      ])
    )
    if (cards.value[0]) states.value[cards.value[0].id].visited = true
    saveReady.value = true
  } catch (err) {
    error.value = normalizeError(err)
  } finally {
    loading.value = false
  }
}

function createSessionId(deckId: string, sessionMode: StudySessionMode, sessionCards: Flashcard[]) {
  const suffix = sessionMode === 'review-unknown' ? sessionCards.map(card => card.id).join('-') : 'full'
  return `${deckId}__${sessionMode}__${suffix}`
}

function restoreStates(statesJson: string, restoredCards: Flashcard[]) {
  let parsed: Record<string, CardStudyState> = {}
  try {
    parsed = JSON.parse(statesJson) as Record<string, CardStudyState>
  } catch {
    parsed = {}
  }

  return Object.fromEntries(
    restoredCards.map(card => [
      card.id,
      {
        cardId: card.id,
        result: parsed[card.id]?.result ?? null,
        visited: parsed[card.id]?.visited ?? false,
        answerVisible: parsed[card.id]?.answerVisible ?? false,
        selectedOptionIds: parsed[card.id]?.selectedOptionIds ?? [],
      },
    ])
  )
}

function goToCard(index: number) {
  if (index < 0 || index >= cards.value.length) return
  currentIndex.value = index
  const card = cards.value[index]
  states.value[card.id].visited = true
}

function nextCard() {
  goToCard(Math.min(currentIndex.value + 1, cards.value.length - 1))
}

function previousCard() {
  goToCard(Math.max(currentIndex.value - 1, 0))
}

function toggleOpenCard() {
  if (!currentState.value) return
  currentState.value.answerVisible = !currentState.value.answerVisible
}

function markCurrent(result: Exclude<StudyResult, null>) {
  if (!currentState.value) return
  currentState.value.result = result
  goToNextUnansweredOrNext()
}

function submitClosedAnswer(selectedOptionIds: string[]) {
  if (!currentCard.value || !currentState.value || currentState.value.result !== null) return
  currentState.value.selectedOptionIds = selectedOptionIds
  const correct = currentCard.value.correctOptionIds
  const selected = [...selectedOptionIds].sort()
  const expected = [...correct].sort()
  currentState.value.result =
    selected.length === expected.length && selected.every((id, index) => id === expected[index])
      ? 'known'
      : 'unknown'
}

function goToNextUnansweredOrNext() {
  const nextUnansweredIndex = cards.value.findIndex(
    (card, index) => index > currentIndex.value && states.value[card.id]?.result === null
  )
  if (nextUnansweredIndex >= 0) {
    goToCard(nextUnansweredIndex)
    return
  }
  const anyUnansweredIndex = cards.value.findIndex(card => states.value[card.id]?.result === null)
  if (anyUnansweredIndex >= 0) {
    goToCard(anyUnansweredIndex)
    return
  }
  nextCard()
}

async function completeSession() {
  if (!deck.value || !isSessionComplete.value) return
  savingSummary.value = true
  try {
    await deckApi.saveStudyHistory({
      deckId: deck.value.id,
      lastKnownCount: knownCount.value,
      lastUnknownCount: unknownCount.value,
      lastUnknownCardIds: unknownCards.value.map(card => card.id),
    })
    if (activeSessionId.value) {
      await deckApi.deleteActiveStudySession(activeSessionId.value)
    }
    summaryOpen.value = true
  } catch (err) {
    error.value = normalizeError(err)
  } finally {
    savingSummary.value = false
  }
}

function resetSession(newCards: Flashcard[]) {
  cards.value = newCards
  currentIndex.value = 0
  summaryOpen.value = false
  activeSessionId.value = deck.value ? createSessionId(deck.value.id, props.sessionMode, newCards) : ''
  states.value = Object.fromEntries(
    newCards.map(card => [
      card.id,
      { cardId: card.id, result: null, visited: false, answerVisible: false, selectedOptionIds: [] },
    ])
  )
  if (newCards[0]) states.value[newCards[0].id].visited = true
  void saveActiveSessionNow()
}

function studyAgain() {
  const baseCards = deck.value?.cards ?? []
  resetSession(props.mode === 'random' && props.sessionMode === 'full-deck' ? shuffled(baseCards) : [...baseCards])
}

function startUnknownReview() {
  resetSession(unknownCards.value)
}

function scheduleSaveActiveSession() {
  if (!saveReady.value || summaryOpen.value || cards.value.length === 0 || !deck.value || !activeSessionId.value) return
  if (saveTimer.value !== null) window.clearTimeout(saveTimer.value)
  saveTimer.value = window.setTimeout(() => {
    void saveActiveSessionNow()
  }, 250)
}

async function saveActiveSessionNow() {
  if (!saveReady.value || summaryOpen.value || cards.value.length === 0 || !deck.value || !activeSessionId.value) return
  await deckApi.saveActiveStudySession({
    id: activeSessionId.value,
    deckId: deck.value.id,
    sessionMode: props.sessionMode,
    studyMode: props.mode,
    cardIds: cards.value.map(card => card.id),
    currentIndex: currentIndex.value,
    statesJson: JSON.stringify(states.value),
  })
}

function resultIcon(result: StudyResult) {
  if (result === 'known') return Check
  if (result === 'unknown') return X
  return Circle
}

function isTyping(event: KeyboardEvent) {
  const target = event.target as HTMLElement | null
  const tag = target?.tagName.toLowerCase()
  return tag === 'input' || tag === 'textarea' || Boolean(target?.isContentEditable)
}

function handleKeydown(event: KeyboardEvent) {
  if (isTyping(event) || loading.value || error.value || summaryOpen.value) return
  if (event.key === 'Escape') {
    event.preventDefault()
    void router.push({ name: 'deck-detail', params: { deckId: props.deckId } })
  }
  if (event.code === 'Space' && currentCard.value?.cardType === 'open') {
    event.preventDefault()
    toggleOpenCard()
  }
  if (event.key === 'ArrowRight') nextCard()
  if (event.key === 'ArrowLeft') previousCard()
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
  void load()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  if (saveTimer.value !== null) window.clearTimeout(saveTimer.value)
  void saveActiveSessionNow()
})

watch([currentIndex, states], scheduleSaveActiveSession, { deep: true })
</script>

<template>
  <section class="flex h-full overflow-hidden px-7 py-6">
    <div class="flex min-w-0 flex-1 flex-col">
      <div class="mb-5 flex items-center justify-between gap-4">
        <BaseButton variant="ghost" @click="router.push({ name: 'deck-detail', params: { deckId } })">
          <ArrowLeft class="h-4 w-4" />
          {{ deck?.name || 'Deck' }}
        </BaseButton>
        <div class="flex items-center gap-3">
          <div class="text-sm font-semibold text-muted-foreground">{{ progressLabel }}</div>
          <BaseButton variant="secondary" size="sm" title="Toggle questions sidebar" @click="sidebarOpen = !sidebarOpen">
            <List class="h-4 w-4" />
          </BaseButton>
        </div>
      </div>

      <div class="mb-6 h-1 overflow-hidden rounded-full bg-muted">
        <div class="h-full bg-primary transition-all" :style="{ width: `${progressPercent}%` }"></div>
      </div>

      <div v-if="loading" class="rounded-lg border border-border bg-card p-8 text-sm text-muted-foreground">
        Loading study session...
      </div>

      <div v-else-if="error" class="rounded-lg border border-destructive/40 bg-destructive/10 p-6">
        <h1 class="text-xl font-bold">{{ error.message }}</h1>
        <p v-for="detail in error.details" :key="detail" class="mt-2 text-sm text-muted-foreground">
          {{ detail }}
        </p>
      </div>

      <div
        v-else-if="summaryOpen"
        class="flex flex-1 flex-col items-center justify-center rounded-lg border border-border bg-card p-8 text-center"
      >
        <h1 class="text-3xl font-bold">Session completed</h1>
        <div class="mt-6 grid grid-cols-2 gap-4 text-sm">
          <div class="rounded-lg border border-border bg-background p-5">
            <p class="text-muted-foreground">Known</p>
            <p class="mt-2 text-3xl font-bold text-emerald-300">{{ knownCount }}</p>
          </div>
          <div class="rounded-lg border border-border bg-background p-5">
            <p class="text-muted-foreground">Unknown</p>
            <p class="mt-2 text-3xl font-bold text-primary">{{ unknownCount }}</p>
          </div>
        </div>
        <div class="mt-8 flex flex-wrap justify-center gap-3">
          <BaseButton v-if="unknownCount > 0" variant="primary" @click="startUnknownReview">
            <RotateCcw class="h-4 w-4" />
            Review unknown cards
          </BaseButton>
          <BaseButton variant="secondary" @click="studyAgain">
            <RotateCcw class="h-4 w-4" />
            Study deck again
          </BaseButton>
          <BaseButton variant="secondary" @click="router.push('/')">Back to decks</BaseButton>
        </div>
      </div>

      <div
        v-else-if="cards.length === 0"
        class="flex flex-1 items-center justify-center rounded-lg border border-border bg-card p-8 text-center text-sm text-muted-foreground"
      >
        No cards are available for this session.
      </div>

      <template v-else-if="currentCard && currentState">
        <div class="min-h-0 flex-1">
          <OpenFlashcard
            v-if="currentCard.cardType === 'open'"
            :card="currentCard"
            :revealed="currentState.answerVisible"
            @toggle="toggleOpenCard"
          />
          <ClosedFlashcard
            v-else
            :card="currentCard"
            :selected-option-ids="currentState.selectedOptionIds"
            @answer="submitClosedAnswer"
          />
        </div>

        <div class="mt-6 grid min-h-11 grid-cols-3 items-center gap-3">
          <BaseButton variant="secondary" :disabled="currentIndex === 0" @click="previousCard">
            <ChevronLeft class="h-4 w-4" />
            Previous
          </BaseButton>
          <div class="flex justify-center gap-2">
            <template v-if="currentCard.cardType === 'open' && currentState.answerVisible">
              <button
                class="flex h-11 w-11 items-center justify-center rounded-md border border-destructive bg-destructive/15 text-destructive transition-colors hover:bg-destructive hover:text-white"
                title="Mark as unknown"
                aria-label="Mark as unknown"
                @click="markCurrent('unknown')"
              >
                <X class="h-5 w-5" />
              </button>
              <button
                class="flex h-11 w-11 items-center justify-center rounded-md border border-emerald-500 bg-emerald-500/15 text-emerald-300 transition-colors hover:bg-emerald-500 hover:text-white"
                title="Mark as known"
                aria-label="Mark as known"
                @click="markCurrent('known')"
              >
                <Check class="h-5 w-5" />
              </button>
              <BaseButton
                v-if="isSessionComplete"
                variant="primary"
                :disabled="savingSummary"
                @click="completeSession"
              >
                {{ savingSummary ? 'Saving...' : 'Complete session' }}
              </BaseButton>
            </template>
            <BaseButton
              v-else-if="isSessionComplete"
              variant="primary"
              :disabled="savingSummary"
              @click="completeSession"
            >
              {{ savingSummary ? 'Saving...' : 'Complete session' }}
            </BaseButton>
            <p v-else class="self-center text-xs text-muted-foreground">
              {{ answeredLabel }}
            </p>
          </div>
          <BaseButton variant="secondary" :disabled="currentIndex === cards.length - 1" @click="nextCard">
            Next
            <ChevronRight class="h-4 w-4" />
          </BaseButton>
        </div>
      </template>
    </div>

    <aside
      class="ml-5 flex min-h-0 shrink-0 flex-col overflow-hidden rounded-lg border border-border bg-card transition-all duration-200"
      :class="sidebarOpen ? 'w-80 opacity-100' : 'w-0 border-transparent opacity-0'"
    >
      <div class="border-b border-border p-4">
        <h2 class="text-sm font-bold">Questions</h2>
        <p class="mt-1 text-xs text-muted-foreground">{{ answeredLabel }}</p>
      </div>
      <div class="min-h-0 flex-1 overflow-auto p-2">
        <button
          v-for="(card, index) in cards"
          :key="card.id"
          class="flex w-full items-start gap-3 rounded-md p-3 text-left text-xs transition-colors hover:bg-muted"
          :class="index === currentIndex ? 'bg-primary/10 text-foreground' : 'text-muted-foreground'"
          @click="goToCard(index)"
        >
          <component
            :is="resultIcon(states[card.id]?.result ?? null)"
            class="mt-0.5 h-4 w-4 shrink-0"
            :class="{
              'text-emerald-300': states[card.id]?.result === 'known',
              'text-primary': states[card.id]?.result === 'unknown',
              'text-muted-foreground': states[card.id]?.result === null,
            }"
          />
          <span class="min-w-0">
            <span class="block font-semibold">{{ index + 1 }}.</span>
            <span class="mt-1 line-clamp-2 block leading-5">{{ card.question }}</span>
          </span>
        </button>
      </div>
    </aside>
  </section>
</template>
