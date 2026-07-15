import { createRouter, createWebHashHistory } from 'vue-router'
import LibraryView from '@/views/LibraryView.vue'
import DeckDetailView from '@/views/DeckDetailView.vue'
import StudyView from '@/views/StudyView.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'library', component: LibraryView },
    { path: '/decks/:deckId', name: 'deck-detail', component: DeckDetailView, props: true },
    {
      path: '/study/:deckId',
      name: 'study',
      component: StudyView,
      props: route => ({
        deckId: route.params.deckId,
        mode: route.query.mode === 'random' ? 'random' : 'original',
        sessionMode: route.query.session === 'review-unknown' ? 'review-unknown' : 'full-deck',
        cardIds:
          typeof route.query.cards === 'string' && route.query.cards.length > 0
            ? route.query.cards.split(',').filter(Boolean)
            : [],
      }),
    },
  ],
})
