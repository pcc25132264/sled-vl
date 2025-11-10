import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 检查Tauri API是否可用
function isTauriEnvironment() {
  const hasWindow = typeof window !== 'undefined'
  const hasTauri = !!(window as any).__TAURI__
  const hasTauriIPC = !!(window as any).__TAURI_IPC__
  const hasTauriInternal = !!(window as any).__TAURI_INTERNALS__
  
  console.log('Tauri环境检测:', {
    hasWindow,
    hasTauri,
    hasTauriIPC,
    hasTauriInternal,
    windowKeys: hasWindow ? Object.keys(window).filter(key => key.includes('TAURI')) : []
  })
  
  // 更宽松的检测条件，只要有一个Tauri相关对象就认为是Tauri环境
  return hasWindow && (hasTauri || hasTauriIPC || hasTauriInternal)
}

// 安全的invoke函数包装器
async function safeInvoke<T = any>(command: string, args?: any): Promise<T> {
  // 每次调用时都检查Tauri API是否可用
  if (!isTauriEnvironment()) {
    const errorMsg = 'Tauri API不可用，请确保在Tauri桌面应用中运行\n\n' +
      '⚠️ 您当前可能在浏览器中运行此应用。\n' +
      '💡 请查找并打开名为"Sled Visualizer"的桌面应用窗口进行测试。'
    
    console.error('❌', errorMsg)
    
    // 尝试等待一段时间后再次检查
    await new Promise(resolve => setTimeout(resolve, 500))
    if (isTauriEnvironment()) {
      console.log('✅ 延迟检测到Tauri环境，继续执行命令')
    } else {
      console.error('❌ 延迟检测后仍未发现Tauri环境')
      throw new Error(errorMsg)
    }
  }
  
  try {
    // 直接使用从@tauri-apps/api/core导入的invoke函数
    // 这比从window对象获取更可靠
    console.log(`调用Tauri命令: ${command}`, args)
    return await invoke<T>(command, args)
  } catch (error) {
    console.error(`Tauri API调用失败 (${command}):`, error)
    throw error
  }
}

// 初始化Tauri API检测
export function initTauriDetection() {
  console.log('开始初始化Tauri检测...')
  
  // 立即检查一次
  console.log('立即Tauri环境检测:', isTauriEnvironment())
  
  // 延迟检查，确保Tauri环境完全初始化
  setTimeout(() => {
    const result = isTauriEnvironment()
    console.log('延迟Tauri环境检测结果:', result)
    if (result) {
      console.log('✅ Tauri API检测成功')
    } else {
      console.warn('❌ Tauri API不可用，应用将在浏览器模式下运行')
      console.warn('请确保在Tauri桌面应用中运行，而不是在浏览器中')
    }
  }, 1000)
  
  // 再次延迟检查，给Tauri更多时间初始化
  setTimeout(() => {
    const result = isTauriEnvironment()
    console.log('第二次延迟Tauri环境检测结果:', result)
    if (result) {
      console.log('✅ Tauri API检测成功（第二次检查）')
    } else {
      console.warn('❌ Tauri API仍然不可用（第二次检查）')
    }
  }, 3000)
}

export interface Connection {
  id: string
  name: string
  path: string
  created_at: string
  last_accessed: string
}

export interface KeyValue {
  key: number[]
  value: number[]
  value_type: 'String' | 'Number' | 'Boolean' | 'Json' | 'Binary'
}

export interface DbStats {
  size_on_disk: number
  key_count: number
  tree_count: number
  last_modified: string
}

export const useSledStore = defineStore('sled', () => {
  // State
  const connections = ref<Connection[]>([])
  const currentConnectionId = ref<string | null>(null)
  const currentTree = ref<string>('default')
  const trees = ref<string[]>([])
  const stats = ref<DbStats | null>(null)
  const isLoading = ref(false)
  
  // 初始化Tauri检测
  initTauriDetection()
  
  // Actions
  async function loadConnections() {
    try {
      isLoading.value = true
      connections.value = await safeInvoke('get_connections')
    } catch (error) {
      console.error('Failed to load connections:', error)
      connections.value = []
    } finally {
      isLoading.value = false
    }
  }
  
  async function createConnection(name: string, path: string): Promise<Connection | undefined> {
    try {
      isLoading.value = true
      
      // 先检查Tauri环境
      if (!isTauriEnvironment()) {
        console.error('Tauri API不可用，无法创建连接')
        return undefined
      }
      
      // 修复参数格式，按照Rust后端期望的request结构传递
      // 移除路径末尾的db目录，因为sled库内部会自动添加/db
      let cleanPath = path;
      // 检查并移除末尾的/db或\db
      if (cleanPath.endsWith('/db')) {
        cleanPath = cleanPath.slice(0, -3);
      } else if (cleanPath.endsWith('\\db')) {
        cleanPath = cleanPath.slice(0, -3);
      }
      // 确保路径不是以/或\结尾
      if (cleanPath.endsWith('/') || cleanPath.endsWith('\\')) {
        cleanPath = cleanPath.slice(0, -1);
      }
      const id = await safeInvoke<string>('create_connection', { request: { name, path: cleanPath } })
      const newConnection: Connection = { 
        id, 
        name, 
        path: cleanPath, 
        created_at: new Date().toISOString(), 
        last_accessed: new Date().toISOString() 
      }
      connections.value.push(newConnection)
      return newConnection
    } catch (error) {
      console.error('Failed to create connection:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }
  
  // 创建数据库方法
  async function createDatabase(name: string, path: string): Promise<Connection | undefined> {
    try {
      isLoading.value = true
      
      // 先检查Tauri环境
      if (!isTauriEnvironment()) {
        console.error('Tauri API不可用，无法创建数据库')
        return undefined
      }
      
      // 确保路径是有效的目录，而不是现有文件
      // 移除路径末尾的db目录，因为sled库内部会自动添加/db
      let cleanPath = path;
      // 检查并移除末尾的/db或\db
      if (cleanPath.endsWith('/db')) {
        cleanPath = cleanPath.slice(0, -3);
      } else if (cleanPath.endsWith('\\db')) {
        cleanPath = cleanPath.slice(0, -3);
      }
      // 确保路径不是以/或\结尾
      if (cleanPath.endsWith('/') || cleanPath.endsWith('\\')) {
        cleanPath = cleanPath.slice(0, -1);
      }
      
      // 创建数据库连接（sled会自动创建数据库文件）
      const id = await safeInvoke<string>('create_connection', { request: { name, path: cleanPath } })
      const newConnection: Connection = { 
        id, 
        name, 
        path: cleanPath, 
        created_at: new Date().toISOString(), 
        last_accessed: new Date().toISOString() 
      }
      connections.value.push(newConnection)
      return newConnection
    } catch (error) {
      console.error('Failed to create database:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }
  
  async function removeConnection(id: string) {
    try {
      await safeInvoke('remove_connection', { connectionId: id })
      connections.value = connections.value.filter(conn => conn.id !== id)
      if (currentConnectionId.value === id) {
        currentConnectionId.value = null
      }
    } catch (error) {
      console.error('Failed to remove connection:', error)
      throw error
    }
  }
  
  async function loadTrees(connectionId: string) {
    try {
      isLoading.value = true
      trees.value = await safeInvoke('get_trees', { connectionId })
    } catch (error) {
      console.error('Failed to load trees:', error)
      trees.value = []
    } finally {
      isLoading.value = false
    }
  }
  
  async function loadStats(connectionId: string) {
    try {
      isLoading.value = true
      stats.value = await safeInvoke('get_stats', { connectionId })
    } catch (error) {
      console.error('Failed to load stats:', error)
      stats.value = { size_on_disk: 0, key_count: 0, tree_count: 0, last_modified: new Date().toISOString() }
    } finally {
      isLoading.value = false
    }
  }
  
  async function get(connectionId: string, treeName: string | null, key: number[]) {
    try {
      return await safeInvoke('get', { request: { connection_id: connectionId, tree_name: treeName, key } })
    } catch (error) {
      console.error('Failed to get value:', error)
      throw error
    }
  }
  
  async function set(connectionId: string, treeName: string | null, key: number[], value: number[]) {
    try {
      await safeInvoke('set', { request: { connection_id: connectionId, tree_name: treeName, key, value } })
    } catch (error) {
      console.error('Failed to set value:', error)
      throw error
    }
  }
  
  async function remove(connectionId: string, treeName: string | null, key: number[]) {
    try {
      await safeInvoke('remove', { request: { connection_id: connectionId, tree_name: treeName, key } })
    } catch (error) {
      console.error('Failed to remove value:', error)
      throw error
    }
  }
  
  async function queryRange(connectionId: string, treeName: string | null, start: number[], end: number[]) {
    try {
      return await safeInvoke('query_range', { 
        request: { 
          connection_id: connectionId, 
          tree_name: treeName, 
          query: { from: start, to: end, limit: null, reverse: false } 
        } 
      })
    } catch (error) {
      console.error('Failed to query range:', error)
      throw error
    }
  }
  
  async function queryPrefix(connectionId: string, treeName: string | null, prefix: number[]) {
    try {
      return await safeInvoke('prefix_query', { 
        request: { 
          connection_id: connectionId, 
          tree_name: treeName, 
          query: { prefix, limit: null } 
        } 
      })
    } catch (error) {
      console.error('Failed to query prefix:', error)
      throw error
    }
  }
  
  // 支持Data.vue中使用的prefixQuery方法，带options参数
  async function prefixQuery(connectionId: string, treeName: string | null, options: { prefix: number[], limit?: number }) {
    try {
      return await safeInvoke('prefix_query', {
        request: {
          connection_id: connectionId,
          tree_name: treeName,
          query: { prefix: options.prefix, limit: options.limit || null }
        }
      })
    } catch (error) {
      console.error('Failed to prefix query:', error)
      throw error
    }
  }
  
  // 导入数据方法
  async function importData(connectionId: string, treeName: string | null, data: string) {
    try {
      // 解析数据为KeyValue数组格式
      const parsedData = JSON.parse(data);
      const count = await safeInvoke<number>('import_data', {
        connection_id: connectionId,
        tree_name: treeName,
        data: parsedData
      })
      return count
    } catch (error) {
      console.error('Failed to import data:', error)
      throw error
    }
  }
  
  // 导出数据方法
  async function exportData(connectionId: string, treeName: string | null, format: string, filePath: string) {
    try {
      const result = await safeInvoke<string>('export_data', {
        connection_id: connectionId,
        tree_name: treeName,
        format,
        file_path: filePath
      })
      return result
    } catch (error) {
      console.error('Failed to export data:', error)
      throw error
    }
  }
  
  async function createTree(connectionId: string, treeName: string) {
    try {
      await safeInvoke('create_tree', { connectionId, treeName })
      // 重新加载树列表
      await loadTrees(connectionId)
    } catch (error) {
      console.error('Failed to create tree:', error)
      throw error
    }
  }
  
  async function removeTree(connectionId: string, treeName: string) {
    try {
      await safeInvoke('remove_tree', { connectionId, treeName })
      // 重新加载树列表
      await loadTrees(connectionId)
    } catch (error) {
      console.error('Failed to remove tree:', error)
      throw error
    }
  }
  
  function setCurrentConnection(id: string | null) {
    currentConnectionId.value = id
  }
  
  function setCurrentTree(tree: string) {
    currentTree.value = tree
  }
  
  return {
    // State
    connections,
    currentConnectionId,
    currentTree,
    trees,
    stats,
    isLoading,
    
    // Actions
    loadConnections,
    createConnection,
    createDatabase,
    removeConnection,
    loadTrees,
    loadStats,
    get,
    set,
    remove,
    queryRange,
    queryPrefix,
    prefixQuery,
    importData,
    exportData,
    createTree,
    removeTree,
    setCurrentConnection,
    setCurrentTree
  }
}, {
  persist: {
    key: 'sled-connections',
    storage: localStorage,
    paths: ['connections', 'currentConnectionId']
  }
})