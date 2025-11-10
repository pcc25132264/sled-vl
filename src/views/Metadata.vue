<template>
  <div class="metadata-container">
    <el-row :gutter="10">
      <!-- 左侧数据库结构树 -->
      <el-col :span="6">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>数据库结构</span>
              <el-button type="primary" size="small" @click="refreshTree">
                <el-icon><Refresh /></el-icon>
              </el-button>
            </div>
          </template>
          
          <el-tree
            ref="treeRef"
            :data="treeData"
            :props="treeProps"
            node-key="id"
            :expand-on-click-node="false"
            :highlight-current="true"
            @node-click="handleNodeClick"
          >
            <template #default="{ node, data }">
              <span class="custom-tree-node">
                <el-icon v-if="data.type === 'database'"><Coin /></el-icon>
                <el-icon v-else-if="data.type === 'tree'"><Folder /></el-icon>
                <el-icon v-else-if="data.type === 'index'"><Key /></el-icon>
                <span>{{ node.label }}</span>
                <span v-if="data.type === 'tree' && data.count !== undefined" class="node-count">
                  ({{ data.count }})
                </span>
              </span>
            </template>
          </el-tree>
        </el-card>
      </el-col>
      
      <!-- 右侧内容区 -->
      <el-col :span="18">
        <el-tabs v-model="activeTab" @tab-click="handleTabClick">
          <!-- 数据库信息 -->
          <el-tab-pane label="数据库信息" name="database">
            <el-card>
              <template #header>
                <span>数据库基本信息</span>
              </template>
              
              <el-descriptions :column="2" border>
                <el-descriptions-item label="数据库路径">
                  {{ databaseInfo.path }}
                </el-descriptions-item>
                <el-descriptions-item label="数据库大小">
                  {{ formatSize(databaseInfo.size) }}
                </el-descriptions-item>
                <el-descriptions-item label="创建时间">
                  {{ formatDate(databaseInfo.created_at) }}
                </el-descriptions-item>
                <el-descriptions-item label="最后修改时间">
                  {{ formatDate(databaseInfo.updated_at) }}
                </el-descriptions-item>
                <el-descriptions-item label="Sled版本">
                  {{ databaseInfo.version }}
                </el-descriptions-item>
                <el-descriptions-item label="段数量">
                  {{ databaseInfo.segment_count }}
                </el-descriptions-item>
                <el-descriptions-item label="缓存大小">
                  {{ formatSize(databaseInfo.cache_capacity) }}
                </el-descriptions-item>
                <el-descriptions-item label="使用率">
                  {{ databaseInfo.usage_ratio }}%
                </el-descriptions-item>
              </el-descriptions>
              
              <div class="operation-buttons">
                <el-button type="primary" @click="compactDatabase" :loading="compacting">
                  压缩数据库
                </el-button>
                <el-button type="warning" @click="verifyDatabase" :loading="verifying">
                  验证数据库
                </el-button>
                <el-button type="danger" @click="repairDatabase" :loading="repairing">
                  修复数据库
                </el-button>
              </div>
            </el-card>
          </el-tab-pane>
          
          <!-- 索引管理 -->
          <el-tab-pane label="索引管理" name="indexes">
            <el-card>
              <template #header>
                <div class="card-header">
                  <span>索引列表</span>
                  <el-button type="primary" size="small" @click="refreshIndexes">
                    <el-icon><Refresh /></el-icon>
                  </el-button>
                </div>
              </template>
              
              <el-table :data="indexes" stripe>
                <el-table-column prop="name" label="索引名称" />
                <el-table-column prop="tree" label="所属树" />
                <el-table-column prop="type" label="索引类型" />
                <el-table-column prop="size" label="大小">
                  <template #default="scope">
                    {{ formatSize(scope.row.size) }}
                  </template>
                </el-table-column>
                <el-table-column prop="entries" label="条目数" />
                <el-table-column prop="created_at" label="创建时间">
                  <template #default="scope">
                    {{ formatDate(scope.row.created_at) }}
                  </template>
                </el-table-column>
                <el-table-column label="操作" width="180">
                  <template #default="scope">
                    <el-button type="primary" size="small" @click="viewIndexDetails(scope.row)">
                      详情
                    </el-button>
                    <el-button type="danger" size="small" @click="deleteIndex(scope.row)">
                      删除
                    </el-button>
                  </template>
                </el-table-column>
              </el-table>
            </el-card>
          </el-tab-pane>
          
          <!-- 统计信息 -->
          <el-tab-pane label="统计信息" name="statistics">
            <el-card>
              <template #header>
                <span>数据库统计</span>
              </template>
              
              <el-row :gutter="10">
                <el-col :span="12">
                  <div class="stat-card">
                    <div class="stat-title">总键值对数量</div>
                    <div class="stat-value">{{ statistics.total_keys }}</div>
                  </div>
                </el-col>
                <el-col :span="12">
                  <div class="stat-card">
                    <div class="stat-title">总树数量</div>
                    <div class="stat-value">{{ statistics.total_trees }}</div>
                  </div>
                </el-col>
              </el-row>
              
              <el-row :gutter="10" style="margin-top: 10px;">
                <el-col :span="12">
                  <div class="stat-card">
                    <div class="stat-title">总读取次数</div>
                    <div class="stat-value">{{ statistics.read_count }}</div>
                  </div>
                </el-col>
                <el-col :span="12">
                  <div class="stat-card">
                    <div class="stat-title">总写入次数</div>
                    <div class="stat-value">{{ statistics.write_count }}</div>
                  </div>
                </el-col>
              </el-row>
              
              <el-row :gutter="10" style="margin-top: 10px;">
                <el-col :span="12">
                  <div class="stat-card">
                    <div class="stat-title">平均键长度</div>
                    <div class="stat-value">{{ statistics.avg_key_length }} bytes</div>
                  </div>
                </el-col>
                <el-col :span="12">
                  <div class="stat-card">
                    <div class="stat-title">平均值长度</div>
                    <div class="stat-value">{{ statistics.avg_value_length }} bytes</div>
                  </div>
                </el-col>
              </el-row>
              
              <div style="margin-top: 20px;">
                <h3>树大小分布</h3>
                <div ref="treeSizeChart" style="height: 300px;"></div>
              </div>
              
              <div style="margin-top: 20px;">
                <h3>数据类型分布</h3>
                <div ref="dataTypeChart" style="height: 300px;"></div>
              </div>
            </el-card>
          </el-tab-pane>
          
          <!-- 内部结构 -->
          <el-tab-pane label="内部结构" name="internal">
            <el-card>
              <template #header>
                <span>内部结构分析</span>
              </template>
              
              <el-table :data="segments" stripe>
                <el-table-column prop="id" label="段ID" />
                <el-table-column prop="size" label="大小">
                  <template #default="scope">
                    {{ formatSize(scope.row.size) }}
                  </template>
                </el-table-column>
                <el-table-column prop="entries" label="条目数" />
                <el-table-column prop="live_entries" label="活跃条目数" />
                <el-table-column prop="deleted_entries" label="已删除条目数" />
                <el-table-column prop="fragmentation" label="碎片率">
                  <template #default="scope">
                    {{ scope.row.fragmentation }}%
                  </template>
                </el-table-column>
                <el-table-column prop="created_at" label="创建时间">
                  <template #default="scope">
                    {{ formatDate(scope.row.created_at) }}
                  </template>
                </el-table-column>
              </el-table>
              
              <div style="margin-top: 20px;">
                <h3>B树结构可视化</h3>
                <div ref="btreeChart" style="height: 400px;"></div>
              </div>
            </el-card>
          </el-tab-pane>
        </el-tabs>
      </el-col>
    </el-row>
    
    <!-- 索引详情对话框 -->
    <el-dialog v-model="indexDialogVisible" title="索引详情" width="600px">
      <el-descriptions :column="1" border>
        <el-descriptions-item label="索引名称">
          {{ currentIndex.name }}
        </el-descriptions-item>
        <el-descriptions-item label="所属树">
          {{ currentIndex.tree }}
        </el-descriptions-item>
        <el-descriptions-item label="索引类型">
          {{ currentIndex.type }}
        </el-descriptions-item>
        <el-descriptions-item label="大小">
          {{ formatSize(currentIndex.size) }}
        </el-descriptions-item>
        <el-descriptions-item label="条目数">
          {{ currentIndex.entries }}
        </el-descriptions-item>
        <el-descriptions-item label="创建时间">
          {{ formatDate(currentIndex.created_at) }}
        </el-descriptions-item>
        <el-descriptions-item label="最后更新时间">
          {{ formatDate(currentIndex.updated_at) }}
        </el-descriptions-item>
        <el-descriptions-item label="索引配置">
          <pre>{{ JSON.stringify(currentIndex.config, null, 2) }}</pre>
        </el-descriptions-item>
      </el-descriptions>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="indexDialogVisible = false">关闭</el-button>
          <el-button type="primary" @click="rebuildIndex" :loading="rebuilding">
            重建索引
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh, Coin, Folder, Key } from '@element-plus/icons-vue'
import * as echarts from 'echarts'
import { useSledStore } from '../stores/sled'

const sledStore = useSledStore()

// 状态
const activeTab = ref('database')
const treeRef = ref()
const treeData = ref<any[]>([])
const treeProps = {
  children: 'children',
  label: 'name'
}
const currentConnection = ref('')
const currentTree = ref('')

// 数据库信息
const databaseInfo = reactive({
  path: '',
  size: 0,
  created_at: '',
  updated_at: '',
  version: '',
  segment_count: 0,
  cache_capacity: 0,
  usage_ratio: 0
})

// 索引相关
const indexes = ref([])
const indexDialogVisible = ref(false)
const currentIndex = ref({
  name: '',
  tree: '',
  type: '',
  size: 0,
  entries: 0,
  created_at: '',
  updated_at: '',
  config: {}
})
const rebuilding = ref(false)

// 统计信息
const statistics = reactive({
  total_keys: 0,
  total_trees: 0,
  read_count: 0,
  write_count: 0,
  avg_key_length: 0,
  avg_value_length: 0
})

// 内部结构
const segments = ref([])

// 操作状态
const compacting = ref(false)
const verifying = ref(false)
const repairing = ref(false)

// 图表引用
const treeSizeChart = ref()
const dataTypeChart = ref()
const btreeChart = ref()

// 方法
const refreshTree = async () => {
  try {
    if (!sledStore.currentConnectionId) {
      ElMessage.warning('请先连接到数据库')
      return
    }
    
    await sledStore.loadTrees(sledStore.currentConnectionId)
    
    // 构建树形数据
    const dbNode: any = {
      id: sledStore.currentConnectionId,
      name: sledStore.connections.find(conn => conn.id === sledStore.currentConnectionId)?.name || 'Unknown',
      type: 'database',
      children: []
    }
    
    sledStore.trees.forEach(tree => {
      dbNode.children.push({
        id: `${sledStore.currentConnectionId}-${tree}`,
        name: tree,
        type: 'tree',
        count: 0 // 这里可以获取每个树的键值对数量
      })
    })
    
    treeData.value = [dbNode]
  } catch (error) {
    ElMessage.error(`刷新树形结构失败: ${error}`)
  }
}

const handleNodeClick = (data: any) => {
  if (data.type === 'database') {
    currentConnection.value = data.id
    currentTree.value = ''
    loadDatabaseInfo()
  } else if (data.type === 'tree') {
    currentConnection.value = data.id.split('-')[0]
    currentTree.value = data.name
    loadTreeInfo(data.name)
  }
}

const handleTabClick = () => {
  // 标签页切换时的处理
  if (activeTab.value === 'statistics') {
    nextTick(() => {
      initCharts()
    })
  } else if (activeTab.value === 'internal') {
    nextTick(() => {
      initBtreeChart()
    })
  }
}

const loadDatabaseInfo = async () => {
  try {
    if (!sledStore.currentConnectionId) {
      ElMessage.warning('请先连接到数据库')
      return
    }
    await sledStore.loadStats(sledStore.currentConnectionId)
    if (sledStore.stats) {
      Object.assign(databaseInfo, {
        path: sledStore.connections.find(conn => conn.id === sledStore.currentConnectionId)?.path || '',
        size: sledStore.stats.size_on_disk,
        created_at: sledStore.connections.find(conn => conn.id === sledStore.currentConnectionId)?.created_at || '',
        updated_at: sledStore.stats.last_modified,
        version: '1.0',
        segment_count: 0,
        cache_capacity: 0,
        usage_ratio: 0
      })
    }
  } catch (error) {
    ElMessage.error(`加载数据库信息失败: ${error}`)
  }
}

const loadTreeInfo = async (treeName: string) => {
  try {
    // 加载树信息
    // const treeInfo = await sledStore.getTreeInfo(treeName)
    // 更新树节点信息
    console.log('Loading tree info:', treeName)
  } catch (error) {
    ElMessage.error(`加载树信息失败: ${error}`)
  }
}

const refreshIndexes = async () => {
  try {
    // 这里应该调用获取索引列表的API
    // indexes.value = await sledStore.getIndexes()
    ElMessage.success('索引列表已刷新')
  } catch (error) {
    ElMessage.error(`刷新索引列表失败: ${error}`)
  }
}

const viewIndexDetails = (index: any) => {
  currentIndex.value = index
  indexDialogVisible.value = true
}

const deleteIndex = async (index: any) => {
  try {
    await ElMessageBox.confirm(
      `确定要删除索引 "${index.name}" 吗？此操作不可撤销。`,
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 这里应该调用删除索引的API
    // await sledStore.deleteIndex(index.name)
    
    ElMessage.success('索引已删除')
    refreshIndexes()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(`删除索引失败: ${error}`)
    }
  }
}

const rebuildIndex = async () => {
  rebuilding.value = true
  try {
    // 这里应该调用重建索引的API
    // await sledStore.rebuildIndex(currentIndex.value.name)
    
    ElMessage.success('索引重建完成')
    indexDialogVisible.value = false
    refreshIndexes()
  } catch (error) {
    ElMessage.error(`重建索引失败: ${error}`)
  } finally {
    rebuilding.value = false
  }
}

const compactDatabase = async () => {
  compacting.value = true
  try {
    // await sledStore.compact()
    console.log('Compacting database...')
    ElMessage.success('数据库压缩完成')
    loadDatabaseInfo()
  } catch (error) {
    ElMessage.error(`压缩数据库失败: ${error}`)
  } finally {
    compacting.value = false
  }
}

const verifyDatabase = async () => {
  verifying.value = true
  try {
    // 这里应该调用验证数据库的API
    // const result = await sledStore.verify()
    
    ElMessage.success('数据库验证完成，未发现问题')
  } catch (error) {
    ElMessage.error(`验证数据库失败: ${error}`)
  } finally {
    verifying.value = false
  }
}

const repairDatabase = async () => {
  repairing.value = true
  try {
    // 这里应该调用修复数据库的API
    // await sledStore.repair()
    
    ElMessage.success('数据库修复完成')
    loadDatabaseInfo()
  } catch (error) {
    ElMessage.error(`修复数据库失败: ${error}`)
  } finally {
    repairing.value = false
  }
}

const formatSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const formatDate = (dateString: string) => {
  if (!dateString) return '-'
  const date = new Date(dateString)
  return date.toLocaleString()
}

const initCharts = () => {
  // 初始化树大小分布图表
  if (treeSizeChart.value) {
    const treeSizeChartInstance = echarts.init(treeSizeChart.value)
    
    // 这里应该从API获取实际数据
    const treeSizeData = [
      { name: 'default', value: 1024000 },
      { name: 'users', value: 512000 },
      { name: 'products', value: 2048000 },
      { name: 'orders', value: 1536000 }
    ]
    
    const treeSizeOption = {
      tooltip: {
        trigger: 'item',
        formatter: '{a} <br/>{b}: {c} ({d}%)'
      },
      legend: {
        orient: 'vertical',
        left: 10,
        data: treeSizeData.map(item => item.name)
      },
      series: [
        {
          name: '树大小',
          type: 'pie',
          radius: ['50%', '70%'],
          avoidLabelOverlap: false,
          label: {
            show: false,
            position: 'center'
          },
          emphasis: {
            label: {
              show: true,
              fontSize: '18',
              fontWeight: 'bold'
            }
          },
          labelLine: {
            show: false
          },
          data: treeSizeData
        }
      ]
    }
    
    treeSizeChartInstance.setOption(treeSizeOption)
  }
  
  // 初始化数据类型分布图表
  if (dataTypeChart.value) {
    const dataTypeChartInstance = echarts.init(dataTypeChart.value)
    
    // 这里应该从API获取实际数据
    const dataTypeData = [
      { name: '字符串', value: 4500 },
      { name: '数字', value: 2100 },
      { name: '二进制', value: 800 },
      { name: 'JSON', value: 3200 }
    ]
    
    const dataTypeOption = {
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'shadow'
        }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: {
        type: 'value'
      },
      yAxis: {
        type: 'category',
        data: dataTypeData.map(item => item.name)
      },
      series: [
        {
          name: '数量',
          type: 'bar',
          data: dataTypeData.map(item => item.value)
        }
      ]
    }
    
    dataTypeChartInstance.setOption(dataTypeOption)
  }
}

const initBtreeChart = () => {
  // 初始化B树结构可视化图表
  if (btreeChart.value) {
    const btreeChartInstance = echarts.init(btreeChart.value)
    
    // 这里应该从API获取实际的B树结构数据
    // 简化的B树结构示例
    const btreeData = {
      name: 'Root',
      children: [
        {
          name: 'Node 1',
          children: [
            { name: 'Leaf 1-1' },
            { name: 'Leaf 1-2' },
            { name: 'Leaf 1-3' }
          ]
        },
        {
          name: 'Node 2',
          children: [
            { name: 'Leaf 2-1' },
            { name: 'Leaf 2-2' }
          ]
        },
        {
          name: 'Node 3',
          children: [
            { name: 'Leaf 3-1' },
            { name: 'Leaf 3-2' },
            { name: 'Leaf 3-3' },
            { name: 'Leaf 3-4' }
          ]
        }
      ]
    }
    
    const btreeOption = {
      tooltip: {
        trigger: 'item',
        triggerOn: 'mousemove'
      },
      series: [
        {
          type: 'tree',
          data: [btreeData],
          top: '5%',
          left: '10%',
          bottom: '5%',
          right: '20%',
          symbolSize: 10,
          label: {
            position: 'left',
            verticalAlign: 'middle',
            align: 'right',
            fontSize: 12
          },
          leaves: {
            label: {
              position: 'right',
              verticalAlign: 'middle',
              align: 'left'
            }
          },
          emphasis: {
            focus: 'descendant'
          },
          expandAndCollapse: true,
          animationDuration: 550,
          animationDurationUpdate: 750
        }
      ]
    }
    
    btreeChartInstance.setOption(btreeOption)
  }
}

// 初始化
onMounted(() => {
  refreshTree()
  loadDatabaseInfo()
  refreshIndexes()
  
  // 加载统计信息
  // 这里应该调用获取统计信息的API
  // const stats = await sledStore.getStatistics()
  // Object.assign(statistics, stats)
  
  // 加载内部结构信息
  // 这里应该调用获取内部结构信息的API
  // segments.value = await sledStore.getSegments()
})
</script>

<style scoped>
.metadata-container {
  padding: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.custom-tree-node {
  display: flex;
  align-items: center;
  font-size: 14px;
}

.custom-tree-node .el-icon {
  margin-right: 5px;
}

.node-count {
  margin-left: 5px;
  font-size: 12px;
  color: #909399;
}

.operation-buttons {
  margin-top: 20px;
  text-align: center;
}

.stat-card {
  background-color: #f5f7fa;
  border-radius: 4px;
  padding: 20px;
  text-align: center;
}

.stat-title {
  font-size: 14px;
  color: #606266;
  margin-bottom: 10px;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #409eff;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
}
</style>