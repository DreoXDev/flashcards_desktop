<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
    size?: 'sm' | 'md' | 'lg'
    disabled?: boolean
    type?: 'button' | 'submit'
  }>(),
  {
    variant: 'secondary',
    size: 'md',
    type: 'button',
  }
)

const classes = computed(() => [
  'inline-flex items-center justify-center gap-2 rounded-md border font-semibold transition-colors disabled:cursor-not-allowed disabled:opacity-50',
  props.size === 'sm' && 'h-8 px-3 text-xs',
  props.size === 'md' && 'h-10 px-4 text-sm',
  props.size === 'lg' && 'h-11 px-5 text-sm',
  props.variant === 'primary' &&
    'border-primary bg-primary text-primary-foreground hover:bg-primary/90',
  props.variant === 'secondary' &&
    'border-border bg-secondary text-secondary-foreground hover:bg-muted',
  props.variant === 'ghost' &&
    'border-transparent bg-transparent text-muted-foreground hover:bg-muted hover:text-foreground',
  props.variant === 'danger' &&
    'border-destructive bg-destructive text-white hover:bg-destructive/90',
])
</script>

<template>
  <button :type="type" :disabled="disabled" :class="classes">
    <slot />
  </button>
</template>
