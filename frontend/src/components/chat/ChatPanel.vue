<script setup lang="ts">
import { Link, Delete, Setting, Search, Folder, Document } from '@element-plus/icons-vue';
import { ElInput, ElButton, ElSelect, ElOption, ElTooltip, ElTag, ElDialog, ElIcon } from 'element-plus';
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { listen } from '@tauri-apps/api/event';

import { useFileStore } from '@/stores/filesStore';
import { useAppStore } from '@/stores/workspaceStore';
import { sendChatMessageStreaming } from '@/services/tauri/commands';
import type { ChatMessage as ChatMessageType } from '@/utils/types';

const appStore = useAppStore();
const fileStore = useFileStore();

const message = ref('');
const isLoading = ref(false);
const associatedFiles = ref<string[]>([]);
const messages = ref<ChatMessageType[]>([]);
const messagesContainer = ref<HTMLElement | null>(null);
const currentRequestId = ref<string | null>(null);
let aiUnlisten: (() => void) | null = null;

// 关联文件弹窗
const showAssociateDialog = ref(false);
const associateSearch = ref('');
const recentFiles = ref<string[]>([]);
const selectedPaths = ref<string[]>([]);

function getFileName(path: string): string {
  const parts = path.split(/[/\\]/);
  return parts[parts.length - 1] || path;
}

const filteredEditingFiles = computed(() => {
  const q = associateSearch.value.trim().toLowerCase();
  const files = fileStore.openedFiles.map((f) => ({
    path: f.path,
    name: getFileName(f.path),
    active: fileStore.activeFile?.path === f.path,
  }));
  if (!q) return files;
  return files.filter((f) => f.name.toLowerCase().includes(q) || f.path.toLowerCase().includes(q));
});

const filteredDirectoryFiles = computed(() => {
  const q = associateSearch.value.trim().toLowerCase();
  const files = fileStore.files.map((f) => ({
    path: f.path,
    name: f.name,
    isDirectory: f.isDirectory,
  }));
  if (!q) return files;
  return files.filter((f) => f.name.toLowerCase().includes(q) || f.path.toLowerCase().includes(q));
});

const filteredRecentFiles = computed(() => {
  const q = associateSearch.value.trim().toLowerCase();
  const files = recentFiles.value.map((p) => ({ path: p, name: getFileName(p) }));
  if (!q) return files;
  return files.filter((f) => f.name.toLowerCase().includes(q) || f.path.toLowerCase().includes(q));
});

interface AiResponseEventPayload {
  request_id: string;
  delta: string;
  done: boolean;
}

async function setupAiResponseListener() {
  try {
    aiUnlisten = await listen<string>('ai-response', (event) => {
      try {
        const raw = event.payload;
        const data: AiResponseEventPayload = typeof raw === 'string' ? JSON.parse(raw) : (raw as any);

        if (!currentRequestId.value || data.request_id !== currentRequestId.value) return;

        const last = messages.value[messages.value.length - 1];
        if (!last || last.id !== data.request_id || last.role !== 'assistant') return;

        last.content += data.delta;

        // 流式追加内容后，自动滚动到底部
        void nextTick(() => {
          if (messagesContainer.value) {
            messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
          }
        });

        if (data.done) {
          isLoading.value = false;
          currentRequestId.value = null;
        }
      } catch (err) {
        console.error('Failed to handle ai-response event:', err);
      }
    });
  } catch (error) {
    console.error('Failed to listen ai-response events:', error);
  }
}

onMounted(() => {
  setupAiResponseListener();
});

onBeforeUnmount(() => {
  if (aiUnlisten) {
    aiUnlisten();
    aiUnlisten = null;
  }
});

// Send message
async function sendMessage() {
  if (!message.value.trim() || isLoading.value) {
    return;
  }

  const content = message.value.trim();
  const timestamp = new Date().toISOString();
  const contextFiles = [...associatedFiles.value];
  const model = appStore.currentAiModel;

  // 先推送用户消息到本地列表
  messages.value.push({
    id: `${Date.now()}-user`,
    role: 'user',
    content,
    timestamp,
    files: contextFiles,
    model,
  });

  try {
    isLoading.value = true;
    // 调用后端流式 AI 命令，返回本次会话的 requestId
    const requestId = await sendChatMessageStreaming(content, contextFiles);
    currentRequestId.value = requestId;

    // 先插入一个空内容的 assistant 气泡，后续通过事件流式填充
    messages.value.push({
      id: requestId,
      role: 'assistant',
      content: '',
      timestamp: new Date().toISOString(),
      files: contextFiles,
      model,
    });

    // Clear message
    message.value = '';
  } catch (error) {
    console.error('Failed to send message:', error);
  } finally {
    // 结束状态由流式事件在 done 时重置
  }
}

// Clear chat
function clearChat() {
  if (confirm('确定要清空聊天记录吗？')) {
    messages.value = [];
  }
}

// Remove associated file
function removeAssociatedFile(index: number) {
  associatedFiles.value.splice(index, 1);
}

// Handle key press
function handleKeyPress(event: KeyboardEvent | Event) {
  if (event instanceof KeyboardEvent && event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    sendMessage();
  }
}

function openAssociateDialog(fromCurrentFile = false) {
  selectedPaths.value = [...associatedFiles.value];
  if (fromCurrentFile && fileStore.activeFile) {
    const p = fileStore.activeFile.path;
    if (!selectedPaths.value.includes(p)) selectedPaths.value.push(p);
  }
  showAssociateDialog.value = true;
}

function toggleSelect(path: string) {
  const idx = selectedPaths.value.indexOf(path);
  if (idx === -1) {
    selectedPaths.value.push(path);
  } else {
    selectedPaths.value.splice(idx, 1);
  }
}

function isSelected(path: string) {
  return selectedPaths.value.includes(path);
}

function confirmAssociate() {
  associatedFiles.value = [...selectedPaths.value];
  // 更新最近关联文件列表（去重并按时间倒序）
  for (const p of selectedPaths.value) {
    const idx = recentFiles.value.indexOf(p);
    if (idx !== -1) recentFiles.value.splice(idx, 1);
    recentFiles.value.unshift(p);
  }
  if (recentFiles.value.length > 20) {
    recentFiles.value = recentFiles.value.slice(0, 20);
  }
  showAssociateDialog.value = false;
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Associated Files -->
    <div
      v-if="associatedFiles.length > 0"
      class="border-b border-border bg-surface p-2"
    >
      <div class="flex items-center justify-between mb-1">
        <div class="flex items-center text-sm text-text-secondary">
          <ElIcon
            :size="14"
            class="mr-1"
          >
            <Link />
          </ElIcon>
          关联文件:
        </div>
        <ElButton
          size="small"
          text
	      @click="openAssociateDialog(true)"
        >
          添加当前文件
        </ElButton>
      </div>
      <div class="flex flex-wrap gap-1">
        <ElTag
          v-for="(file, index) in associatedFiles"
          :key="file"
          size="small"
          closable
          @close="removeAssociatedFile(index)"
        >
	      {{ getFileName(file) }}
        </ElTag>
      </div>
    </div>

    <!-- Chat Messages Area -->
    <div
	  ref="messagesContainer"
	  class="flex-1 overflow-auto p-4 space-y-3 bg-surface/50"
	>
      <div
        v-if="messages.length === 0"
        class="text-center text-text-secondary"
      >
        暂无消息，输入内容开始对话。
      </div>

      <div
        v-else
        class="space-y-3"
      >
        <div
          v-for="msg in messages"
          :key="msg.id"
          class="flex"
          :class="msg.role === 'user' ? 'justify-end' : 'justify-start'"
        >
          <div
            class="max-w-[70%] rounded-lg px-3 py-2 text-sm shadow-sm"
            :class="msg.role === 'user' ? 'bg-primary text-white' : 'bg-surface text-text'"
          >
            <div class="whitespace-pre-wrap break-words">
              {{ msg.content }}
            </div>

            <!-- 关联文件展示，便于 AI 编程场景查看上下文 -->
            <div
              v-if="msg.files && msg.files.length"
              class="mt-2 flex flex-wrap gap-1 text-[11px] opacity-80"
            >
              <span class="mr-1">关联文件:</span>
              <span
                v-for="file in msg.files"
                :key="file"
                class="px-1 py-0.5 rounded bg-black/10 dark:bg-white/10 cursor-default max-w-[180px] truncate"
                :title="file"
              >
                {{ file.split(/[/\\\\]/).pop() }}
              </span>
            </div>

            <div class="mt-1 text-[11px] opacity-70 text-right">
              {{ new Date(msg.timestamp).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }) }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Input Area -->
    <div class="border-t border-border bg-surface p-4">
      <!-- Model Selection -->
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center space-x-4">
          <div class="flex items-center">
            <span class="text-sm text-text-secondary mr-2">模型:</span>
            <ElSelect
              v-model="appStore.currentAiModel"
              size="small"
              style="width: 180px"
              @change="appStore.setCurrentAiModel"
            >
              <ElOption
                v-for="model in appStore.availableAiModels"
                :key="model"
                :label="model"
                :value="model"
              />
            </ElSelect>
          </div>

          <div class="flex items-center">
            <span class="text-sm text-text-secondary mr-2">Code CLI:</span>
            <ElSelect
              size="small"
              style="width: 140px"
            >
              <ElOption
                label="OpenAI Codex"
                value="codex"
              />
              <ElOption
                label="Local Coder"
                value="local"
              />
            </ElSelect>
          </div>
        </div>

        <div class="flex items-center space-x-2">
          <ElTooltip
            content="关联当前文件"
            placement="bottom"
          >
            <ElButton
              :icon="Link"
              size="small"
              text
	        @click="openAssociateDialog(true)"
            />
          </ElTooltip>

          <ElTooltip
            content="清空聊天"
            placement="bottom"
          >
            <ElButton
              :icon="Delete"
              size="small"
              text
              @click="clearChat"
            />
          </ElTooltip>
        </div>
      </div>

      <!-- Message Input -->
      <div class="flex items-end space-x-2">
        <ElInput
          v-model="message"
          type="textarea"
          :rows="3"
          placeholder="请输入消息... (支持 Markdown)"
          resize="none"
          class="flex-1"
          @keydown="handleKeyPress"
        />

        <ElButton
          type="primary"
          :icon="Setting"
          :loading="isLoading"
          @click="sendMessage"
        >
          发送
        </ElButton>
      </div>

      <!-- Input Hints -->
      <div class="mt-2 text-xs text-text-secondary">
        <div class="flex items-center justify-between">
          <span>按 Enter 发送，Shift+Enter 换行</span>
          <span>Markdown 预览可用</span>
        </div>
      </div>
    </div>
  </div>

  <!-- 关联文件弹窗 -->
  <ElDialog
    v-model="showAssociateDialog"
    title="关联文件"
    width="640px"
  >
    <!-- 搜索栏 -->
    <div class="mb-3">
      <ElInput
        v-model="associateSearch"
        :prefix-icon="Search"
        size="small"
        placeholder="搜索正在编辑的文件或当前目录中的文件..."
        clearable
      />
    </div>

    <div class="max-h-[420px] space-y-4 overflow-auto pr-1">
      <!-- 正在编辑 -->
      <div>
        <div class="mb-2 text-xs font-semibold text-text-secondary">正在编辑</div>
        <div v-if="!filteredEditingFiles.length" class="text-xs text-text-secondary">暂无正在编辑的文件</div>
        <div v-else class="space-y-1">
          <div
            v-for="file in filteredEditingFiles"
            :key="file.path"
            class="flex items-center px-2 py-1 rounded cursor-pointer text-xs hover:bg-surface"
            :class="{ 'bg-primary/10': isSelected(file.path) }"
            @click="toggleSelect(file.path)"
          >
            <ElIcon :size="14" class="mr-2">
              <Document />
            </ElIcon>
            <span class="flex-1 truncate" :title="file.path">{{ file.name }}</span>
            <span v-if="file.active" class="ml-2 text-[11px] text-primary">当前</span>
          </div>
        </div>
      </div>

      <!-- 文件和文件夹（当前目录） -->
      <div>
        <div class="mb-2 text-xs font-semibold text-text-secondary">
          文件和文件夹
          <span class="ml-1 text-[11px] text-text-secondary/70">(当前目录: {{ fileStore.currentDirectory }})</span>
        </div>
        <div v-if="!filteredDirectoryFiles.length" class="text-xs text-text-secondary">当前目录暂无文件/文件夹</div>
        <div v-else class="space-y-1">
          <div
            v-for="file in filteredDirectoryFiles"
            :key="file.path"
            class="flex items-center px-2 py-1 rounded cursor-pointer text-xs hover:bg-surface"
            :class="{ 'bg-primary/10': isSelected(file.path) }"
            @click="toggleSelect(file.path)"
          >
            <ElIcon :size="14" class="mr-2">
              <Folder v-if="file.isDirectory" />
              <Document v-else />
            </ElIcon>
            <span class="flex-1 truncate" :title="file.path">{{ file.name }}</span>
          </div>
        </div>
      </div>

      <!-- 最近关联的文件 -->
      <div>
        <div class="mb-2 text-xs font-semibold text-text-secondary">最近关联的文件</div>
        <div v-if="!filteredRecentFiles.length" class="text-xs text-text-secondary">暂无最近关联的文件</div>
        <div v-else class="flex flex-wrap gap-1 text-[11px]">
          <span
            v-for="file in filteredRecentFiles"
            :key="file.path"
            class="px-2 py-1 rounded bg-black/5 dark:bg-white/5 cursor-pointer truncate max-w-[200px]"
            :class="{ 'bg-primary/20 text-primary': isSelected(file.path) }"
            :title="file.path"
            @click="toggleSelect(file.path)"
          >
            {{ file.name }}
          </span>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex items-center justify-between w-full">
        <div class="text-xs text-text-secondary">
          已选择 {{ selectedPaths.length }} 个文件
        </div>
        <div>
          <ElButton size="small" @click="showAssociateDialog = false">取消</ElButton>
          <ElButton type="primary" size="small" @click="confirmAssociate">确定</ElButton>
        </div>
      </div>
    </template>
  </ElDialog>
</template>

<style scoped>
:deep(.el-textarea__inner) {
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans',
    'Helvetica Neue', sans-serif;
  line-height: 1.5;
}

:deep(.el-select) {
  width: auto;
}
</style>
