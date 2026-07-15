<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import type { Flashcard } from '@/types/deck'

const props = defineProps<{
  card: Flashcard
  selectedOptionIds: string[]
}>()

const emit = defineEmits<{
  answer: [selectedOptionIds: string[]]
}>()

const selectedIds = ref<Set<string>>(new Set(props.selectedOptionIds))
const isMultiple = computed(() => props.card.correctOptionIds.length > 1)
const isAnswered = computed(() => props.selectedOptionIds.length > 0)

watch(
  () => [props.card.id, props.selectedOptionIds.join('|')],
  () => {
    selectedIds.value = new Set(props.selectedOptionIds)
  }
)

function toggleOption(optionId: string) {
  if (isAnswered.value) return
  const next = new Set(selectedIds.value)
  if (isMultiple.value) {
    if (next.has(optionId)) next.delete(optionId)
    else next.add(optionId)
  } else {
    next.clear()
    next.add(optionId)
  }
  selectedIds.value = next
  if (!isMultiple.value) {
    emit('answer', [...next])
  }
}

function optionClass(optionId: string) {
  const isSelected = selectedIds.value.has(optionId)
  const isCorrect = props.card.correctOptionIds.includes(optionId)
  const isWrongSelection = isSelected && !isCorrect
  if (!isAnswered.value && isSelected) return 'border-primary bg-primary/10 text-foreground'
  if (!isAnswered.value) return 'border-border bg-background text-foreground hover:bg-muted'
  if (isCorrect) return 'border-emerald-500 bg-emerald-500/15 text-foreground'
  if (isWrongSelection) return 'border-destructive bg-destructive/15 text-foreground'
  return 'border-border bg-background text-muted-foreground opacity-70'
}

function checkAnswer() {
  if (selectedIds.value.size === 0 || isAnswered.value) return
  emit('answer', [...selectedIds.value])
}
</script>

<template>
  <div class="flex h-full flex-col rounded-lg border border-border bg-card p-8 shadow-[0_0_40px_hsl(0_72%_51%/0.08)]">
    <p class="text-xs uppercase tracking-wider text-muted-foreground">Question</p>
    <h2 class="mt-4 text-2xl font-semibold leading-9">{{ card.question }}</h2>

    <div class="mt-6 grid gap-3 overflow-auto pr-1">
      <button
        v-for="option in card.options"
        :key="option.id"
        class="flex items-start gap-3 rounded-lg border p-4 text-left text-sm transition-colors"
        :class="optionClass(option.id)"
        @click="toggleOption(option.id)"
      >
        <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-sm border border-current text-xs font-bold">
          {{ option.id }}
        </span>
        <span class="leading-6">{{ option.text }}</span>
      </button>
    </div>

    <div v-if="isAnswered" class="mt-5 rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm leading-6">
      <p class="font-semibold text-emerald-300">Correct answer</p>
      <p class="mt-1">
        {{ card.correctOptionIds.join(', ') }}
      </p>
      <p v-if="card.explanation" class="mt-3 text-muted-foreground">{{ card.explanation }}</p>
    </div>

    <div class="mt-auto pt-6">
      <BaseButton v-if="isMultiple" variant="primary" :disabled="selectedIds.size === 0 || isAnswered" @click="checkAnswer">
        Check answer
      </BaseButton>
      <p v-else-if="!isAnswered" class="text-xs text-muted-foreground">Choose one answer.</p>
      <p v-else class="text-xs text-muted-foreground">Answer locked for this session.</p>
    </div>
  </div>
</template>
