<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ElNotification } from 'element-plus'
import { useAppStore } from './stores/app'

const appStore = useAppStore()
const isLoading = ref(true)

onMounted(async () => {
  try {
    // Initialize application
    await appStore.initialize()

    // Listen to Tauri events
    await setupEventListeners()

    ElNotification({
      title: '应用启动成功',
      message: 'Code AI Assistant 已准备就绪',
      type: 'success',
      duration: 3000,
    })
  } catch (error) {
    console.error('Failed to initialize app:', error)
    ElNotification({
      title: '应用启动失败',
      message: error instanceof Error ? error.message : '未知错误',
      type: 'error',
      duration: 5000,
    })
  } finally {
    // Always show the layout even if initialization fails
    isLoading.value = false
  }
})

async function setupEventListeners() {
  // Listen for file change events
  const unlistenFileChanged = await listen('file-changed', (event) => {
    console.log('File changed:', event.payload)
    // Handle file change event
  })

  // Listen for terminal output events
  const unlistenTerminalOutput = await listen('terminal-output', (event) => {
    console.log('Terminal output:', event.payload)
    // Handle terminal output event
  })

  // Listen for AI response events
  const unlistenAiResponse = await listen('ai-response', (event) => {
    console.log('AI response:', event.payload)
    // Handle AI response event
  })

  // Listen for log message events
  const unlistenLogMessage = await listen('log-message', (event) => {
    console.log('Log message:', event.payload)
    // Handle log message event
  })

  // Listen for workspace changed events
  const unlistenWorkspaceChanged = await listen('workspace-changed', (event) => {
    console.log('Workspace changed:', event.payload)
    appStore.setCurrentWorkspace(event.payload.workspace)
  })

  // Listen for settings updated events
  const unlistenSettingsUpdated = await listen('settings-updated', () => {
    console.log('Settings updated')
    appStore.loadSettings()
  })

  // Listen for app ready events
  const unlistenAppReady = await listen('app-ready', () => {
    console.log('App ready')
  })

  // Listen for error events
  const unlistenError = await listen('error', (event) => {
    console.error('Error event:', event.payload)
    ElNotification({
      title: '错误',
      message: event.payload.error,
      type: 'error',
      duration: 5000,
    })
  })

  // Store unlisten functions for cleanup
  appStore.setUnlistenFunctions([
    unlistenFileChanged,
    unlistenTerminalOutput,
    unlistenAiResponse,
    unlistenLogMessage,
    unlistenWorkspaceChanged,
    unlistenSettingsUpdated,
    unlistenAppReady,
    unlistenError,
  ])
}
</script>

<template>
  <div id="app">
    <div v-if="isLoading" class="flex-col-center h-full w-full">
      <div class="animate-spin rounded-full h-16 w-16 border-t-2 border-b-2 border-primary-500"></div>
      <p class="mt-4 text-text-secondary">正在初始化应用...</p>
    </div>

    <router-view v-else />
  </div>
</template>

<style scoped>
#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style>
