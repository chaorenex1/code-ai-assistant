<script setup lang="ts">
import {
  Link,
  Delete,
  Setting,
  Search,
  Folder,
  Document,
  Clock,
  DocumentCopy,
  Loading,
} from '@element-plus/icons-vue';
import { ElInput, ElButton, ElSelect, ElOption, ElTooltip, ElTag, ElDialog, ElIcon, ElMessageBox } from 'element-plus';
import { ref, computed, nextTick, watch, onMounted, onUnmounted, reactive } from 'vue';
import { storeToRefs } from 'pinia';
import { marked } from 'marked';
import { convertFileSrc, invoke as tauriInvoke } from '@tauri-apps/api/core';

marked.setOptions({
  breaks: true,
});

import { useFileStore, useAppStore, useChatStore } from '@/stores';
import ChatHistoryDialog from '@/components/chat/ChatHistoryDialog.vue';
import { normalizePath } from '@/utils/pathUtils';
import { showSuccess, showError, showWarning } from '@/utils/toast';
import { parseCliArgs } from '@/utils/helpers';

const appStore = useAppStore();
const fileStore = useFileStore();
const chatStore = useChatStore();
const { messages, associatedFiles, isStreaming, currentRequestId } = storeToRefs(chatStore);

interface ClipboardImageEntry {
  filePath: string;
  fileName: string;
  width: number;
  height: number;
  preview: string;
}

const message = ref('');
const messagesContainer = ref<HTMLElement | null>(null);
const isMarkdownMode = ref(true);
const clipboardImages = ref<ClipboardImageEntry[]>([]);
const contextMenu = reactive({
  visible: false,
  x: 0,
  y: 0,
});

// 历史记录相关
const showHistoryDialog = ref(false);

// 关联文件弹窗
const showAssociateDialog = ref(false);
const associateSearch = ref('');
const recentFiles = ref<string[]>([]);
const selectedPaths = ref<string[]>([]);
// const currentSessionId = computed(() => chatStore.currentSessionId || '');


onMounted(() => {
  chatStore.setCurrentCodeCli(appStore.currentCodeCli);
});

onUnmounted(() => {
  chatStore.setCurrentCodeCli('');
  closeContextMenu();
});

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

function scrollMessagesToBottom() {
  void nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

watch(
  messages,
  () => {
    scrollMessagesToBottom();
  },
  { deep: true }
);

// Send message
async function sendMessage() {
  if (!message.value.trim() || isStreaming.value) {
    return;
  }

  const clipboardAttachments = clipboardImages.value.map((entry) => ({
    path: entry.filePath,
    width: entry.width,
    height: entry.height,
    preview: entry.preview,
  }));
  const content = message.value.trim();
  const contextFiles = [...associatedFiles.value, ...clipboardAttachments.map((entry) => entry.path)];

  const directCli = appStore.settings.ai.useDirectCodeCli ?? false;
  const selectedCli = appStore.settings.codeCli.find(
    (cli) => cli.name === appStore.currentCodeCli
  );
  const cliCommand = selectedCli?.command?.trim() || '';
  const cliArgs = selectedCli ? parseCliArgs(selectedCli.args || '') : [];

  try {
    await chatStore.sendMessage({
      content,
      files: contextFiles,
      codeCli: appStore.currentCodeCli,
      workspaceId: appStore.getCurrentWorkspace.id,
      workspaceDir: normalizePath(appStore.getCurrentWorkspace.path),
      resumeSessionId: '',
      model: appStore.currentAiModel,
      clipboardAttachments,
      directCli,
      cliCommand,
      cliArgs,
    });
    message.value = '';
    clipboardImages.value = [];
    scrollMessagesToBottom();
  } catch (error) {
    console.error('Failed to send message:', error);
  }
}

// Clear chat
function clearChat() {
  ElMessageBox.confirm('确定要清空聊天记录吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning',
  })
    .then(() => {
      chatStore.clearChat();
    })
    .catch(() => {
      // 用户取消
    });
}

// Remove associated file
function removeAssociatedFile(index: number) {
  chatStore.removeAssociatedFile(index);
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
  chatStore.setAssociatedFiles([...selectedPaths.value]);
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

function renderMarkdown(content: string) {
  return marked.parse(content || '');
}

async function copyMessage(content: string) {
  try {
    await navigator.clipboard.writeText(content || '');
    showSuccess('消息已复制');
  } catch (error) {
    console.error('Failed to copy message:', error);
    showError('复制失败');
  }
}

function removeClipboardImage(index: number) {
  clipboardImages.value.splice(index, 1);
}

async function getImageDimensions(blob: Blob): Promise<{ width: number; height: number }> {
  return new Promise((resolve) => {
    const img = new Image();
    const url = URL.createObjectURL(blob);
    img.onload = () => {
      resolve({ width: img.width, height: img.height });
      URL.revokeObjectURL(url);
    };
    img.onerror = () => {
      resolve({ width: 0, height: 0 });
      URL.revokeObjectURL(url);
    };
    img.src = url;
  });
}

async function persistClipboardBlob(blob: Blob, extensionHint?: string) {
  const extension = extensionHint || blob.type.split('/')[1] || 'png';
  const fileName = `codex-clipboard-${Date.now().toString(36)}-${Math.random()
    .toString(16)
    .slice(2)}.${extension}`;
  const arrayBuffer = await blob.arrayBuffer();
  const bytes = Array.from(new Uint8Array(arrayBuffer));
  const filePath = await tauriInvoke<string>('save_clipboard_image', {
    fileName,
    bytes,
  });
  const { width, height } = await getImageDimensions(blob);
  clipboardImages.value.push({
    filePath,
    fileName,
    width,
    height,
    preview: convertFileSrc(filePath),
  });
}

function handleTextareaPaste(event: ClipboardEvent) {
  const items = event.clipboardData?.items;
  if (!items?.length) {
    return;
  }

  for (const item of items) {
    if (item.type.startsWith('image/')) {
      const file = item.getAsFile();
      if (!file) continue;
      void persistClipboardBlob(file, file.type.split('/')[1]).then(() => {
        showSuccess('已添加剪贴板图片');
      });
    }
  }
}

function closeContextMenu() {
  if (!contextMenu.visible) return;
  contextMenu.visible = false;
  document.removeEventListener('click', closeContextMenu);
}

function handleInputContextMenu(event: MouseEvent) {
  event.preventDefault();
  contextMenu.x = event.clientX;
  contextMenu.y = event.clientY;
  contextMenu.visible = true;
  document.addEventListener('click', closeContextMenu);
}

async function insertClipboardImageFromClipboard() {
  if (!navigator.clipboard || typeof navigator.clipboard.read !== 'function') {
    showWarning('当前环境不支持读取剪贴板');
    closeContextMenu();
    return;
  }

  try {
    const items = await navigator.clipboard.read();
    for (const item of items) {
      const type = item.types.find((t) => t.startsWith('image/'));
      if (!type) {
        continue;
      }
      const blob = await item.getType(type);
      await persistClipboardBlob(blob, type.split('/')[1]);
      showSuccess('已插入剪贴板图片');
      return;
    }
    showWarning('剪贴板中没有图片');
  } catch (error) {
    console.error('Failed to read clipboard image:', error);
    showError('读取剪贴板失败');
  } finally {
    closeContextMenu();
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Associated Files -->
    <div v-if="associatedFiles.length > 0" class="border-b border-border bg-surface p-2">
      <div class="flex items-center justify-between mb-1">
        <div class="flex items-center text-sm text-text-secondary">
          <ElIcon :size="14" class="mr-1">
            <Link />
          </ElIcon>
          关联文件:
        </div>
        <ElButton size="small" text @click="openAssociateDialog(true)"> 添加当前文件 </ElButton>
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
    <div ref="messagesContainer" class="flex-1 overflow-auto p-4 space-y-3 bg-surface/50">
      <div v-if="messages.length === 0" class="text-center text-text-secondary">
        暂无消息，输入内容开始对话。
      </div>

      <div v-else class="space-y-3">
        <div
          v-for="msg in messages"
          :key="msg.id"
          class="flex"
          :class="msg.role === 'user' ? 'justify-end' : 'justify-start'"
        >
          <div
            class="message-card max-w-[70%] rounded-lg px-3 py-3 text-sm shadow-sm relative"
            :class="msg.role === 'user' ? 'bg-primary text-white' : 'bg-surface text-text'"
          >
            <div class="copy-button">
              <ElTooltip content="复制消息" placement="top">
                <ElButton
                  :icon="DocumentCopy"
                  size="small"
                  text
                  class="copy-trigger"
                  @click="copyMessage(msg.content)"
                />
              </ElTooltip>
            </div>

            <div class="message-content" :class="{ markdown: isMarkdownMode }">
              <div v-if="isMarkdownMode" class="markdown-body" v-html="renderMarkdown(msg.content)" />
              <div v-else class="whitespace-pre-wrap break-words">
                {{ msg.content }}
              </div>
            </div>

            <div
              v-if="msg.files && msg.files.length"
              class="mt-2 flex flex-wrap gap-2 text-[11px] opacity-90 attachments"
            >
              <div
                v-for="file in msg.files"
                :key="file"
                class="attachment-item"
                :title="getFileName(file)"
              >
                <template v-if="msg.fileMetadata?.[file]">
                  <img
                    class="w-14 h-14 object-cover rounded mb-1"
                    :src="msg.fileMetadata[file]?.preview || convertFileSrc(file)"
                    alt="附件"
                  />
                  <div class="font-medium truncate w-24">{{ getFileName(file) }}</div>
                  <div class="text-[10px] text-text-secondary/80">
                    {{ msg.fileMetadata[file]?.width }}×{{ msg.fileMetadata[file]?.height }}
                  </div>
                </template>
                <template v-else>
                  <span class="px-2 py-1 rounded bg-black/10 dark:bg-white/10 block truncate max-w-[180px]">
                    {{ getFileName(file) }}
                  </span>
                </template>
              </div>
            </div>

            <div
              v-if="isStreaming && msg.id === currentRequestId"
              class="mt-2 flex items-center justify-between text-xs text-primary/90 status-row"
            >
              <div class="flex items-center">
                <ElIcon class="mr-1 animate-spin" :size="14">
                  <Loading />
                </ElIcon>
                <span>AI 正在生成...</span>
              </div>
              <ElButton size="small" text type="danger" @click="chatStore.cancelStreaming()">终止</ElButton>
            </div>

            <div class="mt-1 text-[11px] opacity-70 text-right">
              {{
                new Date(msg.timestamp).toLocaleTimeString('zh-CN', {
                  hour: '2-digit',
                  minute: '2-digit',
                })
              }}
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
              v-model="appStore.currentCodeCli"
              @change="appStore.setCurrentCodeCli"
            >
              <ElOption
                v-for="cli in appStore.availableCodeClis"
                :key="cli"
                :label="cli"
                :value="cli"
              />
            </ElSelect>
          </div>
        </div>

        <div class="flex items-center space-x-3 flex-wrap justify-end">
          <div class="flex items-center space-x-1">
            <span class="text-sm text-text-secondary">渲染:</span>
            <ElButton
              size="small"
              :type="isMarkdownMode ? 'primary' : 'default'"
              text
              @click="isMarkdownMode = true"
            >
              Markdown
            </ElButton>
            <ElButton
              size="small"
              :type="!isMarkdownMode ? 'primary' : 'default'"
              text
              @click="isMarkdownMode = false"
            >
              文本
            </ElButton>
          </div>

          <ElTooltip content="聊天历史" placement="bottom">
            <ElButton :icon="Clock" size="small" text @click="showHistoryDialog = true" />
          </ElTooltip>

          <ElTooltip content="关联当前文件" placement="bottom">
            <ElButton :icon="Link" size="small" text @click="openAssociateDialog(true)" />
          </ElTooltip>

          <ElTooltip content="清空聊天" placement="bottom">
            <ElButton :icon="Delete" size="small" text @click="clearChat" />
          </ElTooltip>
        </div>
      </div>

      <!-- Message Input -->
      <div class="flex items-end space-x-2" @contextmenu="handleInputContextMenu">
        <ElInput
          v-model="message"
          type="textarea"
          :rows="3"
          placeholder="请输入消息… (支持 Markdown)"
          resize="none"
          class="flex-1"
          @keydown="handleKeyPress"
          @paste="handleTextareaPaste"
        />

        <ElButton type="primary" :icon="Setting" :loading="isStreaming" @click="sendMessage">
          发送
        </ElButton>
      </div>

      <div v-if="clipboardImages.length" class="mt-2 flex flex-wrap gap-3">
        <div
          v-for="(img, index) in clipboardImages"
          :key="index"
          class="relative border border-dashed border-primary/40 rounded-lg p-2 bg-surface/60 w-28"
        >
          <img :src="img.preview" alt="clipboard preview" class="w-full h-20 object-cover rounded mb-1" />
          <div class="text-xs truncate" :title="img.fileName">{{ img.fileName }}</div>
          <div class="text-[10px] text-text-secondary/80">{{ img.width }}×{{ img.height }}</div>
          <ElButton size="small" text class="absolute top-0 right-0" @click="removeClipboardImage(index)">
            移除
          </ElButton>
        </div>
      </div>

      <!-- Input Hints -->
      <div class="mt-2 text-xs text-text-secondary">
        <div class="flex items-center justify-between">
          <span>按 Enter 发送，Shift+Enter 换行，右键输入框插入剪贴板图片</span>
          <span>Markdown 预览可用</span>
        </div>
      </div>
    </div>
  </div>

  <!-- 关联文件弹窗 -->
  <ElDialog v-model="showAssociateDialog" title="关联文件" width="640px">
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
        <div v-if="!filteredEditingFiles.length" class="text-xs text-text-secondary">
          暂无正在编辑的文件
        </div>
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
          <span class="ml-1 text-[11px] text-text-secondary/70"
            >(当前目录: {{ fileStore.currentDirectory }})</span
          >
        </div>
        <div v-if="!filteredDirectoryFiles.length" class="text-xs text-text-secondary">
          当前目录暂无文件/文件夹
        </div>
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
        <div v-if="!filteredRecentFiles.length" class="text-xs text-text-secondary">
          暂无最近关联的文件
        </div>
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
        <div class="text-xs text-text-secondary">已选择 {{ selectedPaths.length }} 个文件</div>
        <div>
          <ElButton size="small" @click="showAssociateDialog = false">取消</ElButton>
          <ElButton type="primary" size="small" @click="confirmAssociate">确定</ElButton>
        </div>
      </div>
    </template>
  </ElDialog>

  <!-- 聊天历史对话框 -->
  <ChatHistoryDialog v-model="showHistoryDialog" />

  <div
    v-if="contextMenu.visible"
    class="clipboard-context-menu"
    :style="{ top: `${contextMenu.y}px`, left: `${contextMenu.x}px` }"
  >
    <button type="button" @click.stop="insertClipboardImageFromClipboard">插入剪贴板图片</button>
  </div>
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

:deep(.markdown-body) {
  font-size: 0.95rem;
  line-height: 1.6;
}

:deep(.markdown-body code) {
  padding: 0.1rem 0.3rem;
  border-radius: 4px;
}

:deep(.markdown-body pre) {
  padding: 0.6rem;
  border-radius: 6px;
  overflow: auto;
}

:deep(.markdown-body a) {
  color: var(--el-color-primary);
}

.message-card {
  position: relative;
}

.copy-button {
  position: absolute;
  top: 0.2rem;
  right: 0.2rem;
}

.copy-trigger {
  opacity: 0.7;
}

.message-content {
  width: 100%;
  padding-right: 2.2rem;
}

.attachments .attachment-item {
  min-width: 5rem;
  border-radius: 0.5rem;
  padding: 0.2rem;
  background-color: rgba(0, 0, 0, 0.07);
}

.clipboard-context-menu {
  position: fixed;
  background: var(--el-bg-color-overlay);
  border: 1px solid var(--el-border-color);
  border-radius: 0.5rem;
  padding: 0.4rem 0.6rem;
  z-index: 50;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.clipboard-context-menu button {
  font-size: 0.85rem;
  color: var(--el-color-primary);
}

.animate-spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
