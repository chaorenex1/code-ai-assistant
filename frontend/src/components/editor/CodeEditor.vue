<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import * as monaco from 'monaco-editor'
import { Document, FolderOpened } from '@element-plus/icons-vue'
import { ElTabs, ElTabPane, ElButton, ElIcon, ElTooltip } from 'element-plus'
import { useFileStore } from '../../stores/modules/files'

const fileStore = useFileStore()
const editorContainer = ref<HTMLElement>()
const editor = ref<monaco.editor.IStandaloneCodeEditor>()
const isLoading = ref(false)

// Initialize Monaco Editor
onMounted(async () => {
  await nextTick()

  if (editorContainer.value) {
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
    })

    // Listen to content changes
    editor.value.onDidChangeModelContent(() => {
      if (fileStore.activeFile) {
        const content = editor.value?.getValue() || ''
        fileStore.updateFileContent(content)
      }
    })
  }

  // Watch for active file changes
  watch(() => fileStore.activeFile, (newFile) => {
    if (newFile && editor.value) {
      const model = editor.value.getModel()
      if (model) {
        model.setValue(newFile.content)
        monaco.editor.setModelLanguage(model, newFile.language || 'plaintext')
      }
    } else if (editor.value) {
      editor.value.setValue('')
    }
  }, { immediate: true })
})

// Cleanup on unmount
onUnmounted(() => {
  if (editor.value) {
    editor.value.dispose()
  }
})

// Save current file
async function saveCurrentFile() {
  if (!fileStore.activeFile) return

  try {
    isLoading.value = true
    const content = editor.value?.getValue() || ''
    await fileStore.saveFile(content)
  } catch (error) {
    console.error('Failed to save file:', error)
  } finally {
    isLoading.value = false
  }
}

// Save all files
async function saveAllFiles() {
  try {
    isLoading.value = true
    await fileStore.saveAllFiles()
  } catch (error) {
    console.error('Failed to save all files:', error)
  } finally {
    isLoading.value = false
  }
}

// Close file
function closeFile(index: number) {
  const file = fileStore.openedFiles[index]
  if (!file) return
  
  if (file.modified) {
    if (!confirm('文件有未保存的更改，确定要关闭吗？')) {
      return
    }
  }
  fileStore.closeFile(file.path)
}

// Switch to file
function switchToFile(index: number) {
  fileStore.setActiveFile(index)
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
              <span class="mr-2">{{ file.path.split('/').pop() }}</span>
              <span v-if="file.modified" class="text-warning">*</span>
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
          <ElTooltip content="保存当前文件 (Ctrl+S)" placement="bottom">
            <ElButton
              :icon="Document"
              :loading="isLoading"
              size="small"
              @click="saveCurrentFile"
            >
              保存
            </ElButton>
          </ElTooltip>

          <ElTooltip content="保存所有文件 (Ctrl+Shift+S)" placement="bottom">
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
      <div v-if="!fileStore.activeFile" class="flex-col-center h-full text-text-secondary">
        <ElIcon :size="48" class="mb-4"><Document /></ElIcon>
        <p>打开一个文件开始编辑</p>
      </div>

      <div v-else ref="editorContainer" class="h-full w-full"></div>
    </div>

    <!-- Status Bar -->
    <div class="border-t border-border bg-surface px-4 py-1 text-xs text-text-secondary">
      <div class="flex items-center justify-between">
        <div>
          <span v-if="fileStore.activeFile">
            行: {{ editor?.getModel()?.getLineCount() || 0 }} |
            列: {{ editor?.getPosition()?.column || 1 }}
          </span>
        </div>

        <div>
          <span v-if="fileStore.activeFile?.modified" class="text-warning">
            有未保存的更改
          </span>
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