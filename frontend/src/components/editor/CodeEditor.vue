<script setup lang="ts">
import { Document, FolderOpened } from '@element-plus/icons-vue';
import { ElTabs, ElTabPane, ElButton, ElIcon, ElTooltip } from 'element-plus';
import * as monaco from 'monaco-editor';
import { ref, onMounted, onUnmounted, watch, nextTick, computed } from 'vue';

import { useFileStore } from '@/stores/filesStore';
import { invoke } from '@tauri-apps/api/core';

const fileStore = useFileStore();
const editorContainer = ref<HTMLElement>();
const editor = ref<monaco.editor.IStandaloneCodeEditor>();
const isLoading = ref(false);
let pollTimer: number | null = null;

// 处理路径，兼容 Windows 和 POSIX
function getFileNameFromPath(path: string): string {
  const parts = path.split(/[/\\]/);
  return parts[parts.length - 1] || path;
}

const duplicateNames = computed(() => {
  const counts = new Map<string, number>();
  for (const file of fileStore.openedFiles) {
    const name = getFileNameFromPath(file.path);
    counts.set(name, (counts.get(name) || 0) + 1);
  }
  return new Set<string>(
    Array.from(counts.entries())
      .filter(([, count]) => count > 1)
      .map(([name]) => name)
  );
});

function getTabLabel(filePath: string): string {
  const name = getFileNameFromPath(filePath);
  return duplicateNames.value.has(name) ? filePath : name;
}

async function initEditorIfNeeded() {
  if (editor.value || !editorContainer.value) return;

  editor.value = monaco.editor.create(editorContainer.value, {
    value: '',
    language: 'plaintext',
    theme: 'vs',
    fontSize: 14,
    lineNumbers: 'on',
    wordWrap: 'on',
    minimap: { enabled: true },
    scrollBeyondLastLine: false,
    automaticLayout: true,
    formatOnPaste: true,
    formatOnType: true,
  });

  editor.value.onDidChangeModelContent(() => {
    if (fileStore.activeFile) {
      const content = editor.value?.getValue() || '';
      fileStore.updateFileContent(content);
    }
  });
}

// Initialize Monaco Editor & react to active file changes
onMounted(() => {
  watch(
    () => fileStore.activeFile,
    async (newFile) => {
      if (newFile) {
        // 确保容器已渲染再创建编辑器
        await nextTick();
        await initEditorIfNeeded();

        if (editor.value) {
          const model = editor.value.getModel();
          if (model) {
            model.setValue(newFile.content);
            monaco.editor.setModelLanguage(model, newFile.language || 'plaintext');
          }
        }
      } else if (editor.value) {
        editor.value.setValue('');
      }
    },
    { immediate: true }
  );

  // 每 10 秒轮询当前活动文件是否在磁盘上发生变化
  // pollTimer = window.setInterval(async () => {
  //   const active = fileStore.activeFile;
  //   if (!active || active.modified) {
  //     return;
  //   }

  //   try {
  //     const latest = (await invoke('read_file', { path: active.path })) as string;
  //     if (typeof latest === 'string' && latest !== active.content) {
  //       // 更新编辑器内容与 store，但不标记为已修改
  //       if (editor.value) {
  //         const model = editor.value.getModel();
  //         if (model) {
  //           model.setValue(latest);
  //         } else {
  //           editor.value.setValue(latest);
  //         }
  //       }
  //       fileStore.refreshActiveFileContentFromDisk(latest);
  //     }
  //   } catch (error) {
  //     // 轮询失败时静默忽略，避免打扰用户
  //     // 可以在需要时添加调试日志
  //   }
  // }, 10_000);
});

// Cleanup on unmount
onUnmounted(() => {
  if (editor.value) {
    editor.value.dispose();
  }

  if (pollTimer !== null) {
    window.clearInterval(pollTimer);
    pollTimer = null;
  }
});

// Save current file
async function saveCurrentFile() {
  if (!fileStore.activeFile) {
    return;
  }

  try {
    isLoading.value = true;
    const content = editor.value?.getValue() || '';
    await fileStore.saveFile(content);
  } catch (error) {
    console.error('Failed to save file:', error);
  } finally {
    isLoading.value = false;
  }
}

// Save all files
async function saveAllFiles() {
  try {
    isLoading.value = true;
    await fileStore.saveAllFiles();
  } catch (error) {
    console.error('Failed to save all files:', error);
  } finally {
    isLoading.value = false;
  }
}

// Close file
function closeFile(index: number) {
  const file = fileStore.openedFiles[index];
  if (!file) {
    return;
  }

  if (file.modified) {
    if (!confirm('文件有未保存的更改，确定要关闭吗？')) {
      return;
    }
  }
  fileStore.closeFile(file.path);
}

// Switch to file
function switchToFile(index: number) {
  fileStore.setActiveFile(index);
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- File Tabs -->
    <div class="border-b border-border bg-surface">
      <ElTabs
        v-model="fileStore.activeFileIndex"
        type="card"
        closable
        @tab-click="(pane: any) => switchToFile(pane.props.name as number)"
        @tab-remove="(name: any) => closeFile(name as number)"
      >
        <ElTabPane
          v-for="(file, index) in fileStore.openedFiles"
          :key="file.path"
          :name="index"
        >
          <template #label>
            <div class="flex items-center">
              <span
                class="mr-2 max-w-[180px] truncate"
                :title="file.path"
              >
                {{ getTabLabel(file.path) }}
              </span>
              <span
                v-if="file.modified"
                class="text-warning"
              >*</span>
            </div>
          </template>
        </ElTabPane>
      </ElTabs>

      <!-- Editor Toolbar -->
      <div class="flex items-center justify-between px-4 py-2 border-t border-border">
        <div class="flex items-center space-x-2">
          <span class="text-sm text-text-secondary">
            {{ fileStore.activeFile?.language || 'plaintext' }}
          </span>
        </div>

        <div class="flex items-center space-x-2">
          <ElTooltip
            content="保存当前文件 (Ctrl+S)"
            placement="bottom"
          >
            <ElButton
              :icon="Document"
              :loading="isLoading"
              size="small"
              @click="saveCurrentFile"
            >
              保存
            </ElButton>
          </ElTooltip>

          <ElTooltip
            content="保存所有文件 (Ctrl+Shift+S)"
            placement="bottom"
          >
            <ElButton
              :icon="FolderOpened"
              :loading="isLoading"
              size="small"
              @click="saveAllFiles"
            >
              全部保存
            </ElButton>
          </ElTooltip>
        </div>
      </div>
    </div>

    <!-- Editor Area -->
    <div class="flex-1 overflow-hidden">
      <div
        v-if="!fileStore.activeFile"
        class="flex flex-col items-center justify-center h-full text-text-secondary"
      >
        <ElIcon
          :size="48"
          class="mb-4"
        >
          <Document />
        </ElIcon>
        <p>打开一个文件开始编辑</p>
      </div>

      <div
        v-else
        ref="editorContainer"
        class="h-full w-full"
      />
    </div>

    <!-- Status Bar -->
    <div class="border-t border-border bg-surface px-4 py-1 text-xs text-text-secondary">
      <div class="flex items-center justify-between">
        <div>
          <span v-if="fileStore.activeFile">
            行: {{ editor?.getModel()?.getLineCount() || 0 }} | 列:
            {{ editor?.getPosition()?.column || 1 }}
          </span>
        </div>

        <div>
          <span
            v-if="fileStore.activeFile?.modified"
            class="text-warning"
          > 有未保存的更改 </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.el-tabs__header) {
  margin: 0;
}

:deep(.el-tabs__nav-wrap) {
  padding: 0 16px;
}

:deep(.el-tabs__item) {
  padding: 0 12px;
  height: 32px;
  line-height: 32px;
}
</style>
