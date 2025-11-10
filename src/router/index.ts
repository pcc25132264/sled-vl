import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/connections',
      name: 'connections',
      component: () => import('../views/Connections.vue')
    },
    {
      path: '/data',
      name: 'data',
      component: () => import('../views/Data.vue')
    },
    {
      path: '/query',
      name: 'query',
      component: () => import('../views/Query.vue')
    },
    {
      path: '/monitor',
      name: 'monitor',
      component: () => import('../views/Monitor.vue')
    },
    {
      path: '/metadata',
      name: 'metadata',
      component: () => import('../views/Metadata.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/Settings.vue')
    },
    {
      path: '/import-export',
      name: 'import-export',
      component: () => import('../views/ImportExport.vue')
    }
  ]
})

export default router