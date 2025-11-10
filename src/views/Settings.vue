<template>
  <div class="settings-container">
    <el-row :gutter="10">
      <el-col :span="24">
        <el-tabs v-model="activeTab" @tab-click="handleTabClick">
          <!-- 通用设置 -->
          <el-tab-pane label="通用设置" name="general">
            <el-card>
              <template #header>
                <span>通用设置</span>
              </template>
              
              <el-form ref="generalSettingsFormRef" :model="generalSettings" label-width="150px">
                <el-form-item label="主题模式">
                  <el-radio-group v-model="generalSettings.theme" @change="handleThemeChange">
                    <el-radio label="light">亮色模式</el-radio>
                    <el-radio label="dark">暗黑模式</el-radio>
                    <el-radio label="auto">跟随系统</el-radio>
                  </el-radio-group>
                </el-form-item>
                
                <el-form-item label="语言">
                  <el-select v-model="generalSettings.language" style="width: 200px;">
                    <el-option label="简体中文" value="zh-CN" />
                    <el-option label="English" value="en-US" />
                  </el-select>
                </el-form-item>
                
                <el-form-item label="默认页面大小">
                  <el-input-number
                    v-model="generalSettings.defaultPageSize"
                    :min="10"
                    :max="1000"
                    :step="10"
                  />
                </el-form-item>
                
                <el-form-item label="自动刷新间隔">
                  <el-select v-model="generalSettings.autoRefreshInterval" style="width: 200px;">
                    <el-option label="不自动刷新" value="0" />
                    <el-option label="5秒" value="5" />
                    <el-option label="10秒" value="10" />
                    <el-option label="30秒" value="30" />
                    <el-option label="1分钟" value="60" />
                  </el-select>
                </el-form-item>
                
                <el-form-item label="显示确认对话框">
                  <el-switch v-model="generalSettings.showConfirmDialogs" />
                </el-form-item>
                
                <el-form-item>
                  <el-button type="primary" @click="saveGeneralSettings">保存设置</el-button>
                  <el-button @click="resetGeneralSettings">重置为默认</el-button>
                </el-form-item>
              </el-form>
            </el-card>
          </el-tab-pane>
          
          <!-- 界面设置 -->
          <el-tab-pane label="界面设置" name="ui">
            <el-card>
              <template #header>
                <span>界面设置</span>
              </template>
              
              <el-form ref="uiSettingsFormRef" :model="uiSettings" label-width="150px">
                <el-form-item label="侧边栏宽度">
                  <el-slider
                    v-model="uiSettings.sidebarWidth"
                    :min="200"
                    :max="400"
                    :step="10"
                    show-stops
                    style="width: 300px;"
                  />
                  <span style="margin-left: 10px;">{{ uiSettings.sidebarWidth }}px</span>
                </el-form-item>
                
                <el-form-item label="紧凑模式">
                  <el-switch v-model="uiSettings.compactMode" />
                </el-form-item>
                
                <el-form-item label="显示行号">
                  <el-switch v-model="uiSettings.showLineNumbers" />
                </el-form-item>
                
                <el-form-item label="字体大小">
                  <el-input-number
                    v-model="uiSettings.fontSize"
                    :min="12"
                    :max="20"
                    :step="1"
                  />
                  <span style="margin-left: 10px;">px</span>
                </el-form-item>
                
                <el-form-item label="代码编辑器主题">
                  <el-select v-model="uiSettings.editorTheme" style="width: 200px;">
                    <el-option label="VS Code Dark" value="vs-dark" />
                    <el-option label="VS Code Light" value="vs" />
                    <el-option label="High Contrast Dark" value="hc-black" />
                  </el-select>
                </el-form-item>
                
                <el-form-item label="数据视图默认列">
                  <el-checkbox-group v-model="uiSettings.defaultColumns">
                    <el-checkbox label="key">键</el-checkbox>
                    <el-checkbox label="value">值</el-checkbox>
                    <el-checkbox label="type">类型</el-checkbox>
                    <el-checkbox label="size">大小</el-checkbox>
                    <el-checkbox label="created_at">创建时间</el-checkbox>
                    <el-checkbox label="updated_at">更新时间</el-checkbox>
                  </el-checkbox-group>
                </el-form-item>
                
                <el-form-item>
                  <el-button type="primary" @click="saveUiSettings">保存设置</el-button>
                  <el-button @click="resetUiSettings">重置为默认</el-button>
                </el-form-item>
              </el-form>
            </el-card>
          </el-tab-pane>
          
          <!-- 连接设置 -->
          <el-tab-pane label="连接设置" name="connection">
            <el-card>
              <template #header>
                <span>连接设置</span>
              </template>
              
              <el-form ref="connectionSettingsFormRef" :model="connectionSettings" label-width="150px">
                <el-form-item label="连接超时时间">
                  <el-input-number
                    v-model="connectionSettings.connectionTimeout"
                    :min="5"
                    :max="60"
                    :step="5"
                  />
                  <span style="margin-left: 10px;">秒</span>
                </el-form-item>
                
                <el-form-item label="最大连接数">
                  <el-input-number
                    v-model="connectionSettings.maxConnections"
                    :min="1"
                    :max="10"
                    :step="1"
                  />
                </el-form-item>
                
                <el-form-item label="自动重连">
                  <el-switch v-model="connectionSettings.autoReconnect" />
                </el-form-item>
                
                <el-form-item label="重连间隔">
                  <el-input-number
                    v-model="connectionSettings.reconnectInterval"
                    :min="1"
                    :max="30"
                    :step="1"
                    :disabled="!connectionSettings.autoReconnect"
                  />
                  <span style="margin-left: 10px;">秒</span>
                </el-form-item>
                
                <el-form-item label="保活间隔">
                  <el-input-number
                    v-model="connectionSettings.keepAliveInterval"
                    :min="10"
                    :max="300"
                    :step="10"
                  />
                  <span style="margin-left: 10px;">秒</span>
                </el-form-item>
                
                <el-form-item label="连接池大小">
                  <el-input-number
                    v-model="connectionSettings.poolSize"
                    :min="1"
                    :max="20"
                    :step="1"
                  />
                </el-form-item>
                
                <el-form-item>
                  <el-button type="primary" @click="saveConnectionSettings">保存设置</el-button>
                  <el-button @click="resetConnectionSettings">重置为默认</el-button>
                </el-form-item>
              </el-form>
            </el-card>
          </el-tab-pane>
          
          <!-- 高级设置 -->
          <el-tab-pane label="高级设置" name="advanced">
            <el-card>
              <template #header>
                <span>高级设置</span>
              </template>
              
              <el-form ref="advancedSettingsFormRef" :model="advancedSettings" label-width="150px">
                <el-form-item label="启用调试模式">
                  <el-switch v-model="advancedSettings.debugMode" />
                </el-form-item>
                
                <el-form-item label="日志级别">
                  <el-select v-model="advancedSettings.logLevel" style="width: 200px;">
                    <el-option label="错误" value="error" />
                    <el-option label="警告" value="warn" />
                    <el-option label="信息" value="info" />
                    <el-option label="调试" value="debug" />
                  </el-select>
                </el-form-item>
                
                <el-form-item label="最大日志文件大小">
                  <el-input-number
                    v-model="advancedSettings.maxLogFileSize"
                    :min="1"
                    :max="100"
                    :step="1"
                  />
                  <span style="margin-left: 10px;">MB</span>
                </el-form-item>
                
                <el-form-item label="日志保留天数">
                  <el-input-number
                    v-model="advancedSettings.logRetentionDays"
                    :min="1"
                    :max="365"
                    :step="1"
                  />
                  <span style="margin-left: 10px;">天</span>
                </el-form-item>
                
                <el-form-item label="缓存大小">
                  <el-input-number
                    v-model="advancedSettings.cacheSize"
                    :min="100"
                    :max="10000"
                    :step="100"
                  />
                  <span style="margin-left: 10px;">条记录</span>
                </el-form-item>
                
                <el-form-item label="批量操作大小">
                  <el-input-number
                    v-model="advancedSettings.batchSize"
                    :min="10"
                    :max="1000"
                    :step="10"
                  />
                  <span style="margin-left: 10px;">条记录</span>
                </el-form-item>
                
                <el-form-item label="启用性能监控">
                  <el-switch v-model="advancedSettings.enablePerformanceMonitoring" />
                </el-form-item>
                
                <el-form-item label="数据导出格式">
                  <el-checkbox-group v-model="advancedSettings.exportFormats">
                    <el-checkbox label="json">JSON</el-checkbox>
                    <el-checkbox label="csv">CSV</el-checkbox>
                    <el-checkbox label="xml">XML</el-checkbox>
                    <el-checkbox label="yaml">YAML</el-checkbox>
                  </el-checkbox-group>
                </el-form-item>
                
                <el-form-item>
                  <el-button type="primary" @click="saveAdvancedSettings">保存设置</el-button>
                  <el-button @click="resetAdvancedSettings">重置为默认</el-button>
                </el-form-item>
              </el-form>
            </el-card>
          </el-tab-pane>
          
          <!-- 关于 -->
          <el-tab-pane label="关于" name="about">
            <el-card>
              <template #header>
                <span>关于</span>
              </template>
              
              <div class="about-content">
                <div class="app-info">
                  <h2>Sled Visualizer</h2>
                  <p>版本: 1.0.0</p>
                  <p>一个功能强大的 Sled 数据库可视化管理工具</p>
                </div>
                
                <el-divider />
                
                <div class="tech-stack">
                  <h3>技术栈</h3>
                  <el-row :gutter="20">
                    <el-col :span="12">
                      <h4>前端</h4>
                      <ul>
                        <li>Vue 3</li>
                        <li>TypeScript</li>
                        <li>Element Plus</li>
                        <li>Pinia</li>
                        <li>Vue Router</li>
                        <li>ECharts</li>
                        <li>Monaco Editor</li>
                      </ul>
                    </el-col>
                    <el-col :span="12">
                      <h4>后端</h4>
                      <ul>
                        <li>Rust</li>
                        <li>Tauri</li>
                        <li>Sled</li>
                        <li>Tokio</li>
                      </ul>
                    </el-col>
                  </el-row>
                </div>
                
                <el-divider />
                
                <div class="links">
                  <h3>相关链接</h3>
                  <el-space wrap>
                    <el-button type="primary" link @click="openLink('https://github.com/spacejam/sled')">
                      Sled 官方仓库
                    </el-button>
                    <el-button type="primary" link @click="openLink('https://tauri.app/')">
                      Tauri 官网
                    </el-button>
                    <el-button type="primary" link @click="openLink('https://vuejs.org/')">
                      Vue 3 官网
                    </el-button>
                    <el-button type="primary" link @click="openLink('https://element-plus.org/')">
                      Element Plus 官网
                    </el-button>
                  </el-space>
                </div>
                
                <el-divider />
                
                <div class="license">
                  <h3>许可证</h3>
                  <p>MIT License</p>
                  <el-button type="text" @click="showLicense = true">查看许可证详情</el-button>
                </div>
              </div>
            </el-card>
          </el-tab-pane>
        </el-tabs>
      </el-col>
    </el-row>
    
    <!-- 许可证对话框 -->
    <el-dialog v-model="showLicense" title="MIT License" width="600px">
      <div class="license-content">
        <pre>
MIT License

Copyright (c) 2023 Sled Visualizer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
        </pre>
      </div>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showLicense = false">关闭</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
// import { open } from '@tauri-apps/plugin-opener'

// 状态
const activeTab = ref('general')
const showLicense = ref(false)

// 表单引用
const generalSettingsFormRef = ref()
const uiSettingsFormRef = ref()
const connectionSettingsFormRef = ref()
const advancedSettingsFormRef = ref()

// 通用设置
const generalSettings = reactive({
  theme: 'light',
  language: 'zh-CN',
  defaultPageSize: 50,
  autoRefreshInterval: '30',
  showConfirmDialogs: true
})

// 界面设置
const uiSettings = reactive({
  sidebarWidth: 250,
  compactMode: false,
  showLineNumbers: true,
  fontSize: 14,
  editorTheme: 'vs-dark',
  defaultColumns: ['key', 'value', 'type']
})

// 连接设置
const connectionSettings = reactive({
  connectionTimeout: 10,
  maxConnections: 5,
  autoReconnect: true,
  reconnectInterval: 5,
  keepAliveInterval: 60,
  poolSize: 10
})

// 高级设置
const advancedSettings = reactive({
  debugMode: false,
  logLevel: 'info',
  maxLogFileSize: 10,
  logRetentionDays: 7,
  cacheSize: 1000,
  batchSize: 100,
  enablePerformanceMonitoring: true,
  exportFormats: ['json', 'csv']
})

// 方法
const handleTabClick = () => {
  // 标签页切换时的处理
}

const handleThemeChange = (value: string) => {
  // 应用主题变化
  if (value === 'dark') {
    document.documentElement.classList.add('dark')
  } else if (value === 'light') {
    document.documentElement.classList.remove('dark')
  } else {
    // 跟随系统
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    if (prefersDark) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }
}

const saveGeneralSettings = () => {
  // 保存通用设置到本地存储
  localStorage.setItem('sled-vl-general-settings', JSON.stringify(generalSettings))
  
  // 应用主题设置
  handleThemeChange(generalSettings.theme)
  
  ElMessage.success('通用设置已保存')
}

const resetGeneralSettings = () => {
  // 重置通用设置为默认值
  generalSettings.theme = 'light'
  generalSettings.language = 'zh-CN'
  generalSettings.defaultPageSize = 50
  generalSettings.autoRefreshInterval = '30'
  generalSettings.showConfirmDialogs = true
  
  ElMessage.success('通用设置已重置为默认值')
}

const saveUiSettings = () => {
  // 保存界面设置到本地存储
  localStorage.setItem('sled-vl-ui-settings', JSON.stringify(uiSettings))
  
  // 应用界面设置
  applyUiSettings()
  
  ElMessage.success('界面设置已保存')
}

const resetUiSettings = () => {
  // 重置界面设置为默认值
  uiSettings.sidebarWidth = 250
  uiSettings.compactMode = false
  uiSettings.showLineNumbers = true
  uiSettings.fontSize = 14
  uiSettings.editorTheme = 'vs-dark'
  uiSettings.defaultColumns = ['key', 'value', 'type']
  
  applyUiSettings()
  
  ElMessage.success('界面设置已重置为默认值')
}

const applyUiSettings = () => {
  // 应用界面设置
  document.documentElement.style.setProperty('--sidebar-width', `${uiSettings.sidebarWidth}px`)
  document.documentElement.style.setProperty('--font-size', `${uiSettings.fontSize}px`)
  
  // 应用紧凑模式
  if (uiSettings.compactMode) {
    document.body.classList.add('compact-mode')
  } else {
    document.body.classList.remove('compact-mode')
  }
}

const saveConnectionSettings = () => {
  // 保存连接设置到本地存储
  localStorage.setItem('sled-vl-connection-settings', JSON.stringify(connectionSettings))
  
  ElMessage.success('连接设置已保存')
}

const resetConnectionSettings = () => {
  // 重置连接设置为默认值
  connectionSettings.connectionTimeout = 10
  connectionSettings.maxConnections = 5
  connectionSettings.autoReconnect = true
  connectionSettings.reconnectInterval = 5
  connectionSettings.keepAliveInterval = 60
  connectionSettings.poolSize = 10
  
  ElMessage.success('连接设置已重置为默认值')
}

const saveAdvancedSettings = () => {
  // 保存高级设置到本地存储
  localStorage.setItem('sled-vl-advanced-settings', JSON.stringify(advancedSettings))
  
  ElMessage.success('高级设置已保存')
}

const resetAdvancedSettings = () => {
  // 重置高级设置为默认值
  advancedSettings.debugMode = false
  advancedSettings.logLevel = 'info'
  advancedSettings.maxLogFileSize = 10
  advancedSettings.logRetentionDays = 7
  advancedSettings.cacheSize = 1000
  advancedSettings.batchSize = 100
  advancedSettings.enablePerformanceMonitoring = true
  advancedSettings.exportFormats = ['json', 'csv']
  
  ElMessage.success('高级设置已重置为默认值')
}

const openLink = (url: string) => {
  window.open(url, '_blank')
}

const loadSettings = () => {
  // 从本地存储加载设置
  
  // 通用设置
  const generalSettingsStr = localStorage.getItem('sled-vl-general-settings')
  if (generalSettingsStr) {
    try {
      const saved = JSON.parse(generalSettingsStr)
      Object.assign(generalSettings, saved)
      handleThemeChange(generalSettings.theme)
    } catch (e) {
      console.error('Failed to load general settings', e)
    }
  }
  
  // 界面设置
  const uiSettingsStr = localStorage.getItem('sled-vl-ui-settings')
  if (uiSettingsStr) {
    try {
      const saved = JSON.parse(uiSettingsStr)
      Object.assign(uiSettings, saved)
      applyUiSettings()
    } catch (e) {
      console.error('Failed to load UI settings', e)
    }
  }
  
  // 连接设置
  const connectionSettingsStr = localStorage.getItem('sled-vl-connection-settings')
  if (connectionSettingsStr) {
    try {
      const saved = JSON.parse(connectionSettingsStr)
      Object.assign(connectionSettings, saved)
    } catch (e) {
      console.error('Failed to load connection settings', e)
    }
  }
  
  // 高级设置
  const advancedSettingsStr = localStorage.getItem('sled-vl-advanced-settings')
  if (advancedSettingsStr) {
    try {
      const saved = JSON.parse(advancedSettingsStr)
      Object.assign(advancedSettings, saved)
    } catch (e) {
      console.error('Failed to load advanced settings', e)
    }
  }
}

// 初始化
onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.settings-container {
  padding: 20px;
}

.about-content {
  max-width: 800px;
  margin: 0 auto;
}

.app-info {
  text-align: center;
  margin-bottom: 20px;
}

.app-info h2 {
  margin: 0 0 10px 0;
}

.tech-stack h3,
.links h3,
.license h3 {
  margin-top: 20px;
  margin-bottom: 10px;
}

.tech-stack ul {
  padding-left: 20px;
}

.tech-stack li {
  margin-bottom: 5px;
}

.license-content {
  max-height: 400px;
  overflow-y: auto;
}

.license-content pre {
  white-space: pre-wrap;
  word-break: break-word;
  font-family: monospace;
  font-size: 14px;
  line-height: 1.5;
  margin: 0;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
}
</style>