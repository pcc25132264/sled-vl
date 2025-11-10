<template>
  <el-container class="app-container">
    <el-header class="app-header">
      <div class="header-left">
        <h1>Sled Visualizer</h1>
      </div>
      <div class="header-right">
        <el-switch
          v-model="isDarkMode"
          inline-prompt
          :active-icon="Moon"
          :inactive-icon="Sunny"
          @change="toggleDarkMode"
        />
      </div>
    </el-header>
    
    <el-container class="main-container">
      <el-aside width="180px" class="sidebar">
        <el-menu
          :default-active="$route.path"
          router
          class="sidebar-menu"
        >
          <el-menu-item index="/">
            <el-icon><HomeFilled /></el-icon>
            <span>首页</span>
          </el-menu-item>
          <el-menu-item index="/connections">
            <el-icon><Connection /></el-icon>
            <span>连接管理</span>
          </el-menu-item>
          <el-menu-item index="/data">
          <el-icon><DataBoard /></el-icon>
          <span>数据浏览</span>
        </el-menu-item>
        <el-menu-item index="/query">
            <el-icon><Search /></el-icon>
            <span>查询工具</span>
          </el-menu-item>
          <el-menu-item index="/monitor">
          <el-icon><TrendCharts /></el-icon>
          <span>监控与诊断</span>
        </el-menu-item>
        <el-menu-item index="/metadata">
          <el-icon><FolderOpened /></el-icon>
          <span>索引与元数据</span>
        </el-menu-item>
        <el-menu-item index="/import-export">
          <el-icon><Upload /></el-icon>
          <span>数据导入导出</span>
        </el-menu-item>
          <el-menu-item index="/settings">
            <el-icon><Setting /></el-icon>
            <span>设置</span>
          </el-menu-item>
        </el-menu>
        
        <div class="connection-status" v-if="currentConnection">
          <el-divider>当前连接</el-divider>
          <div class="connection-info">
            <el-text size="small" type="primary">{{ currentConnection.name }}</el-text>
            <el-text size="small" type="info">{{ currentConnection.path }}</el-text>
          </div>
        </div>
      </el-aside>
      
      <el-main class="main-content">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useSledStore } from './stores/sled'
import { HomeFilled, Connection, DataBoard, Search, TrendCharts, FolderOpened, Upload, Setting, Moon, Sunny } from '@element-plus/icons-vue'

const sledStore = useSledStore()
const isDarkMode = ref(false)

const currentConnection = computed(() => {
  if (!sledStore.currentConnectionId) return null
  return sledStore.connections.find(c => c.id === sledStore.currentConnectionId)
})

const toggleDarkMode = () => {
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}

onMounted(() => {
  sledStore.loadConnections()
  
  // Check system preference for dark mode
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    isDarkMode.value = true
    toggleDarkMode()
  }
})
</script>

<style scoped>
.app-container {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
  background-color: var(--el-bg-color);
  border-bottom: 1px solid var(--el-border-color);
}

.header-left h1 {
  margin: 0;
  font-size: 20px;
  color: var(--el-text-color-primary);
}

.main-container {
  height: calc(100vh - 60px);
}

.sidebar {
  background-color: var(--el-bg-color-page);
  border-right: 1px solid var(--el-border-color);
  display: flex;
  flex-direction: column;
}

.sidebar-menu {
  flex: 1;
  border-right: none;
}

.connection-status {
  padding: 10px;
  border-top: 1px solid var(--el-border-color);
}

.connection-info {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.main-content {
  padding: 20px;
  background-color: var(--el-bg-color-page);
  overflow-y: auto;
}
</style>