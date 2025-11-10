<template>
  <div class="query-container">
    <el-row :gutter="10" v-if="currentConnection">
      <el-col :span="24">
        <el-card class="query-card">
          <template #header>
            <div class="card-header">
              <span>查询工具</span>
              <el-button size="small" @click="clearResults">
                <el-icon><Delete /></el-icon>
                清空结果
              </el-button>
            </div>
          </template>
          
          <!-- 查询构建器 -->
          <div class="query-builder">
            <h3>查询构建器</h3>
            <el-form ref="queryFormRef" :model="queryForm" label-width="100px" size="default">
              <el-form-item label="树选择">
                <el-select v-model="queryForm.tree" style="width: 100%;">
                  <el-option label="默认树" value="" />
                  <el-option
                    v-for="tree in trees"
                    :key="tree"
                    :label="tree"
                    :value="tree"
                  />
                </el-select>
              </el-form-item>
              
              <el-tabs v-model="queryType" @tab-click="handleTabChange">
                <el-tab-pane label="键匹配" name="key">
                  <el-form-item label="键">
                    <el-input
                      v-model="queryForm.key"
                      placeholder="输入要查询的键"
                      @keyup.enter="runKeyQuery"
                    >
                      <template #append>
                        <el-button @click="runKeyQuery" :loading="isQuerying">
                          <el-icon><Search /></el-icon>
                        </el-button>
                      </template>
                    </el-input>
                  </el-form-item>
                </el-tab-pane>
                
                <el-tab-pane label="范围查询" name="range">
                  <el-form-item label="起始键">
                    <el-input v-model="queryForm.startKey" placeholder="输入起始键" />
                  </el-form-item>
                  <el-form-item label="结束键">
                    <el-input v-model="queryForm.endKey" placeholder="输入结束键" />
                  </el-form-item>
                  <el-form-item>
                    <el-button type="primary" @click="runRangeQuery" :loading="isQuerying">
                      <el-icon><Search /></el-icon>
                      执行范围查询
                    </el-button>
                  </el-form-item>
                </el-tab-pane>
                
                <el-tab-pane label="前缀匹配" name="prefix">
                  <el-form-item label="前缀">
                    <el-input
                      v-model="queryForm.prefix"
                      placeholder="输入键前缀"
                      @keyup.enter="runPrefixQuery"
                    >
                      <template #append>
                        <el-button @click="runPrefixQuery" :loading="isQuerying">
                          <el-icon><Search /></el-icon>
                        </el-button>
                      </template>
                    </el-input>
                  </el-form-item>
                  <el-form-item label="限制数量">
                    <el-input-number
                      v-model="queryForm.limit"
                      :min="1"
                      :max="1000"
                      style="width: 100%;"
                    />
                  </el-form-item>
                </el-tab-pane>
              </el-tabs>
            </el-form>
          </div>
          
          <el-divider />
          
          <!-- 自定义脚本 -->
          <div class="custom-script">
            <div class="script-header">
              <h3>自定义脚本</h3>
              <div class="script-actions">
                <el-button size="small" @click="loadExample">加载示例</el-button>
                <el-button size="small" type="primary" @click="runCustomScript" :loading="isQuerying">
                  <el-icon><CaretRight /></el-icon>
                  执行脚本
                </el-button>
              </div>
            </div>
            
            <div class="editor-container">
              <div ref="editorRef" style="height: 200px;"></div>
            </div>
            
            <div class="script-help">
              <el-collapse>
                <el-collapse-item title="脚本语法帮助" name="help">
                  <div class="help-content">
                    <p>脚本基于简化的 Rust 语法，支持以下操作：</p>
                    <ul>
                      <li><code>get(key)</code> - 获取键对应的值</li>
                      <li><code>set(key, value)</code> - 设置键值对</li>
                      <li><code>remove(key)</code> - 删除键值对</li>
                      <li><code>range(start, end)</code> - 范围查询</li>
                      <li><code>prefix(prefix)</code> - 前缀查询</li>
                      <li><code>filter(predicate)</code> - 过滤数据</li>
                      <li><code>map(transform)</code> - 转换数据</li>
                      <li><code>limit(count)</code> - 限制结果数量</li>
                    </ul>
                    <p>示例：</p>
                    <pre><code>// 查找所有以 "user:" 开头的键
prefix("user:")
  .filter(|kv| kv.value.contains("admin"))
  .limit(10)</code></pre>
                  </div>
                </el-collapse-item>
              </el-collapse>
            </div>
          </div>
          
          <el-divider />
          
          <!-- 查询结果 -->
          <div class="query-results" v-if="queryResults.length > 0">
            <div class="results-header">
              <h3>查询结果 ({{ queryResults.length }} 条)</h3>
              <div class="results-actions">
                <el-button size="small" @click="exportResults">导出结果</el-button>
              </div>
            </div>
            
            <el-table :data="queryResults" style="width: 100%" height="400">
              <el-table-column prop="key" label="键" width="200">
                <template #default="scope">
                  <el-text truncated>{{ formatKey(scope.row.key) }}</el-text>
                </template>
              </el-table-column>
              <el-table-column prop="value" label="值">
                <template #default="scope">
                  <el-text truncated>{{ formatValue(scope.row.value, scope.row.value_type) }}</el-text>
                </template>
              </el-table-column>
              <el-table-column prop="value_type" label="类型" width="100" />
              <el-table-column label="操作" width="150">
                <template #default="scope">
                  <el-button size="small" @click="viewItem(scope.row)">查看</el-button>
                  <el-button size="small" type="danger" @click="deleteItem(scope.row)">删除</el-button>
                </template>
              </el-table-column>
            </el-table>
          </div>
          
          <div v-else-if="hasQueried && !isQuerying" class="no-results">
            <el-empty description="未找到匹配的数据" />
          </div>
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
    
    <!-- 查看详情对话框 -->
    <el-dialog v-model="showDetailDialog" title="键值对详情" width="600px">
      <div v-if="selectedItem">
        <el-descriptions :column="1" border>
          <el-descriptions-item label="键">{{ formatKey(selectedItem.key) }}</el-descriptions-item>
          <el-descriptions-item label="类型">{{ selectedItem.value_type }}</el-descriptions-item>
          <el-descriptions-item label="值">
            <pre v-if="selectedItem.value_type === 'Json'">{{ formatValue(selectedItem.value, selectedItem.value_type) }}</pre>
            <span v-else>{{ formatValue(selectedItem.value, selectedItem.value_type) }}</span>
          </el-descriptions-item>
        </el-descriptions>
      </div>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showDetailDialog = false">关闭</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Delete, Search, CaretRight } from '@element-plus/icons-vue'
import { useSledStore } from '../stores/sled'
import * as monaco from 'monaco-editor'

const sledStore = useSledStore()

// 状态
const currentConnection = computed(() => {
  if (!sledStore.currentConnectionId) return null
  return sledStore.connections.find(c => c.id === sledStore.currentConnectionId)
})

const trees = computed(() => sledStore.trees)
const isQuerying = ref(false)
const hasQueried = ref(false)
const queryResults = ref<any[]>([])
const queryType = ref('key')
const showDetailDialog = ref(false)
const selectedItem = ref<any>(null)
const editorRef = ref<HTMLElement>()
const queryFormRef = ref()
let editor: monaco.editor.IStandaloneCodeEditor | null = null

// 查询表单
const queryForm = reactive({
  tree: '',
  key: '',
  startKey: '',
  endKey: '',
  prefix: '',
  limit: 100
})

// 方法
const formatKey = (key: number[]) => {
  try {
    return new TextDecoder().decode(new Uint8Array(key))
  } catch (e) {
    return `[${key.join(', ')}]`
  }
}

const formatValue = (value: number[], type: string) => {
  try {
    const decoded = new TextDecoder().decode(new Uint8Array(value))
    
    if (type === 'Json') {
      try {
        return JSON.stringify(JSON.parse(decoded), null, 2)
      } catch {
        return decoded
      }
    }
    
    return decoded
  } catch (e) {
    return `[${value.join(', ')}]`
  }
}

const handleTabChange = () => {
  // 清空之前的结果
  clearResults()
}

const clearResults = () => {
  queryResults.value = []
  hasQueried.value = false
}

const runKeyQuery = async () => {
  if (!currentConnection.value || !queryForm.key) {
    ElMessage.warning('请输入要查询的键')
    return
  }
  
  try {
    isQuerying.value = true
    
    // 将字符串转换为字节数组
    const keyBytes = new TextEncoder().encode(queryForm.key).reduce((acc, byte) => [...acc, byte], [] as number[])
    
    const result = await sledStore.get(
      currentConnection.value.id,
      queryForm.tree || null,
      keyBytes
    )
    
    if (result) {
      queryResults.value = [result]
    } else {
      queryResults.value = []
    }
    
    hasQueried.value = true
  } catch (error) {
    ElMessage.error('查询失败')
  } finally {
    isQuerying.value = false
  }
}

const runRangeQuery = async () => {
  if (!currentConnection.value || !queryForm.startKey || !queryForm.endKey) {
    ElMessage.warning('请输入起始键和结束键')
    return
  }
  
  try {
    isQuerying.value = true
    
    // 将字符串转换为字节数组
    const startKeyBytes = new TextEncoder().encode(queryForm.startKey).reduce((acc, byte) => [...acc, byte], [] as number[])
    const endKeyBytes = new TextEncoder().encode(queryForm.endKey).reduce((acc, byte) => [...acc, byte], [] as number[])
    
    const result = await sledStore.rangeQuery(
      currentConnection.value.id,
      queryForm.tree || null,
      {
        start_key: startKeyBytes,
        end_key: endKeyBytes,
        limit: queryForm.limit
      }
    )
    
    queryResults.value = (result as any).entries
    hasQueried.value = true
  } catch (error) {
    ElMessage.error('查询失败')
  } finally {
    isQuerying.value = false
  }
}

const runPrefixQuery = async () => {
  if (!currentConnection.value || !queryForm.prefix) {
    ElMessage.warning('请输入前缀')
    return
  }
  
  try {
    isQuerying.value = true
    
    // 将字符串转换为字节数组
    const prefixBytes = new TextEncoder().encode(queryForm.prefix).reduce((acc, byte) => [...acc, byte], [] as number[])
    
    const result = await sledStore.prefixQuery(
      currentConnection.value.id,
      queryForm.tree || null,
      {
        prefix: prefixBytes,
        limit: queryForm.limit
      }
    )
    
    queryResults.value = (result as any).entries
    hasQueried.value = true
  } catch (error) {
    ElMessage.error('查询失败')
  } finally {
    isQuerying.value = false
  }
}

const runCustomScript = async () => {
  if (!currentConnection.value || !editor) {
    ElMessage.warning('请输入脚本')
    return
  }
  
  try {
    isQuerying.value = true
    
    // const script = editor.getValue()
    
    // 这里应该实现自定义脚本的解析和执行
    // 由于这是一个复杂的功能，这里只是一个模拟实现
    ElMessage.info('自定义脚本执行功能正在开发中，请使用查询构建器')
    
    // 模拟执行结果
    // const result = await sledStore.executeScript(currentConnection.value.id, script)
    // queryResults.value = result.entries
    // hasQueried.value = true
  } catch (error) {
    ElMessage.error('脚本执行失败')
  } finally {
    isQuerying.value = false
  }
}

const loadExample = () => {
  if (!editor) return
  
  const exampleScript = `// 查找所有以 "user:" 开头的键
prefix("user:")
  .filter(|kv| kv.value.contains("admin"))
  .limit(10)`
  
  editor.setValue(exampleScript)
}

const viewItem = (item: any) => {
  selectedItem.value = item
  showDetailDialog.value = true
}

const deleteItem = async (item: any) => {
  if (!currentConnection.value) return
  
  try {
    await ElMessageBox.confirm(
      `确定要删除键 "${formatKey(item.key)}" 吗？`,
      '删除确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    await sledStore.remove(
      currentConnection.value.id,
      queryForm.tree || null,
      item.key
    )
    
    ElMessage.success('删除成功')
    
    // 从结果中移除已删除的项
    const index = queryResults.value.findIndex(r => 
      JSON.stringify(r.key) === JSON.stringify(item.key)
    )
    if (index !== -1) {
      queryResults.value.splice(index, 1)
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const exportResults = () => {
  if (queryResults.value.length === 0) return
  
  try {
    // 转换数据格式
    const jsonData = queryResults.value.map(item => ({
      key: formatKey(item.key),
      value: formatValue(item.value, item.value_type),
      value_type: item.value_type
    }))
    
    // 创建下载链接
    const blob = new Blob([JSON.stringify(jsonData, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = `query_results_${new Date().toISOString().slice(0, 10)}.json`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
    
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败')
  }
}

const initEditor = () => {
  if (!editorRef.value) return
  
  editor = monaco.editor.create(editorRef.value, {
    value: '// 在此输入自定义查询脚本',
    language: 'rust',
    theme: 'vs-dark',
    minimap: { enabled: false },
    scrollBeyondLastLine: false,
    automaticLayout: true
  })
}

// 初始化
onMounted(async () => {
  await nextTick()
  initEditor()
  
  if (currentConnection.value) {
    await sledStore.loadTrees(currentConnection.value.id)
  }
})
</script>

<style scoped>
.query-container {
  height: 100%;
}

.query-card {
  min-height: 600px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.query-builder {
  margin-bottom: 20px;
}

.custom-script {
  margin-bottom: 20px;
}

.script-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.script-actions {
  display: flex;
  gap: 10px;
}

.editor-container {
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  overflow: hidden;
}

.script-help {
  margin-top: 10px;
}

.help-content {
  font-size: 14px;
}

.help-content pre {
  background-color: var(--el-bg-color-page);
  padding: 10px;
  border-radius: 4px;
  overflow-x: auto;
}

.help-content code {
  font-family: monospace;
  background-color: var(--el-bg-color-page);
  padding: 2px 4px;
  border-radius: 3px;
}

.query-results {
  margin-top: 20px;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.results-actions {
  display: flex;
  gap: 10px;
}

.no-results {
  margin-top: 20px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
}
</style>