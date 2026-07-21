<script setup lang="ts">
import { computed } from 'vue'
import { renderMarkdown } from '@/lib/markdown'
import type { Flashcard } from '@/types/deck'

const props = defineProps<{
  card: Flashcard
  revealed: boolean
}>()

defineEmits<{
  toggle: []
}>()

const answerHtml = computed(() => renderMarkdown(props.card.answer))
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
        <div class="rounded-md border border-border bg-background/60 p-4">
          <p class="text-xs uppercase tracking-wider text-muted-foreground">Question</p>
          <p class="mt-2 line-clamp-3 text-sm font-semibold leading-6 text-foreground">{{ card.question }}</p>
        </div>
        <p class="mt-5 text-xs uppercase tracking-wider text-primary">Answer</p>
        <div class="markdown-answer mt-5 min-h-0 overflow-auto text-lg leading-8 text-foreground" v-html="answerHtml"></div>
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

.markdown-answer :deep(p + p),
.markdown-answer :deep(ul + p),
.markdown-answer :deep(p + ul) {
  margin-top: 1rem;
}

.markdown-answer :deep(ul) {
  list-style: disc;
  padding-left: 1.5rem;
}

.markdown-answer :deep(li + li) {
  margin-top: 0.35rem;
}

.markdown-answer :deep(strong) {
  color: hsl(var(--foreground));
  font-weight: 700;
}

.markdown-answer :deep(em) {
  color: hsl(var(--foreground));
}

.markdown-answer :deep(code) {
  border: 1px solid hsl(var(--border));
  border-radius: 0.25rem;
  background: hsl(var(--muted));
  padding: 0.1rem 0.35rem;
  font-size: 0.9em;
}

.markdown-answer :deep(.markdown-link) {
  color: hsl(var(--primary));
  text-decoration: underline;
  text-underline-offset: 3px;
}

.markdown-answer :deep(.markdown-url) {
  color: hsl(var(--muted-foreground));
  font-size: 0.85em;
}
</style>
