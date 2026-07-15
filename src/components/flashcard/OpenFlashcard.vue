<script setup lang="ts">
import type { Flashcard } from '@/types/deck'

defineProps<{
  card: Flashcard
  revealed: boolean
}>()

defineEmits<{
  toggle: []
}>()
</script>

<template>
  <button
    class="flashcard-perspective h-full w-full text-left"
    :aria-pressed="revealed"
    @click="$emit('toggle')"
  >
    <div class="flashcard-flip relative h-full w-full" :class="{ 'is-revealed': revealed }">
      <div
        class="flashcard-face absolute inset-0 flex flex-col justify-center rounded-lg border border-border bg-card p-8 shadow-[0_0_40px_hsl(0_72%_51%/0.08)]"
      >
        <p class="text-xs uppercase tracking-wider text-muted-foreground">Question</p>
        <div class="mt-5 overflow-auto text-balance text-2xl font-semibold leading-10">
          {{ card.question }}
        </div>
        <p class="mt-8 text-xs text-muted-foreground">Click or press Space to flip.</p>
      </div>
      <div
        class="flashcard-face flashcard-back absolute inset-0 flex flex-col rounded-lg border border-primary/35 bg-card p-8 shadow-[0_0_48px_hsl(0_72%_51%/0.12)]"
      >
        <p class="text-xs uppercase tracking-wider text-primary">Answer</p>
        <div class="mt-5 min-h-0 overflow-auto text-lg leading-8 text-foreground">
          {{ card.answer }}
        </div>
      </div>
    </div>
  </button>
</template>

<style scoped>
.flashcard-perspective {
  perspective: 1600px;
}

.flashcard-flip {
  transform-style: preserve-3d;
  transition: transform 180ms ease;
}

.flashcard-flip.is-revealed {
  transform: rotateY(180deg);
}

.flashcard-face {
  backface-visibility: hidden;
}

.flashcard-back {
  transform: rotateY(180deg);
}
</style>
