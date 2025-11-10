<template>
  <div class="connections-container">
    <el-row :gutter="10">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>数据库连接管理</span>
              <el-button-group>
                <el-button 
                  type="primary" 
                  :icon="Plus" 
                  @click="showCreateDialog = true"
                >
                  新建连接
                </el-button>
                <el-button 
                  type="success" 
                  :icon="Plus" 
                  @click="showCreateDatabaseDialog = true"
                >
                  创建数据库
                </el-button>
              </el-button-group>
            </div>
          </template>
          
          <el-table :data="connections" style="width: 100%">
            <el-table-column prop="name" label="连接名称" width="200" />
            <el-table-column prop="path" label="数据库路径" />
            <el-table-column prop="created_at" label="创建时间" width="180">
              <template #default="scope">
                {{ formatDate(scope.row.created_at) }}
              </template>
            </el-table-column>
            <el-table-column prop="last_accessed" label="最后访问" width="180">
              <template #default="scope">
                {{ formatDate(scope.row.last_accessed) }}
              </template>
            </el-table-column>
            <el-table-column label="操作" width="200">
              <template #default="scope">
                <el-button 
                  type="primary" 
                  size="small" 
                  @click="connectToDatabase(scope.row)"
                  :disabled="currentConnectionId === scope.row.id"
                >
                  {{ currentConnectionId === scope.row.id ? '已连接' : '连接' }}
                </el-button>
                <el-button 
                  type="danger" 
                  size="small" 
                  @click="confirmDelete(scope.row)"
                >
                  删除
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
    
    <!-- 创建连接对话框 -->
    <el-dialog 
      v-model="showCreateDialog" 
      title="创建新连接" 
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form 
        ref="createFormRef" 
        :model="createForm" 
        :rules="createRules" 
        label-width="100px"
      >
        <el-form-item label="连接名称" prop="name">
          <el-input v-model="createForm.name" placeholder="请输入连接名称" />
        </el-form-item>
        <el-form-item label="数据库路径" prop="path">
          <el-input v-model="createForm.path" placeholder="请输入数据库路径">
            <template #append>
              <el-button @click="selectDirectory">浏览</el-button>
            </template>
          </el-input>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showCreateDialog = false">取消</el-button>
          <el-button type="primary" @click="createConnection" :loading="isLoading">
            创建
          </el-button>
        </span>
      </template>
    </el-dialog>
    
    <!-- 创建数据库对话框 -->
    <el-dialog 
      v-model="showCreateDatabaseDialog" 
      title="创建新数据库" 
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form 
        ref="createDatabaseFormRef" 
        :model="createDatabaseForm" 
        :rules="createDatabaseRules" 
        label-width="100px"
      >
        <el-form-item label="数据库名称" prop="name">
          <el-input v-model="createDatabaseForm.name" placeholder="请输入数据库名称" />
        </el-form-item>
        <el-form-item label="数据库路径" prop="path">
          <el-input v-model="createDatabaseForm.path" placeholder="请输入数据库保存路径">
            <template #append>
              <el-button @click="selectDatabaseDirectory">浏览</el-button>
            </template>
          </el-input>
          <div class="form-tip">选择一个空目录或新建一个目录作为数据库保存位置</div>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showCreateDatabaseDialog = false">取消</el-button>
          <el-button type="success" @click="createDatabase" :loading="isLoading">
            创建
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox, FormInstance, FormRules } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useSledStore } from '../stores/sled'

const sledStore = useSledStore()

const connections = ref(sledStore.connections)
const currentConnectionId = ref(sledStore.currentConnectionId)
const isLoading = ref(false)
const showCreateDialog = ref(false)
const showCreateDatabaseDialog = ref(false)
const createFormRef = ref<FormInstance>()
const createDatabaseFormRef = ref<FormInstance>()

const createForm = reactive({
  name: '',
  path: ''
})

const createDatabaseForm = reactive({
  name: '',
  path: ''
})

const createRules: FormRules = {
  name: [
    { required: true, message: '请输入连接名称', trigger: 'blur' },
    { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' }
  ],
  path: [
    { required: true, message: '请输入数据库路径', trigger: 'blur' }
  ]
}

const createDatabaseRules: FormRules = {
  name: [
    { required: true, message: '请输入数据库名称', trigger: 'blur' },
    { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' }
  ],
  path: [
    { required: true, message: '请选择数据库保存路径', trigger: 'blur' }
  ]
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString()
}

// 检查Tauri环境
const checkTauriEnvironment = () => {
  const hasWindow = typeof window !== 'undefined'
  const hasTauri = !!(window as any).__TAURI__
  const hasTauriIPC = !!(window as any).__TAURI_IPC__
  const hasTauriInternal = !!(window as any).__TAURI_INTERNALS__
  
  console.log('Connections.vue Tauri环境检测:', {
    hasWindow,
    hasTauri,
    hasTauriIPC,
    hasTauriInternal,
    windowKeys: hasWindow ? Object.keys(window).filter(key => key.includes('TAURI')) : []
  })
  
  return hasWindow && (hasTauri || hasTauriIPC || hasTauriInternal)
}

const selectDirectory = async () => {
  try {
    // 检查Tauri环境
    if (!checkTauriEnvironment()) {
      ElMessage.error('Tauri API不可用，无法选择目录')
      return
    }
    
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择数据库目录'
    })
    
    if (selected && typeof selected === 'string') {
      createForm.path = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
    ElMessage.error('选择目录失败')
  }
}

const selectDatabaseDirectory = async () => {
  try {
    // 检查Tauri环境
    if (!checkTauriEnvironment()) {
      ElMessage.error('Tauri API不可用，无法选择目录')
      return
    }
    
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择数据库保存目录'
    })
    
    if (selected && typeof selected === 'string') {
      createDatabaseForm.path = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
    ElMessage.error('选择目录失败')
  }
}

const createDatabase = async () => {
  if (!createDatabaseFormRef.value) return
  
  try {
    await createDatabaseFormRef.value.validate()
    isLoading.value = true
    
    // 检查Tauri环境
    if (!checkTauriEnvironment()) {
      ElMessage.error('Tauri API不可用，请确保在Tauri应用环境中运行')
      isLoading.value = false
      return
    }
    
    console.log('开始创建数据库...')
    const newConnection = await sledStore.createDatabase(createDatabaseForm.name, createDatabaseForm.path)
    
    if (!newConnection) {
      ElMessage.error('创建数据库失败：Tauri API不可用')
      return
    }
    
    ElMessage.success('数据库创建成功')
    showCreateDatabaseDialog.value = false
    createDatabaseForm.name = ''
    createDatabaseForm.path = ''
    
    // 自动连接到新创建的数据库
    await connectToDatabase(newConnection)
  } catch (error) {
    console.error('创建数据库失败:', error)
    ElMessage.error(`创建数据库失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

const createConnection = async () => {
  if (!createFormRef.value) return
  
  try {
    await createFormRef.value.validate()
    isLoading.value = true
    
    // 检查Tauri环境
    if (!checkTauriEnvironment()) {
      ElMessage.error('Tauri API不可用，请确保在Tauri应用环境中运行')
      isLoading.value = false
      return
    }
    
    console.log('开始创建连接...')
    const newConnection = await sledStore.createConnection(createForm.name, createForm.path)
    
    if (!newConnection) {
      ElMessage.error('创建连接失败：Tauri API不可用')
      return
    }
    
    ElMessage.success('连接创建成功')
    showCreateDialog.value = false
    createForm.name = ''
    createForm.path = ''
    
    // 自动连接到新创建的数据库
    await connectToDatabase(newConnection)
  } catch (error) {
    console.error('创建连接失败:', error)
    ElMessage.error(`创建连接失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

const connectToDatabase = async (connection: any) => {
  try {
    // 检查Tauri环境
    if (!checkTauriEnvironment()) {
      ElMessage.error('Tauri API不可用，无法连接数据库')
      return
    }
    
    sledStore.setCurrentConnection(connection.id)
    currentConnectionId.value = connection.id
    
    // 加载该连接的树和统计信息
    await sledStore.loadTrees(connection.id)
    await sledStore.loadStats(connection.id)
    
    ElMessage.success(`已连接到 ${connection.name}`)
  } catch (error) {
    console.error('连接数据库失败:', error)
    ElMessage.error('连接数据库失败')
  }
}

const confirmDelete = (connection: any) => {
  ElMessageBox.confirm(
    `确定要删除连接 "${connection.name}" 吗？此操作不可恢复。`,
    '删除确认',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    }
  ).then(async () => {
    try {
      await sledStore.removeConnection(connection.id)
      ElMessage.success('连接删除成功')
    } catch (error) {
      console.error('删除连接失败:', error)
      ElMessage.error('删除连接失败')
    }
  }).catch((error) => {
    console.error('用户取消删除:', error)
    // 用户取消删除
  })
}

onMounted(() => {
  // 检查Tauri环境
  if (!checkTauriEnvironment()) {
    console.warn('Connections.vue: Tauri API不可用，应用可能在浏览器中运行')
    ElMessage.warning('Tauri API不可用，请确保在Tauri桌面应用中运行')
  }
  
  // 连接数据现在通过pinia-plugin-persistedstate自动从localStorage恢复
  // 不再需要手动调用loadConnections()
  console.log('连接数据已从本地存储自动恢复')
})
</script>

<style scoped>
.connections-container {
  max-width: 1200px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.form-tip {
  font-size: 12px;
  color: #606266;
  margin-top: 5px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>