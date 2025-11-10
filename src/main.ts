import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createPersistedState } from 'pinia-plugin-persistedstate'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import App from './App.vue'
import router from './router'

// 导入Tauri API
import '@tauri-apps/api/core'

// 添加调试日志
console.log('Main.ts: Starting application initialization...')

// 检查Tauri环境
if (typeof window !== 'undefined' && (window as any).__TAURI__) {
  console.log('Main.ts: Tauri environment detected')
} else {
  console.log('Main.ts: Running in browser environment')
}

const app = createApp(App)

// 注册所有图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

const pinia = createPinia()
pinia.use(createPersistedState({
  storage: localStorage,
  beforeRestore: (context) => {
    console.log('Before restore:', context)
  },
  afterRestore: (context) => {
    console.log('After restore:', context)
  }
}))
app.use(pinia)
app.use(router)
app.use(ElementPlus)

console.log('Main.ts: Mounting app...')
app.mount('#app')
console.log('Main.ts: App mounted successfully!')
