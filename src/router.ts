import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'

const routes = [
  {
    path: '/',
    component: App,
  }
]

export const router = createRouter({
  history: createWebHistory(),
  routes
}) 