<script setup lang="ts">
import { onUnmounted, watch } from 'vue'
import BaseButton from '@/components/ui/BaseButton.vue'

const emit = defineEmits<{
  cancel: []
  confirm: []
}>()

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape' && props.open) {
    emit('cancel')
  }
}

const props = withDefaults(
  defineProps<{
    open: boolean
    title: string
    description: string
    confirmLabel?: string
  }>(),
  {
    confirmLabel: 'Delete',
  }
)

watch(
  () => props.open,
  open => {
    if (open) window.addEventListener('keydown', handleKeydown)
    else window.removeEventListener('keydown', handleKeydown)
  },
  { immediate: true }
)

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <Teleport to="body">
    <div
      v-if="open"
      class="dark fixed inset-0 z-[100] flex items-center justify-center bg-black/75 p-6 backdrop-blur-sm"
      role="dialog"
      aria-modal="true"
      :aria-label="title"
      @keydown.esc="$emit('cancel')"
    >
      <div
        class="w-full max-w-md rounded-lg border border-border bg-card p-6 text-card-foreground shadow-[0_24px_80px_rgba(0,0,0,0.65),0_0_0_1px_rgba(255,255,255,0.04)]"
      >
        <div class="mb-5 border-b border-border pb-4">
          <h2 class="text-lg font-bold text-foreground">{{ title }}</h2>
          <p class="mt-3 text-sm leading-6 text-muted-foreground">{{ description }}</p>
        </div>
        <div class="flex justify-end gap-3">
          <BaseButton variant="secondary" @click="$emit('cancel')">Cancel</BaseButton>
          <BaseButton variant="danger" @click="$emit('confirm')">{{ confirmLabel }}</BaseButton>
        </div>
      </div>
    </div>
  </Teleport>
</template>
