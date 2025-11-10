<template>
  <div class="import-export-page">
    <el-card class="page-header">
      <div class="header-content">
        <h1>数据导入导出</h1>
        <p>管理数据的导入和导出操作，支持多种格式</p>
      </div>
    </el-card>

    <div class="page-content">
      <el-row :gutter="20">
        <!-- 导出数据 -->
        <el-col :span="12">
          <el-card class="operation-card">
            <template #header>
              <div class="card-header">
                <el-icon><Download /></el-icon>
                <span>导出数据</span>
              </div>
            </template>
            
            <el-form ref="exportFormRef" :model="exportForm" label-width="100px">
              <el-form-item label="连接">
                <el-select 
                  v-model="exportForm.connectionId" 
                  placeholder="选择连接"
                  style="width: 100%"
                  @change="loadTreesForExport"
                >
                  <el-option
                    v-for="connection in connections"
                    :key="connection.id"
                    :label="connection.name"
                    :value="connection.id"
                  />
                </el-select>
              </el-form-item>
              
              <el-form-item label="数据树">
                <el-select 
                  v-model="exportForm.treeName" 
                  placeholder="选择数据树"
                  style="width: 100%"
                >
                  <el-option
                    v-for="tree in exportTrees"
                    :key="tree"
                    :label="tree"
                    :value="tree"
                  />
                </el-select>
              </el-form-item>
              
              <el-form-item label="导出格式">
                <el-select 
                  v-model="exportForm.format" 
                  placeholder="选择导出格式"
                  style="width: 100%"
                >
                  <el-option label="JSON" value="json" />
                  <el-option label="CSV" value="csv" />
                  <el-option label="XML" value="xml" />
                  <el-option label="YAML" value="yaml" />
                </el-select>
              </el-form-item>
              
              <el-form-item label="保存路径">
                <el-input 
                  v-model="exportForm.filePath" 
                  placeholder="选择保存路径"
                  readonly
                >
                  <template #append>
                    <el-button @click="selectExportPath">选择</el-button>
                  </template>
                </el-input>
              </el-form-item>
              
              <el-form-item>
                <el-button 
                  type="primary" 
                  @click="exportData"
                  :loading="exportLoading"
                  :disabled="!canExport"
                >
                  导出数据
                </el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>
        
        <!-- 导入数据 -->
        <el-col :span="12">
          <el-card class="operation-card">
            <template #header>
              <div class="card-header">
                <el-icon><Upload /></el-icon>
                <span>导入数据</span>
              </div>
            </template>
            
            <el-form ref="importFormRef" :model="importForm" label-width="100px">
              <el-form-item label="连接">
                <el-select 
                  v-model="importForm.connectionId" 
                  placeholder="选择连接"
                  style="width: 100%"
                  @change="loadTreesForImport"
                >
                  <el-option
                    v-for="connection in connections"
                    :key="connection.id"
                    :label="connection.name"
                    :value="connection.id"
                  />
                </el-select>
              </el-form-item>
              
              <el-form-item label="数据树">
                <el-select 
                  v-model="importForm.treeName" 
                  placeholder="选择数据树"
                  style="width: 100%"
                >
                  <el-option
                    v-for="tree in importTrees"
                    :key="tree"
                    :label="tree"
                    :value="tree"
                  />
                </el-select>
              </el-form-item>
              
              <el-form-item label="文件格式">
                <el-select 
                  v-model="importForm.format" 
                  placeholder="选择文件格式"
                  style="width: 100%"
                >
                  <el-option label="JSON" value="json" />
                  <el-option label="CSV" value="csv" />
                  <el-option label="YAML" value="yaml" />
                </el-select>
              </el-form-item>
              
              <el-form-item label="文件路径">
                <el-input 
                  v-model="importForm.filePath" 
                  placeholder="选择导入文件"
                  readonly
                >
                  <template #append>
                    <el-button @click="selectImportPath">选择</el-button>
                  </template>
                </el-input>
              </el-form-item>
              
              <el-form-item>
                <el-button 
                  type="primary" 
                  @click="importData"
                  :loading="importLoading"
                  :disabled="!canImport"
                >
                  导入数据
                </el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>
      </el-row>
      
      <!-- 操作历史 -->
      <el-card class="history-card">
        <template #header>
          <div class="card-header">
            <el-icon><Clock /></el-icon>
            <span>操作历史</span>
          </div>
        </template>
        
        <el-table :data="operationHistory" style="width: 100%">
          <el-table-column prop="time" label="时间" width="180" />
          <el-table-column prop="type" label="类型" width="100">
            <template #default="scope">
              <el-tag :type="scope.row.type === '导出' ? 'success' : 'primary'">
                {{ scope.row.type }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="connection" label="连接" width="150" />
          <el-table-column prop="tree" label="数据树" width="150" />
          <el-table-column prop="format" label="格式" width="100" />
          <el-table-column prop="path" label="路径" />
          <el-table-column prop="status" label="状态" width="100">
            <template #default="scope">
              <el-tag :type="scope.row.status === '成功' ? 'success' : 'danger'">
                {{ scope.row.status }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="message" label="消息" />
        </el-table>
      </el-card>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Download, Upload, Clock } from '@element-plus/icons-vue'
import { useSledStore } from '../stores/sled'
import { invoke } from '@tauri-apps/api/core'
// import { open } from '@tauri-apps/api/dialog'

const sledStore = useSledStore()
const connections = computed(() => sledStore.connections)
const currentConnection = computed(() => sledStore.connections.find(c => c.id === sledStore.currentConnectionId))

// 导出表单
const exportForm = ref({
  connectionId: '',
  treeName: '',
  format: 'json',
  filePath: ''
})

// 导入表单
const importForm = ref({
  connectionId: '',
  treeName: '',
  format: 'json',
  filePath: ''
})

// 表单引用
const exportFormRef = ref()
const importFormRef = ref()

// 数据树列表
const exportTrees = ref([])
const importTrees = ref([])

// 加载状态
const exportLoading = ref(false)
const importLoading = ref(false)

// 操作历史
const operationHistory = ref([])

// 计算属性
const canExport = computed(() => {
  return exportForm.value.connectionId && 
         exportForm.value.treeName && 
         exportForm.value.format && 
         exportForm.value.filePath
})

const canImport = computed(() => {
  return importForm.value.connectionId && 
         importForm.value.treeName && 
         importForm.value.format && 
         importForm.value.filePath
})

// 加载数据树列表
const loadTreesForExport = async () => {
  if (!exportForm.value.connectionId) return
  
  try {
    const trees = await invoke('get_trees', { 
      connectionId: exportForm.value.connectionId 
    })
    exportTrees.value = trees
    
    // 默认选择第一个树
    if (trees.length > 0 && !exportForm.value.treeName) {
      exportForm.value.treeName = trees[0]
    }
  } catch (error) {
    ElMessage.error(`加载数据树失败: ${error}`)
  }
}

const loadTreesForImport = async () => {
  if (!importForm.value.connectionId) return
  
  try {
    const trees = await invoke('get_trees', { 
      connectionId: importForm.value.connectionId 
    })
    importTrees.value = trees
    
    // 默认选择第一个树
    if (trees.length > 0 && !importForm.value.treeName) {
      importForm.value.treeName = trees[0]
    }
  } catch (error) {
    ElMessage.error(`加载数据树失败: ${error}`)
  }
}

// 初始化
onMounted(async () => {
  await sledStore.loadConnections()
})

// 选择导出路径
const selectExportPath = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'JSON文件',
          extensions: ['json']
        },
        {
          name: 'CSV文件',
          extensions: ['csv']
        },
        {
          name: 'XML文件',
          extensions: ['xml']
        },
        {
          name: 'YAML文件',
          extensions: ['yaml', 'yml']
        }
      ]
    })
    
    if (selected) {
      exportForm.value.filePath = selected
    }
  } catch (error) {
    ElMessage.error(`选择文件失败: ${error}`)
  }
}

// 选择导入路径
const selectImportPath = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'JSON文件',
          extensions: ['json']
        },
        {
          name: 'CSV文件',
          extensions: ['csv']
        },
        {
          name: 'YAML文件',
          extensions: ['yaml', 'yml']
        }
      ]
    })
    
    if (selected) {
      importForm.value.filePath = selected
    }
  } catch (error) {
    ElMessage.error(`选择文件失败: ${error}`)
  }
}

// 导出数据
const exportData = async () => {
  if (!canExport.value) return
  
  exportLoading.value = true
  
  try {
    const connection = connections.value.find(c => c.id === exportForm.value.connectionId)
    
    const result = await invoke('export_data', {
      request: {
        connection_id: exportForm.value.connectionId,
        tree_name: exportForm.value.treeName,
        format: exportForm.value.format,
        file_path: exportForm.value.filePath
      }
    })
    
    ElMessage.success(result)
    
    // 添加到操作历史
    operationHistory.value.unshift({
      time: new Date().toLocaleString(),
      type: '导出',
      connection: connection.name,
      tree: exportForm.value.treeName,
      format: exportForm.value.format,
      path: exportForm.value.filePath,
      status: '成功',
      message: result
    })
    
    // 限制历史记录数量
    if (operationHistory.value.length > 20) {
      operationHistory.value = operationHistory.value.slice(0, 20)
    }
  } catch (error) {
    ElMessage.error(`导出失败: ${error}`)
    
    // 添加失败记录到历史
    operationHistory.value.unshift({
      time: new Date().toLocaleString(),
      type: '导出',
      connection: connection.name,
      tree: exportForm.value.treeName,
      format: exportForm.value.format,
      path: exportForm.value.filePath,
      status: '失败',
      message: error
    })
  } finally {
    exportLoading.value = false
  }
}

// 导入数据
const importData = async () => {
  if (!canImport.value) return
  
  // 确认导入
  try {
    await ElMessageBox.confirm(
      '导入数据可能会覆盖现有键值对，确定要继续吗？',
      '确认导入',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
  } catch {
    return
  }
  
  importLoading.value = true
  
  try {
    const connection = connections.value.find(c => c.id === importForm.value.connectionId)
    
    const result = await invoke('import_from_path', {
      request: {
        connection_id: importForm.value.connectionId,
        tree_name: importForm.value.treeName,
        format: importForm.value.format,
        file_path: importForm.value.filePath
      }
    })
    
    ElMessage.success(result)
    
    // 添加到操作历史
    operationHistory.value.unshift({
      time: new Date().toLocaleString(),
      type: '导入',
      connection: connection.name,
      tree: importForm.value.treeName,
      format: importForm.value.format,
      path: importForm.value.filePath,
      status: '成功',
      message: result
    })
    
    // 限制历史记录数量
    if (operationHistory.value.length > 20) {
      operationHistory.value = operationHistory.value.slice(0, 20)
    }
  } catch (error) {
    ElMessage.error(`导入失败: ${error}`)
    
    // 添加失败记录到历史
    operationHistory.value.unshift({
      time: new Date().toLocaleString(),
      type: '导入',
      connection: connection.name,
      tree: importForm.value.treeName,
      format: importForm.value.format,
      path: importForm.value.filePath,
      status: '失败',
      message: error
    })
  } finally {
    importLoading.value = false
  }
}

// 初始化
onMounted(async () => {
  // 加载连接列表
  await connectionStore.loadConnections()
  
  // 如果有当前连接，设置为默认
  if (currentConnection.value) {
    exportForm.value.connectionId = currentConnection.value.id
    importForm.value.connectionId = currentConnection.value.id
    
    // 加载数据树
    await loadTreesForExport()
    await loadTreesForImport()
  }
})
</script>

<style scoped>
.import-export-page {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.page-header {
  margin-bottom: 20px;
}

.header-content h1 {
  margin: 0 0 8px 0;
  font-size: 24px;
  font-weight: 600;
}

.header-content p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.page-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.operation-card {
  height: fit-content;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}

.history-card {
  margin-top: 20px;
}
</style>