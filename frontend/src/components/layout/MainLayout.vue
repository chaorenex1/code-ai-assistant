<script setup lang="ts">
import { Menu, Setting, Folder, Message, Document } from '@element-plus/icons-vue';
import { ElContainer, ElHeader, ElMain, ElAside, ElFooter, ElMessage } from 'element-plus';
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';

import { useAppStore } from '@/stores/workspaceStore';
import { useFileStore } from '@/stores/filesStore';
import { addRecentDirectory, getRecentDirectories, type RecentDirectory } from '@/services/tauri/commands';
import ChatPanel from '@/components/chat/ChatPanel.vue';
import CodeEditor from '@/components/editor/CodeEditor.vue';
import FileExplorer from '@/components/file-explorer/FileExplorer.vue';
import OutputPanel from '@/components/output/OutputPanel.vue';
import TerminalPanel from '@/components/terminal/TerminalPanel.vue';

const appStore = useAppStore();
const fileStore = useFileStore();
const router = useRouter();

// Panel visibility
const showFileExplorer = ref(true);
const showBottomPanel = ref(true);

// Bottom panel tabs
const bottomTabs = [
  { key: 'chat', label: '聊天', icon: Message },
  { key: 'output', label: '输出', icon: Document },
  { key: 'terminal', label: '终端', icon: Message },
];

const activeBottomTab = ref('chat');

// Recent directories
const recentDirectories = ref<RecentDirectory[]>([]);

// Toggle panels
function toggleFileExplorer() {
  showFileExplorer.value = !showFileExplorer.value;
}

function toggleBottomPanel() {
  showBottomPanel.value = !showBottomPanel.value;
}

// Open settings page
function openSettings() {
  router.push('/settings');
}

// Load recent directories from backend
async function loadRecentDirectories() {
  try {
    const directories = await getRecentDirectories();
    recentDirectories.value = directories;
  } catch (error) {
    console.error('加载最近目录失败', error);
    recentDirectories.value = [];
  }
}

// Open a recent directory from header dropdown
async function openRecentDirectoryFromHeader(dir: RecentDirectory) {
  try {
    await fileStore.loadDirectory(dir.path);
    await addRecentDirectory(dir.path);
    router.push('/dashboard');
  } catch (error) {
    ElMessage.error('打开目录失败: ' + (error as Error).message);
    console.error('打开目录失败', error);
  }
}

function handleSelectRecentDirectory(command: RecentDirectory) {
  if (command && command.path) {
    openRecentDirectoryFromHeader(command);
  }
}

onMounted(() => {
  loadRecentDirectories();
});
</script>

<template>
  <ElContainer class="h-full w-full">
    <!-- Header -->
    <ElHeader class="flex items-center justify-between border-b border-border bg-surface px-4">
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2">
          <img
            src="/vite.svg"
            class="h-8 w-8"
            alt="Logo"
          >
          <span class="text-lg font-semibold">Code AI Assistant</span>
        </div>

        <div class="flex items-center space-x-2">
          <el-button
            :icon="Menu"
            text
            @click="toggleFileExplorer"
          >
            {{ showFileExplorer ? '隐藏导航' : '显示导航' }}
          </el-button>
        </div>
      </div>

      <div class="flex items-center space-x-4">
        <ElDropdown
          v-if="recentDirectories.length > 0"
          trigger="click"
          @command="handleSelectRecentDirectory"
        >
          <span class="recent-dropdown-trigger">
            <el-icon class="mr-1">
              <Folder />
            </el-icon>
            <span class="recent-dropdown-label">
              最近目录
            </span>
          </span>
          <template #dropdown>
            <ElDropdownMenu class="recent-dropdown-menu">
              <ElDropdownItem
                v-for="dir in recentDirectories"
                :key="dir.path"
                :command="dir"
              >
                <div class="recent-dir-item">
                  <div class="recent-dir-path">
                    {{ dir.path }}
                  </div>
                  <div class="recent-dir-time">
                    {{ new Date(dir.openedAt).toLocaleString('zh-CN') }}
                  </div>
                </div>
              </ElDropdownItem>
            </ElDropdownMenu>
          </template>
        </ElDropdown>

        <el-button-group>
          <el-button
            type="primary"
            disabled
          >
            编辑器
          </el-button>
          <el-button
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
        <div class="h-full flex flex-col">
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

              <el-button
                :icon="showBottomPanel ? 'ArrowDown' : 'ArrowUp'"
                text
                @click="toggleBottomPanel"
              >
                {{ showBottomPanel ? '隐藏面板' : '显示面板' }}
              </el-button>
            </div>
          </div>

          <!-- Bottom Panel -->
          <div
            v-if="showBottomPanel"
            class="h-64 border-t border-border overflow-hidden"
          >
            <ChatPanel v-if="activeBottomTab === 'chat'" />
            <OutputPanel v-else-if="activeBottomTab === 'output'" />
            <TerminalPanel v-else-if="activeBottomTab === 'terminal'" />
          </div>
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

.recent-dropdown-trigger {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  border: 1px solid var(--color-border);
  cursor: pointer;
  font-size: 13px;
  color: var(--color-text-secondary);
  transition: all 0.15s ease-in-out;
}

.recent-dropdown-trigger:hover {
  background-color: rgba(0, 0, 0, 0.04);
  color: var(--color-text);
}

.recent-dropdown-label {
  max-width: 160px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.recent-dir-item {
  display: flex;
  flex-direction: column;
  max-width: 320px;
}

.recent-dir-path {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.recent-dir-time {
  margin-top: 2px;
  font-size: 12px;
  color: var(--color-text-secondary);
}
</style>
