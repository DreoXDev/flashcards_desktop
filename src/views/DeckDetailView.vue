<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowLeft, Shuffle, Rows3 } from 'lucide-vue-next'
import BaseButton from '@/components/ui/BaseButton.vue'
import { deckApi, normalizeError } from '@/lib/api'
import type { AppErrorPayload, DeckDetail, StudyMode } from '@/types/deck'

const props = defineProps<{ deckId: string }>()
const router = useRouter()
const deck = ref<DeckDetail | null>(null)
const loading = ref(true)
const error = ref<AppErrorPayload | null>(null)
const mode = ref<StudyMode>('original')

const cardCountLabel = computed(() => {
  const count = deck.value?.cards.length ?? 0
  return `${count} ${count === 1 ? 'card' : 'cards'}`
})

async function loadDeck() {
  loading.value = true
  error.value = null
  try {
    deck.value = await deckApi.getDeck(props.deckId)
  } catch (err) {
    error.value = normalizeError(err)
  } finally {
    loading.value = false
  }
}

function startStudy() {
  void router.push({ name: 'study', params: { deckId: props.deckId }, query: { mode: mode.value } })
}

onMounted(loadDeck)
</script>

<template>
  <section class="h-full overflow-auto px-8 py-8">
    <div class="mx-auto flex w-full max-w-4xl flex-col gap-8">
      <BaseButton variant="ghost" class="w-fit" @click="router.push('/')">
        <ArrowLeft class="h-4 w-4" />
        Back
      </BaseButton>

      <div v-if="loading" class="rounded-lg border border-border bg-card p-8 text-sm text-muted-foreground">
        Loading deck...
      </div>

      <div v-else-if="error" class="rounded-lg border border-destructive/40 bg-destructive/10 p-6">
        <h1 class="text-xl font-bold">{{ error.message }}</h1>
        <p v-for="detail in error.details" :key="detail" class="mt-2 text-sm text-muted-foreground">
          {{ detail }}
        </p>
      </div>

      <template v-else-if="deck">
        <header>
          <p class="text-xs uppercase tracking-wider text-muted-foreground">{{ deck.subject || 'General' }}</p>
          <h1 class="mt-3 text-3xl font-bold">{{ deck.name }}</h1>
          <p v-if="deck.description" class="mt-4 max-w-3xl text-sm leading-6 text-muted-foreground">
            {{ deck.description }}
          </p>
          <p class="mt-5 text-sm font-semibold text-foreground">{{ cardCountLabel }}</p>
        </header>

        <div class="rounded-lg border border-border bg-card p-5">
          <p class="mb-4 text-sm font-semibold">Study order</p>
          <div class="grid gap-3 sm:grid-cols-2">
            <button
              class="rounded-lg border p-4 text-left transition-colors"
              :class="mode === 'original' ? 'border-primary bg-primary/10' : 'border-border bg-background hover:bg-muted'"
              @click="mode = 'original'"
            >
              <Rows3 class="h-5 w-5 text-primary" />
              <span class="mt-3 block text-sm font-bold">Original order</span>
              <span class="mt-1 block text-xs leading-5 text-muted-foreground">Follow the JSON deck sequence.</span>
            </button>
            <button
              class="rounded-lg border p-4 text-left transition-colors"
              :class="mode === 'random' ? 'border-primary bg-primary/10' : 'border-border bg-background hover:bg-muted'"
              @click="mode = 'random'"
            >
              <Shuffle class="h-5 w-5 text-primary" />
              <span class="mt-3 block text-sm font-bold">Random order</span>
              <span class="mt-1 block text-xs leading-5 text-muted-foreground">Shuffle once for this session.</span>
            </button>
          </div>
        </div>

        <BaseButton variant="primary" size="lg" class="w-fit" :disabled="deck.cards.length === 0" @click="startStudy">
          Start studying
        </BaseButton>
      </template>
    </div>
  </section>
</template>
