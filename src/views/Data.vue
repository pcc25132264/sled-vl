<template>
  <div class="data-container">
    <el-row :gutter="10">
      <!-- 左侧数据库导航 -->
      <el-col :span="6">
        <el-card class="tree-card" style="height: 100%; display: flex; flex-direction: column;">
          <template #header>
            <div style="display: flex; align-items: center; justify-content: space-between;">
              <span style="font-weight: 600;">数据库导航</span>
              <div style="display: flex; gap: 4px;">
                <el-button size="small" type="primary" @click="showCreateTreeDialog = true" v-if="currentConnection">
                  <el-icon><Plus /></el-icon>
                </el-button>
                <el-button size="small" @click="refreshTree" v-if="currentConnection">
                  <el-icon><Refresh /></el-icon>
                </el-button>
              </div>
            </div>
          </template>
          
          <!-- 无连接提示 -->
          <div v-if="!currentConnection" style="text-align: center; color: var(--el-text-color-secondary); padding: 40px 0;">
            <el-icon size="48" style="margin-bottom: 10px;"><Connection /></el-icon>
            <div>请先连接到数据库</div>
            <el-button size="small" type="primary" @click="$router.push('/connections')" style="margin-top: 10px;">
              去连接管理
            </el-button>
          </div>
          
          <!-- 有连接时的内容 -->
          <div v-else>
            <!-- 树搜索 -->
            <div style="margin-bottom: 12px;">
              <el-input
                v-model="treeSearch"
                placeholder="搜索表名"
                size="small"
                clearable
                @input="handleTreeSearch"
              >
                <template #prefix>
                  <el-icon><Search /></el-icon>
                </template>
              </el-input>
            </div>
            
            <!-- 树形导航 -->
            <div style="flex: 1; overflow-y: auto;">
              <el-tree
                ref="treeRef"
                :data="filteredTreeData"
                :props="defaultProps"
                node-key="id"
                :default-expanded-keys="expandedKeys"
                :filter-node-method="filterNode"
                @node-click="handleNodeClick"
                highlight-current
                :expand-on-click-node="false"
              >
                <template #default="{ node, data }">
                  <span class="custom-tree-node" style="display: flex; align-items: center; justify-content: space-between; width: 100%;">
                    <div style="display: flex; align-items: center; flex: 1;">
                      <el-icon style="margin-right: 6px;">
                        <Folder v-if="data.type === 'database'" />
                        <Document v-else />
                      </el-icon>
                      <span style="flex: 1; overflow: hidden; text-overflow: ellipsis;">{{ node.label }}</span>
                      <span v-if="data.type === 'tree'" class="tree-count" style="font-size: 12px; color: var(--el-text-color-secondary); margin-left: 8px;">
                        {{ data.count }}
                      </span>
                    </div>
                    
                    <div v-if="data.type === 'tree'" class="tree-actions" style="display: flex; gap: 2px; margin-left: 8px;">
                      <el-button size="small" type="text" @click.stop="editTreeName(data)" title="编辑表名">
                        <el-icon><Edit /></el-icon>
                      </el-button>
                      <el-button size="small" type="text" @click.stop="deleteTree(data)" title="删除表" style="color: var(--el-color-danger);">
                        <el-icon><Delete /></el-icon>
                      </el-button>
                    </div>
                  </span>
                </template>
              </el-tree>
            </div>
            
            <!-- 无数据提示 -->
            <div v-if="!treeData || treeData.length === 0" style="text-align: center; color: var(--el-text-color-secondary); padding: 40px 0;">
              <el-icon size="48" style="margin-bottom: 10px;"><Folder /></el-icon>
              <div>暂无数据库表</div>
              <el-button size="small" type="primary" @click="showCreateTreeDialog = true" style="margin-top: 10px;">
                创建新表
              </el-button>
            </div>
          </div>
        </el-card>
        
        <!-- 数据库统计信息 -->
        <el-card class="stats-card" v-if="stats" style="margin-top: 16px;">
          <template #header>
            <span style="font-weight: 600;">数据库统计</span>
          </template>
          <el-descriptions :column="1" size="small">
            <el-descriptions-item label="磁盘占用">
              <el-text type="primary">{{ formatSize(stats.size_on_disk) }}</el-text>
            </el-descriptions-item>
            <el-descriptions-item label="键值对数量">
              <el-text type="primary">{{ stats.key_count }}</el-text>
            </el-descriptions-item>
            <el-descriptions-item label="表数量">
              <el-text type="primary">{{ stats.tree_count }}</el-text>
            </el-descriptions-item>
          </el-descriptions>
        </el-card>
      </el-col>
      
      <!-- 中间数据展示区 -->
      <el-col :span="10">
        <el-card class="data-card">
          <template #header>
            <div class="card-header">
              <span>{{ currentTreeName }} 数据</span>
              <div class="header-actions">
                <el-select v-model="viewMode" size="small" style="width: 100px; margin-right: 10px;" v-if="currentConnection">
                  <el-option label="表格" value="table" />
                  <el-option label="列表" value="list" />
                  <el-option label="JSON" value="json" />
                </el-select>
                <el-button size="small" @click="refreshData" v-if="currentConnection">
                  <el-icon><Refresh /></el-icon>
                </el-button>
              </div>
            </div>
          </template>
          
          <!-- 无连接提示 -->
          <div v-if="!currentConnection" style="text-align: center; color: var(--el-text-color-secondary); padding: 60px 0;">
            <el-icon size="64" style="margin-bottom: 16px;"><Connection /></el-icon>
            <div style="font-size: 16px; margin-bottom: 8px;">请先连接到数据库</div>
            <div style="font-size: 14px; margin-bottom: 20px;">连接数据库后即可查看和操作数据</div>
            <el-button type="primary" @click="$router.push('/connections')">
              去连接管理
            </el-button>
          </div>
          
          <!-- 有连接时的内容 -->
          <div v-else>
            <!-- 过滤器 -->
            <div class="filter-container">
              <el-input
                v-model="filterKey"
                placeholder="键过滤"
                size="small"
                style="width: 200px; margin-right: 10px;"
                @input="handleFilterChange"
              >
                <template #prefix>
                  <el-icon><Search /></el-icon>
                </template>
              </el-input>
              
              <el-select v-model="valueTypeFilter" size="small" style="width: 120px; margin-right: 10px;" @change="handleFilterChange">
                <el-option label="所有类型" value="" />
                <el-option label="字符串" value="String" />
                <el-option label="数字" value="Number" />
                <el-option label="布尔值" value="Boolean" />
                <el-option label="JSON" value="Json" />
                <el-option label="二进制" value="Binary" />
              </el-select>
              
              <el-input-number
                v-model="pageSize"
                :min="10"
                :max="1000"
                :step="10"
                size="small"
                style="width: 120px; margin-right: 10px;"
                @change="handlePageSizeChange"
              />
              <span>条/页</span>
            </div>
            
            <!-- 表格视图 -->
            <div v-if="viewMode === 'table'" class="table-container">
              <el-table
                :data="paginatedData"
                :default-sort="{ prop: 'key', order: 'ascending' }"
                style="width: 100%"
                :row-class-name="getRowClassName"
                @row-click="handleRowClick"
                highlight-current-row
                size="small"
              >
                <el-table-column prop="key" label="键" width="200" sortable :sort-method="(a, b) => formatKey(a.key).localeCompare(formatKey(b.key))">
                  <template #default="{ row }">
                    <div style="display: flex; align-items: center; justify-content: space-between;">
                      <el-text style="font-family: 'Courier New', monospace;">{{ formatKey(row.key) }}</el-text>
                      <el-button size="small" type="text" @click.stop="copyKey(row.key)" style="margin-left: 8px;">
                        <el-icon><CopyDocument /></el-icon>
                      </el-button>
                    </div>
                  </template>
                </el-table-column>
                
                <el-table-column prop="value_type" label="类型" width="100" sortable :filters="[
                  { text: '字符串', value: 'String' },
                  { text: '数字', value: 'Number' },
                  { text: '布尔值', value: 'Boolean' },
                  { text: 'JSON', value: 'Json' },
                  { text: '二进制', value: 'Binary' }
                ]" :filter-method="filterType">
                  <template #default="{ row }">
                    <el-tag :type="getTypeTagType(row.value_type)" size="small">{{ row.value_type }}</el-tag>
                  </template>
                </el-table-column>
                
                <el-table-column prop="value" label="值" min-width="300" sortable :sort-method="(a, b) => formatValue(a.value, a.value_type).localeCompare(formatValue(b.value, b.value_type))">
                  <template #default="{ row }">
                    <div style="display: flex; align-items: center; justify-content: space-between;">
                      <el-text 
                        style="font-family: 'Courier New', monospace; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 250px;"
                        :title="formatValue(row.value, row.value_type)"
                      >
                        {{ formatValue(row.value, row.value_type) }}
                      </el-text>
                      <el-button size="small" type="text" @click.stop="copyValue(row)" style="margin-left: 8px;">
                        <el-icon><CopyDocument /></el-icon>
                      </el-button>
                    </div>
                  </template>
                </el-table-column>
                
                <el-table-column label="操作" width="120" fixed="right">
                  <template #default="{ row }">
                    <div style="display: flex; gap: 4px;">
                      <el-button size="small" type="primary" @click.stop="quickEdit(row)">
                        <el-icon><Edit /></el-icon>
                      </el-button>
                      <el-button size="small" type="danger" @click.stop="deleteItem(row)">
                        <el-icon><Delete /></el-icon>
                      </el-button>
                    </div>
                  </template>
                </el-table-column>
              </el-table>
            </div>
          
          <!-- 数据列表视图 -->
          <div v-else-if="viewMode === 'list'" class="list-view" style="height: 500px; overflow-y: auto;">
            <div
              v-for="item in paginatedData"
              :key="item.key"
              class="list-item"
              @click="handleRowClick(item)"
            >
              <div class="list-item-header">
                <span class="list-item-key">{{ formatKey(item.key) }}</span>
                <el-tag size="small" :type="getTypeTagType(item.value_type)">{{ item.value_type }}</el-tag>
              </div>
              <div class="list-item-value">{{ formatValue(item.value, item.value_type) }}</div>
            </div>
          </div>
          
          <!-- JSON视图 -->
          <div v-else-if="viewMode === 'json'" class="json-view" style="height: 500px; overflow-y: auto;">
            <pre>{{ JSON.stringify(paginatedData, null, 2) }}</pre>
          </div>
          
          <!-- 分页 -->
          <div class="pagination-container">
            <el-pagination
              v-model:current-page="currentPage"
              :page-size="pageSize"
              :total="filteredData.length"
              layout="prev, pager, next, jumper, total"
              @current-change="handlePageChange"
            />
          </div>
          </div>
        </el-card>
      </el-col>
      
      <!-- 右侧操作面板 -->
      <el-col :span="8">
        <el-card class="action-card" style="height: 100%; display: flex; flex-direction: column;">
          <template #header>
            <div style="display: flex; align-items: center; justify-content: space-between;">
              <span style="font-weight: 600;">操作面板</span>
              <el-button size="small" type="text" @click="refreshData" :loading="isLoading" v-if="currentConnection">
                <el-icon><Refresh /></el-icon>
                刷新
              </el-button>
            </div>
          </template>
          
          <div style="flex: 1; overflow-y: auto; padding: 0 5px;">
            <!-- 无连接提示 -->
            <div v-if="!currentConnection" class="no-connection-prompt" style="text-align: center; color: var(--el-text-color-secondary); padding: 40px 0;">
              <el-icon size="48" style="margin-bottom: 10px;"><Connection /></el-icon>
              <div style="margin-bottom: 15px;">请先连接到数据库</div>
              <el-button type="primary" @click="$router.push('/connections')">去连接管理</el-button>
            </div>
            
            <!-- 有连接时的操作内容 -->
            <div v-else>
              <!-- 快速操作 -->
              <div class="quick-actions" style="margin-bottom: 20px;">
                <h4 style="margin: 0 0 10px 0; color: var(--el-text-color-primary); font-size: 14px;">
                  <el-icon><Plus /></el-icon>
                  快速添加
                </h4>
                <el-form ref="addItemFormRef" :model="newItem" size="small" style="margin-bottom: 0;">
                  <el-form-item label="键" style="margin-bottom: 12px;">
                    <el-input v-model="newItem.key" placeholder="输入键名" clearable />
                  </el-form-item>
                  <el-form-item label="类型" style="margin-bottom: 12px;">
                    <el-select v-model="newItem.valueType" style="width: 100%;">
                      <el-option label="字符串" value="String" />
                      <el-option label="数字" value="Number" />
                      <el-option label="布尔值" value="Boolean" />
                      <el-option label="JSON" value="Json" />
                      <el-option label="二进制" value="Binary" />
                    </el-select>
                  </el-form-item>
                  <el-form-item label="值" style="margin-bottom: 12px;">
                    <el-input
                      v-model="newItem.value"
                      type="textarea"
                      :rows="3"
                      placeholder="输入值"
                      resize="none"
                    />
                  </el-form-item>
                  <el-form-item style="margin-bottom: 0;">
                    <el-button type="primary" @click="addItem" :loading="isAdding" size="small" style="width: 100%;">
                      <el-icon><Plus /></el-icon>
                      添加键值对
                    </el-button>
                  </el-form-item>
                </el-form>
              </div>
              
              <el-divider style="margin: 15px 0;">
                <el-icon><Document /></el-icon>
              </el-divider>
              
              <!-- 批量操作 -->
              <div class="batch-operations" style="margin-bottom: 20px;">
                <h4 style="margin: 0 0 10px 0; color: var(--el-text-color-primary); font-size: 14px;">
                  <el-icon><UploadFilled /></el-icon>
                  批量操作
                </h4>
                <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                  <el-button size="small" @click="showImportDialog = true" style="flex: 1;">
                    <el-icon><UploadFilled /></el-icon>
                    导入
                  </el-button>
                  <el-button size="small" @click="exportData" style="flex: 1;">
                    <el-icon><Download /></el-icon>
                    导出
                  </el-button>
                </div>
              </div>
              
              <el-divider style="margin: 15px 0;" v-if="selectedItem">
                <el-icon><Edit /></el-icon>
              </el-divider>
              
              <!-- 选中项详情 -->
              <div v-if="selectedItem" class="selected-item">
                <h4 style="margin: 0 0 15px 0; color: var(--el-text-color-primary); font-size: 14px;">
                  <el-icon><Document /></el-icon>
                  选中项详情
                </h4>
                
                <div class="item-details" style="background: var(--el-fill-color-light); padding: 12px; border-radius: 4px; margin-bottom: 15px;">
                  <div style="display: flex; align-items: center; margin-bottom: 8px;">
                    <span style="font-weight: 500; min-width: 40px;">键:</span>
                    <el-text copyable style="flex: 1; font-family: 'Courier New', monospace;">
                      {{ formatKey(selectedItem.key) }}
                    </el-text>
                  </div>
                  <div style="display: flex; align-items: center; margin-bottom: 8px;">
                    <span style="font-weight: 500; min-width: 40px;">类型:</span>
                    <el-tag :type="getTypeTagType(selectedItem.value_type)" size="small">
                      {{ selectedItem.value_type }}
                    </el-tag>
                  </div>
                  <div style="display: flex; align-items: flex-start;">
                    <span style="font-weight: 500; min-width: 40px; margin-top: 2px;">值:</span>
                    <div style="flex: 1;">
                      <el-input
                        v-model="editableValue"
                        type="textarea"
                        :rows="4"
                        v-if="isEditing"
                        style="font-family: 'Courier New', monospace;"
                        resize="none"
                      />
                      <div v-else class="value-display" style="background: white; padding: 8px; border-radius: 4px; border: 1px solid var(--el-border-color);">
                        <el-text style="font-family: 'Courier New', monospace; white-space: pre-wrap; word-break: break-all; display: block;">
                          {{ formatValue(selectedItem.value, selectedItem.value_type) }}
                        </el-text>
                      </div>
                    </div>
                  </div>
                </div>
                
                <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                  <el-button v-if="!isEditing" size="small" @click="isEditing = true" style="flex: 1;">
                    <el-icon><Edit /></el-icon>
                    编辑
                  </el-button>
                  <el-button v-if="isEditing" size="small" type="primary" @click="saveEdit" style="flex: 1;">
                    <el-icon><Check /></el-icon>
                    保存
                  </el-button>
                  <el-button v-if="isEditing" size="small" @click="isEditing = false" style="flex: 1;">
                    <el-icon><Close /></el-icon>
                    取消
                  </el-button>
                  <el-button size="small" type="danger" @click="deleteSelectedItem" style="flex: 1;">
                    <el-icon><Delete /></el-icon>
                    删除
                  </el-button>
                </div>
                
                <div style="display: flex; gap: 8px; margin-top: 10px;">
                  <el-button size="small" type="text" @click="copySelectedValue" style="flex: 1;">
                    <el-icon><CopyDocument /></el-icon>
                    复制值
                  </el-button>
                  <el-button size="small" type="text" @click="copySelectedItem" style="flex: 1;">
                    <el-icon><CopyDocument /></el-icon>
                    复制全部
                  </el-button>
                  <el-button size="small" type="text" @click="showShortcutHelp" style="flex: 1;">
                    <el-icon><QuestionFilled /></el-icon>
                    快捷键
                  </el-button>
                </div>
              </div>
              
              <!-- 无选中项提示 -->
              <div v-else class="no-selection" style="text-align: center; color: var(--el-text-color-secondary); padding: 40px 0;">
                <el-icon size="48" style="margin-bottom: 10px;"><Document /></el-icon>
                <div>请选择数据项查看详情</div>
              </div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
    
    <!-- 无连接提示 -->
    <el-row v-if="!currentConnection">
      <el-col :span="24">
        <el-empty description="请先连接到数据库">
          <el-button type="primary" @click="$router.push('/connections')">创建连接</el-button>
        </el-empty>
      </el-col>
    </el-row>
    
    <!-- 创建表对话框 -->
    <el-dialog v-model="showCreateTreeDialog" title="新建表" width="400px">
      <el-form ref="createTreeFormRef" :model="newTreeForm" label-width="80px" size="small">
        <el-form-item label="表名" prop="name">
          <el-input v-model="newTreeForm.name" placeholder="请输入表名" />
        </el-form-item>
      </el-form>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showCreateTreeDialog = false">取消</el-button>
          <el-button type="primary" @click="createTree">创建</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 编辑表名对话框 -->
    <el-dialog v-model="showEditTreeDialog" title="编辑表名" width="400px">
      <el-form ref="editTreeFormRef" :model="editTreeForm" label-width="80px" size="small">
        <el-form-item label="表名" prop="name">
          <el-input v-model="editTreeForm.name" placeholder="请输入表名" />
        </el-form-item>
      </el-form>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showEditTreeDialog = false">取消</el-button>
          <el-button type="primary" @click="updateTreeName">保存</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- 导入数据对话框 -->
    <el-dialog v-model="showImportDialog" title="导入数据" width="500px">
      <el-upload
        class="upload-demo"
        drag
        :auto-upload="false"
        :on-change="handleFileChange"
        :limit="1"
      >
        <el-icon class="el-icon--upload"><upload-filled /></el-icon>
        <div class="el-upload__text">
          将文件拖到此处，或<em>点击上传</em>
        </div>
        <template #tip>
          <div class="el-upload__tip">
            支持 JSON/CSV 格式文件
          </div>
        </template>
      </el-upload>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showImportDialog = false">取消</el-button>
          <el-button type="primary" @click="importData" :disabled="!importFile">
            导入
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch, onUnmounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Refresh, Search, Folder, Document, UploadFilled, Plus, Edit, Delete, CopyDocument, Download, Check, Close, QuestionFilled, Connection } from '@element-plus/icons-vue'
import { useSledStore } from '../stores/sled'

const sledStore = useSledStore()

// 状态
const currentConnection = computed(() => {
  if (!sledStore.currentConnectionId) return null
  return sledStore.connections.find(c => c.id === sledStore.currentConnectionId)
})

const currentTreeName = ref('default')
const viewMode = ref('table')
const filterKey = ref('')
const valueTypeFilter = ref('')
const pageSize = ref(50)
const currentPage = ref(1)
const data = ref<any[]>([])
const isLoading = ref(false)
const isAdding = ref(false)
const showImportDialog = ref(false)
const importFile = ref<File | null>(null)

// 树搜索相关状态
const treeSearch = ref('')
const filteredTreeData = ref([])

// 表管理相关状态
const showCreateTreeDialog = ref(false)
const showEditTreeDialog = ref(false)
const newTreeForm = reactive({ name: '' })
const editTreeForm = reactive({ name: '', originalName: '' })
const addItemFormRef = ref()
const createTreeFormRef = ref()
const editTreeFormRef = ref()

// 树形数据
const treeData = computed(() => {
  if (!currentConnection.value) return []
  
  const trees = sledStore.trees
  const result = [
    {
      id: currentConnection.value.id,
      label: currentConnection.value.name,
      type: 'database',
      children: trees.map(tree => ({
        id: `${currentConnection.value?.id}-${tree}`,
        label: tree,
        type: 'tree',
        count: 0 // 这里可以添加获取树中键值对数量的逻辑
      }))
    }
  ]
  
  return result
})

// 树搜索处理
const handleTreeSearch = () => {
  if (treeSearch.value) {
    filteredTreeData.value = filterTreeData(treeData.value, treeSearch.value)
  } else {
    filteredTreeData.value = treeData.value
  }
}

// 过滤树数据
const filterTreeData = (data: any[], searchText: string) => {
  return data.map(item => {
    if (item.children) {
      const filteredChildren = filterTreeData(item.children, searchText)
      if (filteredChildren.length > 0 || item.label.toLowerCase().includes(searchText.toLowerCase())) {
        return {
          ...item,
          children: filteredChildren
        }
      }
      return null
    } else {
      if (item.label.toLowerCase().includes(searchText.toLowerCase())) {
        return item
      }
      return null
    }
  }).filter(Boolean)
}

// 树节点过滤方法
const filterNode = (value: string, data: any) => {
  if (!value) return true
  return data.label.toLowerCase().includes(value.toLowerCase())
}

// 刷新树
const refreshTree = async () => {
  await refreshTrees()
  ElMessage.success('树结构已刷新')
}

const expandedKeys = ref<string[]>([])
const defaultProps = {
  children: 'children',
  label: 'label'
}

// 过滤后的数据
const filteredData = computed(() => {
  let result = data.value
  
  // 键过滤
  if (filterKey.value) {
    result = result.filter(item => 
      formatKey(item.key).toLowerCase().includes(filterKey.value.toLowerCase())
    )
  }
  
  // 值类型过滤
  if (valueTypeFilter.value) {
    result = result.filter(item => item.value_type === valueTypeFilter.value)
  }
  
  return result
})

// 分页后的数据
const paginatedData = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredData.value.slice(start, end)
})

// 选中项
const selectedItem = ref<any>(null)
const isEditing = ref(false)
const editableValue = ref('')

// 新建项
const newItem = reactive({
  key: '',
  value: '',
  valueType: 'String'
})

// 统计信息
const stats = computed(() => sledStore.stats)

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

const formatSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const getTypeTagType = (type: string) => {
  const typeMap: Record<string, string> = {
    'String': '',
    'Number': 'success',
    'Boolean': 'warning',
    'Json': 'info',
    'Binary': 'danger'
  }
  return typeMap[type] || ''
}

// 获取行样式
const getRowClassName = ({ row }: { row: any }) => {
  return selectedItem.value && selectedItem.value.key === row.key ? 'selected-row' : ''
}

// 复制键
const copyKey = async (key: number[]) => {
  try {
    const keyText = formatKey(key)
    await navigator.clipboard.writeText(keyText)
    ElMessage.success('键已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

// 复制值
const copyValue = async (item: any) => {
  try {
    const value = formatValue(item.value, item.value_type)
    await navigator.clipboard.writeText(value)
    ElMessage.success('值已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

// 复制选中项的值
const copySelectedValue = async () => {
  if (!selectedItem.value) return
  
  try {
    const value = formatValue(selectedItem.value.value, selectedItem.value.value_type)
    await navigator.clipboard.writeText(value)
    ElMessage.success('值已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

// 快速编辑
const quickEdit = (row: any) => {
  selectedItem.value = row
  editableValue.value = formatValue(row.value, row.value_type)
  isEditing.value = true
  // 滚动到操作面板
  setTimeout(() => {
    const actionCard = document.querySelector('.action-card')
    if (actionCard) {
      actionCard.scrollIntoView({ behavior: 'smooth', block: 'start' })
    }
  }, 100)
}

// 类型过滤方法
const filterType = (value: string, row: any) => {
  return row.value_type === value
}

// 复制选中项全部内容
const copySelectedItem = async () => {
  if (!selectedItem.value) return
  
  try {
    const content = `键: ${formatKey(selectedItem.value.key)}\n类型: ${selectedItem.value.value_type}\n值: ${formatValue(selectedItem.value.value, selectedItem.value.value_type)}`
    await navigator.clipboard.writeText(content)
    ElMessage.success('已复制到剪贴板')
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

const refreshTrees = async () => {
  if (!currentConnection.value) return
  
  try {
    await sledStore.loadTrees(currentConnection.value.id)
    await sledStore.loadStats(currentConnection.value.id)
  } catch (error) {
    ElMessage.error('刷新树结构失败')
  }
}

// 创建表
const createTree = async () => {
  if (!currentConnection.value || !newTreeForm.name.trim()) {
    ElMessage.warning('请输入表名')
    return
  }
  
  try {
    await sledStore.createTree(currentConnection.value.id, newTreeForm.name.trim())
    ElMessage.success('表创建成功')
    showCreateTreeDialog.value = false
    newTreeForm.name = ''
  } catch (error) {
    ElMessage.error(`创建表失败: ${error}`)
  }
}

// 编辑表名
const editTreeName = (data: any) => {
  editTreeForm.name = data.label
  editTreeForm.originalName = data.label
  showEditTreeDialog.value = true
}

// 更新表名（实际上是删除旧表并创建新表，因为sled不支持直接重命名）
const updateTreeName = async () => {
  if (!currentConnection.value || !editTreeForm.name.trim()) {
    ElMessage.warning('请输入表名')
    return
  }
  
  if (editTreeForm.name.trim() === editTreeForm.originalName) {
    showEditTreeDialog.value = false
    return
  }
  
  try {
    // 在sled中，重命名表需要先导出数据，删除旧表，创建新表，再导入数据
    // 这里简化处理，只创建新表并提示用户
    await sledStore.createTree(currentConnection.value.id, editTreeForm.name.trim())
    ElMessage.success(`表已创建，注意：由于sled限制，您需要手动将数据从 ${editTreeForm.originalName} 迁移到 ${editTreeForm.name}`)
    showEditTreeDialog.value = false
  } catch (error) {
    ElMessage.error(`更新表名失败: ${error}`)
  }
}

// 删除表
const deleteTree = async (data: any) => {
  if (!currentConnection.value || data.label === 'default') {
    ElMessage.warning('不能删除默认表')
    return
  }
  
  try {
    await ElMessageBox.confirm(
      `确定要删除表 "${data.label}" 吗？此操作将删除该表中的所有数据，且无法恢复。`,
      '删除确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    await sledStore.removeTree(currentConnection.value.id, data.label)
    ElMessage.success('表删除成功')
    
    // 如果当前正在查看被删除的表，切换到默认表
    if (currentTreeName.value === data.label) {
      currentTreeName.value = 'default'
      await loadData()
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除表失败')
    }
  }
}

const handleNodeClick = async (data: any) => {
  if (data.type === 'tree') {
    currentTreeName.value = data.label
    await loadData()
  }
}

const loadData = async () => {
  if (!currentConnection.value) return
  
  try {
    isLoading.value = true
    
    // 使用前缀查询获取所有数据
    const result = await sledStore.prefixQuery(
      currentConnection.value.id,
      currentTreeName.value === 'default' ? null : currentTreeName.value,
      { prefix: [], limit: 1000 }
    )
    
    data.value = result.entries
  } catch (error) {
    ElMessage.error('加载数据失败')
  } finally {
    isLoading.value = false
  }
}

const refreshData = async () => {
  await loadData()
}

const handleFilterChange = () => {
  currentPage.value = 1
}

const handlePageSizeChange = () => {
  currentPage.value = 1
}

const handlePageChange = () => {
  // 页面变化时不需要额外操作
}

const handleRowClick = (row: any) => {
  selectedItem.value = row
  editableValue.value = formatValue(row.value, row.value_type)
  isEditing.value = false
}

const addItem = async () => {
  if (!currentConnection.value || !newItem.key || !newItem.value) {
    ElMessage.warning('请填写键和值')
    return
  }
  
  try {
    isAdding.value = true
    
    // 将字符串转换为字节数组
    const keyBytes = new TextEncoder().encode(newItem.key).reduce((acc, byte) => [...acc, byte], [] as number[])
    let valueBytes: number[]
    
    // 根据类型处理值
    switch (newItem.valueType) {
      case 'Number':
        const numValue = parseFloat(newItem.value)
        if (isNaN(numValue)) {
          throw new Error('无效的数字格式')
        }
        valueBytes = new TextEncoder().encode(numValue.toString()).reduce((acc, byte) => [...acc, byte], [] as number[])
        break
      case 'Boolean':
        const boolValue = newItem.value.toLowerCase() === 'true'
        valueBytes = new TextEncoder().encode(boolValue.toString()).reduce((acc, byte) => [...acc, byte], [] as number[])
        break
      case 'Json':
        try {
          JSON.parse(newItem.value)
        } catch {
          throw new Error('无效的JSON格式')
        }
        valueBytes = new TextEncoder().encode(newItem.value).reduce((acc, byte) => [...acc, byte], [] as number[])
        break
      case 'Binary':
        // 这里可以添加二进制数据的处理逻辑
        valueBytes = new TextEncoder().encode(newItem.value).reduce((acc, byte) => [...acc, byte], [] as number[])
        break
      default:
        valueBytes = new TextEncoder().encode(newItem.value).reduce((acc, byte) => [...acc, byte], [] as number[])
    }
    
    await sledStore.set(
      currentConnection.value.id,
      currentTreeName.value === 'default' ? null : currentTreeName.value,
      keyBytes,
      valueBytes
    )
    
    ElMessage.success('添加成功')
    resetNewItem()
    await loadData()
  } catch (error) {
    ElMessage.error(`添加失败: ${error}`)
  } finally {
    isAdding.value = false
  }
}

const resetNewItem = () => {
  newItem.key = ''
  newItem.value = ''
  newItem.valueType = 'String'
}

const editItem = (row: any) => {
  selectedItem.value = row
  editableValue.value = formatValue(row.value, row.value_type)
  isEditing.value = true
}

const saveEdit = async () => {
  if (!selectedItem.value || !currentConnection.value) return
  
  try {
    // 将字符串转换为字节数组
    let valueBytes: number[]
    
    // 根据类型处理值
    switch (selectedItem.value.value_type) {
      case 'Number':
        const numValue = parseFloat(editableValue.value)
        if (isNaN(numValue)) {
          throw new Error('无效的数字格式')
        }
        valueBytes = new TextEncoder().encode(numValue.toString()).reduce((acc, byte) => [...acc, byte], [] as number[])
        break
      case 'Boolean':
        const boolValue = editableValue.value.toLowerCase() === 'true'
        valueBytes = new TextEncoder().encode(boolValue.toString()).reduce((acc, byte) => [...acc, byte], [] as number[])
        break
      case 'Json':
        try {
          JSON.parse(editableValue.value)
        } catch {
          throw new Error('无效的JSON格式')
        }
        valueBytes = new TextEncoder().encode(editableValue.value).reduce((acc, byte) => [...acc, byte], [] as number[])
        break
      default:
        valueBytes = new TextEncoder().encode(editableValue.value).reduce((acc, byte) => [...acc, byte], [] as number[])
    }
    
    await sledStore.set(
      currentConnection.value.id,
      currentTreeName.value === 'default' ? null : currentTreeName.value,
      selectedItem.value.key,
      valueBytes
    )
    
    ElMessage.success('保存成功')
    isEditing.value = false
    await loadData()
  } catch (error) {
    ElMessage.error(`保存失败: ${error}`)
  }
}

const deleteItem = async (row: any) => {
  if (!currentConnection.value) return
  
  try {
    await ElMessageBox.confirm(
      `确定要删除键为 "${formatKey(row.key)}" 的数据项吗？`,
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    await sledStore.remove(
      currentConnection.value.id,
      currentTreeName.value === 'default' ? null : currentTreeName.value,
      row.key
    )
    
    ElMessage.success('删除成功')
    
    // 如果删除的是当前选中项，清空选中状态
    if (selectedItem.value && selectedItem.value.key === row.key) {
      selectedItem.value = null
    }
    
    // 刷新数据
    await loadData()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const deleteSelectedItem = async () => {
  if (!selectedItem.value) return
  await deleteItem(selectedItem.value)
}

const handleFileChange = (file: any) => {
  importFile.value = file.raw
}

const importData = async () => {
  if (!importFile.value || !currentConnection.value) return
  
  try {
    const text = await importFile.value.text()
    let importData: any[]
    
    if (importFile.value.name.endsWith('.json')) {
      importData = JSON.parse(text)
    } else if (importFile.value.name.endsWith('.csv')) {
      // 简单的CSV解析，实际项目中可能需要更复杂的处理
      const lines = text.split('\n')
      const headers = lines[0].split(',')
      importData = lines.slice(1).map(line => {
        const values = line.split(',')
        const obj: any = {}
        headers.forEach((header, index) => {
          obj[header.trim()] = values[index]?.trim()
        })
        return obj
      })
    } else {
      throw new Error('不支持的文件格式')
    }
    
    // 转换数据格式
    const keyValueData = importData.map(item => {
      const key = new TextEncoder().encode(item.key || item.Key || item.KEY).reduce((acc, byte) => [...acc, byte], [] as number[])
      const value = new TextEncoder().encode(item.value || item.Value || item.VALUE).reduce((acc, byte) => [...acc, byte], [] as number[])
      
      return {
        key,
        value,
        value_type: 'String' // 简化处理，实际项目中可能需要更复杂的类型检测
      }
    })
    
    const count = await sledStore.importData(
      currentConnection.value.id,
      currentTreeName.value === 'default' ? null : currentTreeName.value,
      JSON.stringify(keyValueData)
    )
    
    ElMessage.success(`成功导入 ${count} 条数据`)
    showImportDialog.value = false
    importFile.value = null
    await loadData()
  } catch (error) {
    ElMessage.error(`导入失败: ${error}`)
  }
}

const exportData = async () => {
  if (!currentConnection.value) return
  
  try {
    // 创建临时文件路径用于导出
    const timestamp = new Date().getTime();
    const filePath = `export_${currentConnection.value.id}_${currentTreeName.value}_${timestamp}.json`;
    const exportResult = await sledStore.exportData(
      currentConnection.value.id,
      currentTreeName.value === 'default' ? null : currentTreeName.value,
      'json',
      filePath
    );
    
    // 由于export_data命令直接写入文件并返回成功消息，我们需要重新获取数据用于下载
    const exportData = data.value.map(item => ({
      key: item.key,
      value: item.value,
      value_type: item.value_type
    })) as any[]
    
    // 转换数据格式
    const jsonData = exportData.map(item => ({
      key: formatKey(item.key),
      value: formatValue(item.value, item.value_type),
      value_type: item.value_type
    }))
    
    // 创建下载链接
    const blob = new Blob([JSON.stringify(jsonData, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = `${currentTreeName.value}_${new Date().toISOString().slice(0, 10)}.json`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
    
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败')
  }
}

// 监听当前连接变化
watch(currentConnection, (newConnection) => {
  if (newConnection) {
    expandedKeys.value = [newConnection.id]
    loadData()
  }
})

// 键盘快捷键支持
const setupKeyboardShortcuts = () => {
  const handleKeyDown = (event: KeyboardEvent) => {
    // 检查是否在输入框中，如果是则忽略快捷键
    const target = event.target as HTMLElement
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
      return
    }
    
    // Ctrl/Cmd + S: 保存编辑
    if ((event.ctrlKey || event.metaKey) && event.key === 's') {
      event.preventDefault()
      if (isEditing.value && selectedItem.value) {
        saveEdit()
      }
    }
    
    // Ctrl/Cmd + E: 快速编辑
    if ((event.ctrlKey || event.metaKey) && event.key === 'e') {
      event.preventDefault()
      if (selectedItem.value && !isEditing.value) {
        isEditing.value = true
      }
    }
    
    // Ctrl/Cmd + D: 删除选中项
    if ((event.ctrlKey || event.metaKey) && event.key === 'd') {
      event.preventDefault()
      if (selectedItem.value) {
        deleteSelectedItem()
      }
    }
    
    // Ctrl/Cmd + F: 聚焦搜索框
    if ((event.ctrlKey || event.metaKey) && event.key === 'f') {
      event.preventDefault()
      const searchInput = document.querySelector('.filter-container input') as HTMLInputElement
      if (searchInput) {
        searchInput.focus()
      }
    }
    
    // Ctrl/Cmd + R: 刷新数据
    if ((event.ctrlKey || event.metaKey) && event.key === 'r') {
      event.preventDefault()
      refreshData()
    }
    
    // Escape: 取消编辑
    if (event.key === 'Escape') {
      if (isEditing.value) {
        isEditing.value = false
      }
    }
    
    // F2: 编辑表名（如果选中了表）
    if (event.key === 'F2') {
      event.preventDefault()
      if (currentTreeName.value) {
        editTreeName({ label: currentTreeName.value })
      }
    }
  }
  
  // 添加事件监听器
  document.addEventListener('keydown', handleKeyDown)
  
  // 返回清理函数
  return () => {
    document.removeEventListener('keydown', handleKeyDown)
  }
}

// 快捷键帮助提示
const showShortcutHelp = () => {
  ElMessageBox.alert(`
    <div style="line-height: 1.6;">
      <h3 style="margin-bottom: 15px;">键盘快捷键</h3>
      <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px;">
        <div><strong>Ctrl/Cmd + S:</strong> 保存编辑</div>
        <div><strong>Ctrl/Cmd + E:</strong> 快速编辑</div>
        <div><strong>Ctrl/Cmd + D:</strong> 删除选中项</div>
        <div><strong>Ctrl/Cmd + F:</strong> 聚焦搜索</div>
        <div><strong>Ctrl/Cmd + R:</strong> 刷新数据</div>
        <div><strong>Esc:</strong> 取消编辑</div>
        <div><strong>F2:</strong> 编辑表名</div>
      </div>
    </div>
  `, '键盘快捷键', {
    dangerouslyUseHTMLString: true,
    confirmButtonText: '知道了'
  })
}

// 初始化
onMounted(() => {
  if (currentConnection.value) {
    expandedKeys.value = [currentConnection.value.id]
    loadData()
  }
  
  // 设置键盘快捷键
  const cleanup = setupKeyboardShortcuts()
  
  // 在组件卸载时清理
  onUnmounted(() => {
    cleanup()
  })
})
</script>

<style scoped>
.data-container {
  height: 100%;
}

.tree-card, .stats-card, .data-card, .action-card {
  margin-bottom: 20px;
  height: fit-content;
}

.tree-card {
  max-height: 400px;
}

.stats-card {
  max-height: 200px;
}

.data-card {
  min-height: 600px;
}

.action-card {
  min-height: 400px;
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

.custom-tree-node {
  display: flex;
  align-items: center;
  gap: 5px;
  width: 100%;
}

.tree-count {
  margin-left: auto;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.tree-actions {
  display: none;
  margin-left: 10px;
}

.custom-tree-node:hover .tree-actions {
  display: inline-flex;
}

.tree-actions .el-button {
  margin-left: 5px;
}

.filter-container {
  display: flex;
  align-items: center;
  margin-bottom: 15px;
  padding: 10px;
  background-color: var(--el-bg-color-page);
  border-radius: 4px;
}

.list-view {
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
}

.list-item {
  padding: 10px;
  border-bottom: 1px solid var(--el-border-color-lighter);
  cursor: pointer;
}

.list-item:hover {
  background-color: var(--el-bg-color-page);
}

.list-item:last-child {
  border-bottom: none;
}

.list-item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}

.list-item-key {
  font-weight: bold;
}

.list-item-value {
  font-family: monospace;
  font-size: 12px;
  color: var(--el-text-color-regular);
  word-break: break-all;
}

.json-view {
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 10px;
  background-color: var(--el-bg-color-page);
}

.json-view pre {
  margin: 0;
  font-family: monospace;
  font-size: 12px;
  white-space: pre-wrap;
  word-break: break-all;
}

.pagination-container {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.batch-operations, .selected-item {
  margin-bottom: 20px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

/* 选中行样式 */
.selected-row {
  background-color: var(--el-color-primary-light-9) !important;
}

.selected-row:hover {
  background-color: var(--el-color-primary-light-8) !important;
}

/* 操作面板优化样式 */
.selected-item-details {
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 6px;
  padding: 16px;
  background-color: var(--el-bg-color-page);
}

.selected-item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.selected-item-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.selected-item-actions {
  display: flex;
  gap: 8px;
}

.selected-item-content {
  margin-bottom: 16px;
}

.selected-item-field {
  margin-bottom: 12px;
}

.selected-item-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-secondary);
  margin-bottom: 4px;
}

.selected-item-value {
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
  font-size: 13px;
  line-height: 1.4;
  word-break: break-all;
  white-space: pre-wrap;
}

.selected-item-edit {
  margin-top: 16px;
}

.selected-item-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 16px;
}

/* 表格截断样式优化 */
.truncated-text {
  display: inline-block;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  vertical-align: middle;
}

.key-column {
  max-width: 170px;
}

.value-column {
  max-width: 270px;
}

.type-column {
  max-width: 80px;
}

.action-column {
  max-width: 120px;
}
</style>