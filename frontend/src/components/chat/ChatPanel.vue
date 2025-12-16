<script setup lang="ts">
import { ref, computed } from 'vue'
import { Link, Delete, Setting } from '@element-plus/icons-vue'
import { ElInput, ElButton, ElSelect, ElOption, ElTooltip, ElTag } from 'element-plus'
import { useAppStore } from '../../stores/app'
import { useFileStore } from '../../stores/modules/files'

const appStore = useAppStore()
const fileStore = useFileStore()

const message = ref('')
const isLoading = ref(false)
const associatedFiles = ref<string[]>([])

// Send message
async function sendMessage() {
  if (!message.value.trim() || isLoading.value) return

  try {
    isLoading.value = true

    // TODO: Send to AI backend
    console.log('Sending message:', message.value)
    console.log('Associated files:', associatedFiles.value)

    // Clear message
    message.value = ''
  } catch (error) {
    console.error('Failed to send message:', error)
  } finally {
    isLoading.value = false
  }
}

// Clear chat
function clearChat() {
  if (confirm('确定要清空聊天记录吗？')) {
    // TODO: Clear chat history
  }
}

// Associate file
function associateFile() {
  if (fileStore.activeFile) {
    const filePath = fileStore.activeFile.path
    if (!associatedFiles.value.includes(filePath)) {
      associatedFiles.value.push(filePath)
    }
  }
}

// Remove associated file
function removeAssociatedFile(index: number) {
  associatedFiles.value.splice(index, 1)
}

// Handle key press
function handleKeyPress(event: KeyboardEvent | Event) {
  if (event instanceof KeyboardEvent && event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault()
    sendMessage()
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Associated Files -->
    <div v-if="associatedFiles.length > 0" class="border-b border-border bg-surface p-2">
      <div class="flex items-center justify-between mb-1">
        <div class="flex items-center text-sm text-text-secondary">
          <ElIcon :size="14" class="mr-1"><Link /></ElIcon>
          关联文件:
        </div>
        <ElButton size="small" text @click="associateFile">
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
          {{ file.split('/').pop() }}
        </ElTag>
      </div>
    </div>

    <!-- Chat Messages Area -->
    <div class="flex-1 overflow-auto p-4">
      <div class="text-center text-text-secondary">
        聊天功能开发中...
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
            <ElSelect size="small" style="width: 140px">
              <ElOption label="OpenAI Codex" value="codex" />
              <ElOption label="Local Coder" value="local" />
            </ElSelect>
          </div>
        </div>

        <div class="flex items-center space-x-2">
          <ElTooltip content="关联当前文件" placement="bottom">
            <ElButton
              :icon="Link"
              size="small"
              text
              @click="associateFile"
            />
          </ElTooltip>

          <ElTooltip content="清空聊天" placement="bottom">
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
          @keydown="handleKeyPress"
          class="flex-1"
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
</template>

<style scoped>
:deep(.el-textarea__inner) {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  line-height: 1.5;
}

:deep(.el-select) {
  width: auto;
}
</style>