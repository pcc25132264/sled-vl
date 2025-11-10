<template>
  <div class="monitor-container">
    <el-row :gutter="10" v-if="currentConnection">
      <!-- 性能指标 -->
      <el-col :span="24">
        <el-card class="metrics-card">
          <template #header>
            <div class="card-header">
              <span>性能指标</span>
              <div class="header-actions">
                <el-select v-model="timeRange" size="small" style="width: 120px; margin-right: 10px;" @change="handleTimeRangeChange">
                  <el-option label="最近1分钟" value="1m" />
                  <el-option label="最近5分钟" value="5m" />
                  <el-option label="最近15分钟" value="15m" />
                  <el-option label="最近1小时" value="1h" />
                  <el-option label="最近6小时" value="6h" />
                </el-select>
                <el-button size="small" @click="refreshMetrics">
                  <el-icon><Refresh /></el-icon>
                  刷新
                </el-button>
              </div>
            </div>
          </template>
          
          <el-row :gutter="10">
            <el-col :span="6">
              <div class="metric-item">
                <div class="metric-title">读取 QPS</div>
                <div class="metric-value">{{ metrics.readQps }}</div>
                <div class="metric-trend" :class="metrics.readQpsTrend > 0 ? 'up' : 'down'">
                  <el-icon><CaretTop v-if="metrics.readQpsTrend > 0" /><CaretBottom v-else /></el-icon>
                  {{ Math.abs(metrics.readQpsTrend) }}%
                </div>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="metric-item">
                <div class="metric-title">写入 QPS</div>
                <div class="metric-value">{{ metrics.writeQps }}</div>
                <div class="metric-trend" :class="metrics.writeQpsTrend > 0 ? 'up' : 'down'">
                  <el-icon><CaretTop v-if="metrics.writeQpsTrend > 0" /><CaretBottom v-else /></el-icon>
                  {{ Math.abs(metrics.writeQpsTrend) }}%
                </div>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="metric-item">
                <div class="metric-title">事务成功率</div>
                <div class="metric-value">{{ metrics.transactionSuccessRate }}%</div>
                <div class="metric-trend" :class="metrics.transactionSuccessRateTrend > 0 ? 'up' : 'down'">
                  <el-icon><CaretTop v-if="metrics.transactionSuccessRateTrend > 0" /><CaretBottom v-else /></el-icon>
                  {{ Math.abs(metrics.transactionSuccessRateTrend) }}%
                </div>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="metric-item">
                <div class="metric-title">磁盘占用</div>
                <div class="metric-value">{{ formatSize(metrics.diskUsage) }}</div>
                <div class="metric-trend" :class="metrics.diskUsageTrend > 0 ? 'up' : 'down'">
                  <el-icon><CaretTop v-if="metrics.diskUsageTrend > 0" /><CaretBottom v-else /></el-icon>
                  {{ Math.abs(metrics.diskUsageTrend) }}%
                </div>
              </div>
            </el-col>
          </el-row>
          
          <el-row :gutter="10" style="margin-top: 10px;">
            <el-col :span="12">
              <div class="chart-container">
                <h4>QPS 趋势</h4>
                <div ref="qpsChartRef" style="height: 250px;"></div>
              </div>
            </el-col>
            <el-col :span="12">
              <div class="chart-container">
                <h4>磁盘占用趋势</h4>
                <div ref="diskChartRef" style="height: 250px;"></div>
              </div>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
      
      <!-- 日志查看器 -->
      <el-col :span="24" style="margin-top: 10px;">
        <el-card class="logs-card">
          <template #header>
            <div class="card-header">
              <span>日志查看器</span>
              <div class="header-actions">
                <el-select v-model="logLevel" size="small" style="width: 100px; margin-right: 10px;" @change="filterLogs">
                  <el-option label="全部" value="" />
                  <el-option label="错误" value="error" />
                  <el-option label="警告" value="warn" />
                  <el-option label="信息" value="info" />
                  <el-option label="调试" value="debug" />
                </el-select>
                <el-input
                  v-model="logSearch"
                  placeholder="搜索日志"
                  size="small"
                  style="width: 200px; margin-right: 10px;"
                  @input="filterLogs"
                >
                  <template #prefix>
                    <el-icon><Search /></el-icon>
                  </template>
                </el-input>
                <el-button size="small" @click="refreshLogs">
                  <el-icon><Refresh /></el-icon>
                  刷新
                </el-button>
                <el-button size="small" @click="exportLogs">
                  <el-icon><Download /></el-icon>
                  导出
                </el-button>
              </div>
            </div>
          </template>
          
          <div class="log-container">
            <div
              v-for="(log, index) in filteredLogs"
              :key="index"
              class="log-item"
              :class="log.level"
            >
              <div class="log-header">
                <span class="log-time">{{ formatTime(log.timestamp) }}</span>
                <el-tag :type="getLogLevelType(log.level)" size="small">{{ log.level.toUpperCase() }}</el-tag>
                <span class="log-source">{{ log.source }}</span>
              </div>
              <div class="log-message">{{ log.message }}</div>
              <div v-if="log.details" class="log-details">
                <el-collapse>
                  <el-collapse-item title="详细信息" name="details">
                    <pre>{{ log.details }}</pre>
                  </el-collapse-item>
                </el-collapse>
              </div>
            </div>
            
            <div v-if="filteredLogs.length === 0" class="no-logs">
              <el-empty description="暂无日志" />
            </div>
          </div>
          
          <div class="log-pagination">
            <el-pagination
              v-model:current-page="logPage"
              :page-size="logPageSize"
              :total="logs.length"
              layout="prev, pager, next, jumper, total"
              @current-change="handleLogPageChange"
            />
          </div>
        </el-card>
      </el-col>
      
      <!-- 数据库状态 -->
      <el-col :span="24" style="margin-top: 10px;">
        <el-card class="status-card">
          <template #header>
            <div class="card-header">
              <span>数据库状态</span>
              <el-button size="small" @click="refreshStatus">
                <el-icon><Refresh /></el-icon>
                刷新
              </el-button>
            </div>
          </template>
          
          <el-row :gutter="10">
            <el-col :span="8">
              <el-descriptions title="基本信息" :column="1" border>
                <el-descriptions-item label="连接状态">
                  <el-tag :type="dbStatus.connected ? 'success' : 'danger'">
                    {{ dbStatus.connected ? '已连接' : '未连接' }}
                  </el-tag>
                </el-descriptions-item>
                <el-descriptions-item label="数据库路径">{{ dbStatus.path }}</el-descriptions-item>
                <el-descriptions-item label="数据库大小">{{ formatSize(dbStatus.size) }}</el-descriptions-item>
                <el-descriptions-item label="创建时间">{{ formatTime(dbStatus.created_at) }}</el-descriptions-item>
                <el-descriptions-item label="最后修改">{{ formatTime(dbStatus.last_modified) }}</el-descriptions-item>
              </el-descriptions>
            </el-col>
            <el-col :span="8">
              <el-descriptions title="统计信息" :column="1" border>
                <el-descriptions-item label="键值对数量">{{ dbStats.key_count }}</el-descriptions-item>
                <el-descriptions-item label="树数量">{{ dbStats.tree_count }}</el-descriptions-item>
                <el-descriptions-item label="活跃事务数">{{ dbStats.active_transactions }}</el-descriptions-item>
                <el-descriptions-item label="已完成事务数">{{ dbStats.completed_transactions }}</el-descriptions-item>
                <el-descriptions-item label="事务成功率">{{ dbStats.transaction_success_rate }}%</el-descriptions-item>
              </el-descriptions>
            </el-col>
            <el-col :span="8">
              <el-descriptions title="性能指标" :column="1" border>
                <el-descriptions-item label="平均读取延迟">{{ dbStats.avg_read_latency }}ms</el-descriptions-item>
                <el-descriptions-item label="平均写入延迟">{{ dbStats.avg_write_latency }}ms</el-descriptions-item>
                <el-descriptions-item label="缓存命中率">{{ dbStats.cache_hit_rate }}%</el-descriptions-item>
                <el-descriptions-item label="磁盘使用率">{{ dbStats.disk_usage_rate }}%</el-descriptions-item>
                <el-descriptions-item label="内存使用率">{{ dbStats.memory_usage_rate }}%</el-descriptions-item>
              </el-descriptions>
            </el-col>
          </el-row>
          
          <el-row style="margin-top: 20px;">
            <el-col :span="24">
              <div class="status-actions">
                <el-button type="primary" @click="compactDatabase">压缩数据库</el-button>
                <el-button type="warning" @click="repairDatabase">修复数据库</el-button>
                <el-button type="info" @click="clearCache">清除缓存</el-button>
                <el-button type="success" @click="backupDatabase">备份数据库</el-button>
              </div>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
    </el-row>
    
    <!-- 无连接提示 -->
    <el-row v-else>
      <el-col :span="24">
        <el-empty description="请先连接到数据库">
          <el-button type="primary" @click="$router.push('/connections')">创建连接</el-button>
        </el-empty>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh, CaretTop, CaretBottom, Search, Download } from '@element-plus/icons-vue'
import { useSledStore } from '../stores/sled'
import * as echarts from 'echarts'

const sledStore = useSledStore()

// 状态
const currentConnection = computed(() => {
  if (!sledStore.currentConnectionId) return null
  return sledStore.connections.find(c => c.id === sledStore.currentConnectionId)
})

const timeRange = ref('5m')
const logLevel = ref('')
const logSearch = ref('')
const logPage = ref(1)
const logPageSize = ref(50)
const qpsChartRef = ref<HTMLElement>()
const diskChartRef = ref<HTMLElement>()
let qpsChart: echarts.ECharts | null = null
let diskChart: echarts.ECharts | null = null
let metricsTimer: ReturnType<typeof setTimeout> | null = null

// 性能指标
const metrics = reactive({
  readQps: 0,
  writeQps: 0,
  transactionSuccessRate: 0,
  diskUsage: 0,
  readQpsTrend: 0,
  writeQpsTrend: 0,
  transactionSuccessRateTrend: 0,
  diskUsageTrend: 0
})

// 日志数据
const logs = ref<any[]>([])
const filteredLogs = computed(() => {
  let result = logs.value
  
  // 级别过滤
  if (logLevel.value) {
    result = result.filter(log => log.level === logLevel.value)
  }
  
  // 搜索过滤
  if (logSearch.value) {
    const searchLower = logSearch.value.toLowerCase()
    result = result.filter(log => 
      log.message.toLowerCase().includes(searchLower) ||
      log.source.toLowerCase().includes(searchLower)
    )
  }
  
  // 分页
  const start = (logPage.value - 1) * logPageSize.value
  const end = start + logPageSize.value
  return result.slice(start, end)
})

// 数据库状态
const dbStatus = reactive({
  connected: false,
  path: '',
  size: 0,
  created_at: 0,
  last_modified: 0
})

const dbStats = reactive({
  key_count: 0,
  tree_count: 0,
  active_transactions: 0,
  completed_transactions: 0,
  transaction_success_rate: 0,
  avg_read_latency: 0,
  avg_write_latency: 0,
  cache_hit_rate: 0,
  disk_usage_rate: 0,
  memory_usage_rate: 0
})

// 方法
const formatSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const formatTime = (timestamp: number) => {
  return new Date(timestamp).toLocaleString()
}

const getLogLevelType = (level: string) => {
  const typeMap: Record<string, string> = {
    'error': 'danger',
    'warn': 'warning',
    'info': 'info',
    'debug': ''
  }
  return typeMap[level] || ''
}

const initCharts = () => {
  if (qpsChartRef.value) {
    qpsChart = echarts.init(qpsChartRef.value)
    
    const qpsOption = {
      tooltip: {
        trigger: 'axis'
      },
      legend: {
        data: ['读取 QPS', '写入 QPS']
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: generateTimeLabels()
      },
      yAxis: {
        type: 'value'
      },
      series: [
        {
          name: '读取 QPS',
          type: 'line',
          smooth: true,
          data: generateRandomData(20, 100)
        },
        {
          name: '写入 QPS',
          type: 'line',
          smooth: true,
          data: generateRandomData(10, 50)
        }
      ]
    }
    
    qpsChart.setOption(qpsOption)
  }
  
  if (diskChartRef.value) {
    diskChart = echarts.init(diskChartRef.value)
    
    const diskOption = {
      tooltip: {
        trigger: 'axis'
      },
      legend: {
        data: ['磁盘占用']
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: generateTimeLabels()
      },
      yAxis: {
        type: 'value',
        axisLabel: {
          formatter: function(value: number) {
            return formatSize(value)
          }
        }
      },
      series: [
        {
          name: '磁盘占用',
          type: 'line',
          smooth: true,
          data: generateIncreasingData(1000000, 2000000)
        }
      ]
    }
    
    diskChart.setOption(diskOption)
  }
}

const generateTimeLabels = () => {
  const labels = []
  const now = new Date()
  
  for (let i = 29; i >= 0; i--) {
    const time = new Date(now.getTime() - i * 60000) // 每分钟一个点
    labels.push(time.toLocaleTimeString())
  }
  
  return labels
}

const generateRandomData = (min: number, max: number) => {
  const data = []
  for (let i = 0; i < 30; i++) {
    data.push(Math.floor(Math.random() * (max - min + 1)) + min)
  }
  return data
}

const generateIncreasingData = (min: number, max: number) => {
  const data = []
  let current = min
  
  for (let i = 0; i < 30; i++) {
    data.push(current)
    // 模拟缓慢增长
    current += Math.floor(Math.random() * 10000)
    if (current > max) current = max
  }
  
  return data
}

const refreshMetrics = async () => {
  if (!currentConnection.value) return
  
  try {
    // 模拟获取性能指标
    // 实际项目中，这里应该调用后端API获取真实数据
    metrics.readQps = Math.floor(Math.random() * 100) + 20
    metrics.writeQps = Math.floor(Math.random() * 50) + 10
    metrics.transactionSuccessRate = Math.floor(Math.random() * 10) + 90
    metrics.diskUsage = Math.floor(Math.random() * 1000000) + 1000000
    
    // 模拟趋势变化
    metrics.readQpsTrend = Math.floor(Math.random() * 20) - 10
    metrics.writeQpsTrend = Math.floor(Math.random() * 20) - 10
    metrics.transactionSuccessRateTrend = Math.floor(Math.random() * 10) - 5
    metrics.diskUsageTrend = Math.floor(Math.random() * 10)
    
    // 更新图表
    if (qpsChart) {
      qpsChart.setOption({
        series: [
          {
            data: generateRandomData(20, 100)
          },
          {
            data: generateRandomData(10, 50)
          }
        ]
      })
    }
    
    if (diskChart) {
      diskChart.setOption({
        series: [
          {
            data: generateIncreasingData(1000000, 2000000)
          }
        ]
      })
    }
  } catch (error) {
    ElMessage.error('刷新性能指标失败')
  }
}

const handleTimeRangeChange = () => {
  refreshMetrics()
}

const refreshLogs = async () => {
  if (!currentConnection.value) return
  
  try {
    // 模拟获取日志数据
    // 实际项目中，这里应该调用后端API获取真实日志
    const mockLogs = []
    const now = Date.now()
    const levels = ['error', 'warn', 'info', 'debug']
    const sources = ['sled-core', 'transaction', 'cache', 'disk-io']
    const messages = [
      'Database operation completed successfully',
      'Transaction committed',
      'Cache miss for key',
      'Disk write operation completed',
      'Connection established',
      'Error reading from disk',
      'Warning: low disk space',
      'Debug: processing query'
    ]
    
    for (let i = 0; i < 100; i++) {
      mockLogs.push({
        timestamp: now - i * 60000, // 每分钟一条日志
        level: levels[Math.floor(Math.random() * levels.length)],
        source: sources[Math.floor(Math.random() * sources.length)],
        message: messages[Math.floor(Math.random() * messages.length)],
        details: Math.random() > 0.7 ? `Stack trace:\n  at function1 (file.js:10:5)\n  at function2 (file.js:20:10)` : null
      })
    }
    
    logs.value = mockLogs
    logPage.value = 1
  } catch (error) {
    ElMessage.error('刷新日志失败')
  }
}

const filterLogs = () => {
  logPage.value = 1
}

const handleLogPageChange = () => {
  // 页面变化时不需要额外操作
}

const exportLogs = () => {
  if (logs.value.length === 0) {
    ElMessage.warning('没有日志可导出')
    return
  }
  
  try {
    // 转换数据格式
    const jsonData = logs.value.map(log => ({
      timestamp: formatTime(log.timestamp),
      level: log.level,
      source: log.source,
      message: log.message
    }))
    
    // 创建下载链接
    const blob = new Blob([JSON.stringify(jsonData, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = `logs_${new Date().toISOString().slice(0, 10)}.json`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
    
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败')
  }
}

const refreshStatus = async () => {
  if (!currentConnection.value) return
  
  try {
    // 获取数据库状态
    dbStatus.connected = true
    dbStatus.path = currentConnection.value.path
    dbStatus.size = Math.floor(Math.random() * 10000000) + 1000000
    dbStatus.created_at = Date.now() - Math.floor(Math.random() * 86400000 * 30) // 30天内
    dbStatus.last_modified = Date.now() - Math.floor(Math.random() * 86400000) // 1天内
    
    // 获取统计信息
    dbStats.key_count = Math.floor(Math.random() * 10000) + 1000
    dbStats.tree_count = Math.floor(Math.random() * 10) + 1
    dbStats.active_transactions = Math.floor(Math.random() * 5)
    dbStats.completed_transactions = Math.floor(Math.random() * 1000) + 100
    dbStats.transaction_success_rate = Math.floor(Math.random() * 10) + 90
    dbStats.avg_read_latency = Math.floor(Math.random() * 10) + 1
    dbStats.avg_write_latency = Math.floor(Math.random() * 20) + 5
    dbStats.cache_hit_rate = Math.floor(Math.random() * 30) + 70
    dbStats.disk_usage_rate = Math.floor(Math.random() * 30) + 40
    dbStats.memory_usage_rate = Math.floor(Math.random() * 40) + 30
  } catch (error) {
    ElMessage.error('刷新状态失败')
  }
}

const compactDatabase = async () => {
  if (!currentConnection.value) return
  
  try {
    await ElMessageBox.confirm(
      '确定要压缩数据库吗？此操作可能需要一些时间。',
      '压缩确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 模拟压缩操作
    ElMessage.info('正在压缩数据库...')
    
    setTimeout(() => {
      ElMessage.success('数据库压缩完成')
      refreshStatus()
    }, 2000)
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('压缩失败')
    }
  }
}

const repairDatabase = async () => {
  if (!currentConnection.value) return
  
  try {
    await ElMessageBox.confirm(
      '确定要修复数据库吗？请确保已备份重要数据。',
      '修复确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 模拟修复操作
    ElMessage.info('正在修复数据库...')
    
    setTimeout(() => {
      ElMessage.success('数据库修复完成')
      refreshStatus()
    }, 3000)
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('修复失败')
    }
  }
}

const clearCache = async () => {
  if (!currentConnection.value) return
  
  try {
    // 模拟清除缓存操作
    ElMessage.info('正在清除缓存...')
    
    setTimeout(() => {
      ElMessage.success('缓存清除完成')
      refreshStatus()
    }, 1000)
  } catch (error) {
    ElMessage.error('清除缓存失败')
  }
}

const backupDatabase = async () => {
  if (!currentConnection.value) return
  
  try {
    // 模拟备份操作
    ElMessage.info('正在备份数据库...')
    
    setTimeout(() => {
      ElMessage.success('数据库备份完成')
    }, 2000)
  } catch (error) {
    ElMessage.error('备份失败')
  }
}

// 初始化
onMounted(async () => {
  await nextTick()
  initCharts()
  
  if (currentConnection.value) {
    refreshMetrics()
    refreshLogs()
    refreshStatus()
    
    // 定时刷新性能指标
    metricsTimer = setInterval(() => {
      refreshMetrics()
    }, 5000)
  }
  
  // 监听窗口大小变化，调整图表
  window.addEventListener('resize', () => {
    if (qpsChart) qpsChart.resize()
    if (diskChart) diskChart.resize()
  })
})

onUnmounted(() => {
  if (metricsTimer) {
    clearInterval(metricsTimer)
  }
  
  if (qpsChart) {
    qpsChart.dispose()
    qpsChart = null
  }
  
  if (diskChart) {
    diskChart.dispose()
    diskChart = null
  }
  
  window.removeEventListener('resize', () => {})
})
</script>

<style scoped>
.monitor-container {
  height: 100%;
}

.metrics-card, .logs-card, .status-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-actions {
  display: flex;
  align-items: center;
}

.metric-item {
  background-color: var(--el-bg-color-page);
  border-radius: 8px;
  padding: 15px;
  text-align: center;
  height: 100px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.metric-title {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  margin-bottom: 5px;
}

.metric-value {
  font-size: 24px;
  font-weight: bold;
  margin-bottom: 5px;
}

.metric-trend {
  font-size: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2px;
}

.metric-trend.up {
  color: var(--el-color-success);
}

.metric-trend.down {
  color: var(--el-color-danger);
}

.chart-container {
  background-color: var(--el-bg-color-page);
  border-radius: 8px;
  padding: 15px;
}

.chart-container h4 {
  margin: 0 0 10px 0;
  font-size: 16px;
  font-weight: normal;
}

.log-container {
  height: 400px;
  overflow-y: auto;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 10px;
  background-color: var(--el-bg-color-page);
}

.log-item {
  margin-bottom: 15px;
  padding: 10px;
  border-radius: 4px;
  background-color: var(--el-bg-color);
}

.log-item.error {
  border-left: 4px solid var(--el-color-danger);
}

.log-item.warn {
  border-left: 4px solid var(--el-color-warning);
}

.log-item.info {
  border-left: 4px solid var(--el-color-info);
}

.log-item.debug {
  border-left: 4px solid var(--el-text-color-secondary);
}

.log-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 5px;
}

.log-time {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.log-source {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin-left: auto;
}

.log-message {
  margin-bottom: 5px;
  word-break: break-word;
}

.log-details pre {
  background-color: var(--el-bg-color-page);
  padding: 10px;
  border-radius: 4px;
  overflow-x: auto;
  font-size: 12px;
  margin: 0;
}

.no-logs {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
}

.log-pagination {
  display: flex;
  justify-content: center;
  margin-top: 15px;
}

.status-actions {
  display: flex;
  gap: 10px;
  justify-content: center;
}
</style>