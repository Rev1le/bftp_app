import breakFile from '../pages/breakFile.vue'
import joinFile from '../pages/joinFile.vue'

import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: breakFile,
   
  },
  {
    path: '/join',
    component: joinFile,   
  },
 
]

const router = createRouter({
    history: createWebHistory(import.meta.env),
    routes
  })



export default router