<template>
  <div class="home-container">
    <el-row :gutter="10">
      <el-col :span="24">
        <el-card class="welcome-card">
          <h1>欢迎使用 Sled Visualizer</h1>
          <p>一个功能强大的 Sled 数据库可视化管理工具</p>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="10" style="margin-top: 10px;">
      <el-col :span="8">
        <el-card class="feature-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon size="24"><Connection /></el-icon>
              <span>连接管理</span>
            </div>
          </template>
          <p>管理多个 Sled 数据库连接，支持快速切换和配置保存</p>
          <el-button type="primary" @click="$router.push('/connections')">管理连接</el-button>
        </el-card>
      </el-col>
      
      <el-col :span="8">
        <el-card class="feature-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon size="24"><DataLine /></el-icon>
              <span>数据浏览</span>
            </div>
          </template>
          <p>以多种视图浏览数据库内容，支持键值对的高效展示和操作</p>
          <el-button type="primary" @click="$router.push('/data')">浏览数据</el-button>
        </el-card>
      </el-col>
      
      <el-col :span="8">
        <el-card class="feature-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon size="24"><Search /></el-icon>
              <span>查询工具</span>
            </div>
          </template>
          <p>强大的查询构建器，支持范围查询、前缀匹配等多种查询方式</p>
          <el-button type="primary" @click="$router.push('/query')">查询数据</el-button>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="10" style="margin-top: 10px;">
      <el-col :span="8">
        <el-card class="feature-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon size="24"><Monitor /></el-icon>
              <span>监控诊断</span>
            </div>
          </template>
          <p>实时监控数据库性能指标，查看运行日志和诊断信息</p>
          <el-button type="primary" @click="$router.push('/monitor')">监控诊断</el-button>
        </el-card>
      </el-col>
      
      <el-col :span="8">
        <el-card class="feature-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon size="24"><Download /></el-icon>
              <span>导入导出</span>
            </div>
          </template>
          <p>支持数据的批量导入导出，兼容 JSON、CSV 等多种格式</p>
          <el-button type="primary" @click="$router.push('/data')">导入导出</el-button>
        </el-card>
      </el-col>
      
      <el-col :span="8">
        <el-card class="feature-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <el-icon size="24"><Setting /></el-icon>
              <span>应用设置</span>
            </div>
          </template>
          <p>自定义应用行为和界面外观，管理权限和安全设置</p>
          <el-button type="primary" @click="$router.push('/settings')">应用设置</el-button>
        </el-card>
      </el-col>
    </el-row>
    
    <el-row v-if="currentConnection" :gutter="10" style="margin-top: 10px;">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <el-icon size="24"><InfoFilled /></el-icon>
              <span>当前连接状态</span>
            </div>
          </template>
          <el-descriptions :column="2" border>
            <el-descriptions-item label="连接名称">{{ currentConnection.name }}</el-descriptions-item>
            <el-descriptions-item label="数据库路径">{{ currentConnection.path }}</el-descriptions-item>
            <el-descriptions-item label="创建时间">{{ formatDate(currentConnection.created_at) }}</el-descriptions-item>
            <el-descriptions-item label="最后访问">{{ formatDate(currentConnection.last_accessed) }}</el-descriptions-item>
          </el-descriptions>
          
          <div style="margin-top: 20px;">
            <el-button type="primary" @click="$router.push('/data')">查看数据</el-button>
            <el-button @click="$router.push('/query')">查询数据</el-button>
            <el-button @click="$router.push('/monitor')">查看监控</el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSledStore } from '../stores/sled'
import { Connection, DataLine, Search, Monitor, Download, Setting, InfoFilled } from '@element-plus/icons-vue'

const sledStore = useSledStore()

const currentConnection = computed(() => {
  if (!sledStore.currentConnectionId) return null
  return sledStore.connections.find(c => c.id === sledStore.currentConnectionId)
})

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString()
}
</script>

<style scoped>
.home-container {
  max-width: 1200px;
  margin: 0 auto;
}

.welcome-card {
  text-align: center;
  background: linear-gradient(135deg, #6a11cb 0%, #2575fc 100%);
  color: white;
  border: none;
}

.welcome-card h1 {
  margin: 0 0 10px 0;
  font-size: 28px;
}

.welcome-card p {
  margin: 0;
  font-size: 16px;
  opacity: 0.9;
}

.feature-card {
  height: 100%;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: bold;
}
</style>