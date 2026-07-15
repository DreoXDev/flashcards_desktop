<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minus, Square, X } from 'lucide-vue-next'

const appWindow = getCurrentWindow()

function minimizeWindow() {
  void appWindow.minimize()
}

function toggleMaximizeWindow() {
  void appWindow.toggleMaximize()
}

function closeWindow() {
  void appWindow.close()
}
</script>

<template>
  <header
    class="relative z-50 flex h-10 shrink-0 select-none items-center justify-between border-b border-border/70 bg-background/90 text-foreground backdrop-blur"
    data-tauri-drag-region
    @dblclick="toggleMaximizeWindow"
  >
    <div class="flex h-full items-center gap-2 pl-4" data-tauri-drag-region>
      <div class="h-2 w-2 rounded-full bg-primary"></div>
      <span
        class="text-xs font-bold uppercase tracking-wider text-muted-foreground"
        data-tauri-drag-region
      >
        Flashcards Desktop
      </span>
    </div>

    <div class="flex h-full items-center">
      <button
        class="flex h-full w-10 items-center justify-center text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
        aria-label="Minimize window"
        title="Minimize"
        @click.stop="minimizeWindow"
      >
        <Minus class="h-3.5 w-3.5" />
      </button>
      <button
        class="flex h-full w-10 items-center justify-center text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
        aria-label="Maximize or restore window"
        title="Maximize"
        @click.stop="toggleMaximizeWindow"
      >
        <Square class="h-3 w-3" />
      </button>
      <button
        class="flex h-full w-12 items-center justify-center text-muted-foreground transition-colors hover:bg-destructive hover:text-white"
        aria-label="Close window"
        title="Close"
        @click.stop="closeWindow"
      >
        <X class="h-4 w-4" />
      </button>
    </div>
  </header>
</template>
