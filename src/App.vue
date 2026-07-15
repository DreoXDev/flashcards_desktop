<script setup lang="ts">
import { RouterView } from 'vue-router'
import { Toaster } from 'vue-sonner'
import AppTitleBar from '@/components/app/AppTitleBar.vue'
</script>

<template>
  <div class="dark h-screen w-screen overflow-hidden bg-background text-foreground">
    <div class="flex h-full flex-col">
      <AppTitleBar />
      <main class="min-h-0 flex-1 overflow-hidden">
        <RouterView v-slot="{ Component }">
          <Transition name="page" mode="out-in">
            <component :is="Component" />
          </Transition>
        </RouterView>
      </main>
    </div>
    <Toaster rich-colors position="bottom-right" theme="dark" />
  </div>
</template>
