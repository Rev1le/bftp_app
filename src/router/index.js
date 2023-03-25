import breakFile from '../pages/breakFile.vue'
import joinFile from '../pages/joinFile.vue'

import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: breakFile,
    meta: { transition: 'slide-left' },
   
  },
  {
    path: '/join',
    component: joinFile,
    meta: { transition: 'slide-right' },   
  },
 
]

const router = createRouter({
    history: createWebHistory(import.meta.env),
    routes
  })



export default router