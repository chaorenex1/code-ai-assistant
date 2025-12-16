<script setup lang="ts">
import { ref, computed } from 'vue'
import { ElContainer, ElHeader, ElMain, ElAside, ElFooter } from 'element-plus'
import { Menu, Setting, Folder, Message, Document } from '@element-plus/icons-vue'
import FileExplorer from '../file-explorer/FileExplorer.vue'
import CodeEditor from '../editor/CodeEditor.vue'
import ChatPanel from '../chat/ChatPanel.vue'
import TerminalPanel from '../terminal/TerminalPanel.vue'
import OutputPanel from '../output/OutputPanel.vue'
import SettingsPanel from '../settings/SettingsPanel.vue'
import { useAppStore } from '../../stores/app'

const appStore = useAppStore()

// Active tab
const activeTab = ref('editor')

// Panel visibility
const showFileExplorer = ref(true)
const showBottomPanel = ref(true)

// Bottom panel tabs
const bottomTabs = [
  { key: 'chat', label: '聊天', icon: Message },
  { key: 'output', label: '输出', icon: Document },
  { key: 'terminal', label: '终端', icon: Message },
]

const activeBottomTab = ref('chat')

// Toggle panels
function toggleFileExplorer() {
  showFileExplorer.value = !showFileExplorer.value
}

function toggleBottomPanel() {
  showBottomPanel.value = !showBottomPanel.value
}

// Open settings
function openSettings() {
  activeTab.value = 'settings'
}
</script>

<template>
  <ElContainer class="h-full w-full">
    <!-- Header -->
    <ElHeader class="flex items-center justify-between border-b border-border bg-surface px-4">
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2">
          <img src="/vite.svg" class="h-8 w-8" alt="Logo" />
          <span class="text-lg font-semibold">Code AI Assistant</span>
        </div>

        <div class="flex items-center space-x-2">
          <el-button :icon="Menu" text @click="toggleFileExplorer">
            {{ showFileExplorer ? '隐藏文件' : '显示文件' }}
          </el-button>
        </div>
      </div>

      <div class="flex items-center space-x-4">
        <el-button-group>
          <el-button
            :type="activeTab === 'editor' ? 'primary' : 'default'"
            @click="activeTab = 'editor'"
          >
            编辑器
          </el-button>
          <el-button
            :type="activeTab === 'settings' ? 'primary' : 'default'"
            @click="openSettings"
          >
            <el-icon><Setting /></el-icon>
            设置
          </el-button>
        </el-button-group>
      </div>
    </ElHeader>

    <!-- Main Content -->
    <ElContainer class="flex-1">
      <!-- File Explorer Sidebar -->
      <ElAside
        v-if="showFileExplorer"
        class="w-64 border-r border-border bg-surface overflow-auto"
      >
        <FileExplorer />
      </ElAside>

      <!-- Main Content Area -->
      <ElMain class="flex-1 overflow-hidden">
        <!-- Editor View -->
        <div v-if="activeTab === 'editor'" class="h-full flex flex-col">
          <!-- Editor Area -->
          <div class="flex-1 overflow-hidden">
            <CodeEditor />
          </div>

          <!-- Bottom Panel Toggle -->
          <div class="border-t border-border bg-surface px-4 py-1">
            <div class="flex items-center justify-between">
              <el-button-group>
                <el-button
                  v-for="tab in bottomTabs"
                  :key="tab.key"
                  :type="activeBottomTab === tab.key ? 'primary' : 'default'"
                  :icon="tab.icon"
                  @click="activeBottomTab = tab.key"
                >
                  {{ tab.label }}
                </el-button>
              </el-button-group>

              <el-button :icon="showBottomPanel ? 'ArrowDown' : 'ArrowUp'" text @click="toggleBottomPanel">
                {{ showBottomPanel ? '隐藏面板' : '显示面板' }}
              </el-button>
            </div>
          </div>

          <!-- Bottom Panel -->
          <div v-if="showBottomPanel" class="h-64 border-t border-border overflow-hidden">
            <ChatPanel v-if="activeBottomTab === 'chat'" />
            <OutputPanel v-else-if="activeBottomTab === 'output'" />
            <TerminalPanel v-else-if="activeBottomTab === 'terminal'" />
          </div>
        </div>

        <!-- Settings View -->
        <div v-else-if="activeTab === 'settings'" class="h-full overflow-auto p-6">
          <SettingsPanel />
        </div>
      </ElMain>
    </ElContainer>

    <!-- Footer -->
    <ElFooter class="border-t border-border bg-surface px-4 py-2 text-sm text-text-secondary">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <span>工作区: {{ appStore.currentWorkspace }}</span>
          <span>|</span>
          <span>文件: {{ appStore.currentFile || '未选择文件' }}</span>
        </div>

        <div class="flex items-center space-x-4">
          <span>AI模型: {{ appStore.currentAiModel }}</span>
          <span>|</span>
          <span>状态: {{ appStore.isConnected ? '已连接' : '未连接' }}</span>
        </div>
      </div>
    </ElFooter>
  </ElContainer>
</template>

<style scoped>
:deep(.el-header) {
  padding: 0;
  height: 48px;
}

:deep(.el-aside) {
  width: 256px;
}

:deep(.el-footer) {
  padding: 8px 16px;
  height: 32px;
}

:deep(.el-main) {
  padding: 0;
}
</style>