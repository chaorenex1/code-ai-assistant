import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { UnlistenFn } from '@tauri-apps/api/event'

export interface AppSettings {
  workspace: string
  theme: 'light' | 'dark'
  fontSize: number
  autoSave: boolean
  aiModel: string
  terminalShell: string
}

export interface Workspace {
  id: string
  name: string
  path: string
  createdAt: string
  updatedAt: string
}

export const useAppStore = defineStore('app', () => {
  // State
  const currentWorkspace = ref('default')
  const currentFile = ref<string | null>(null)
  const currentAiModel = ref('claude-3-5-sonnet')
  const isConnected = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const settings = ref<AppSettings>({
    workspace: 'default',
    theme: 'light',
    fontSize: 14,
    autoSave: true,
    aiModel: 'claude-3-5-sonnet',
    terminalShell: 'bash',
  })
  const workspaces = ref<Workspace[]>([])
  const unlistenFunctions = ref<UnlistenFn[]>([])

  // Getters
  const themeClass = computed(() => settings.value.theme)
  const fontSizeStyle = computed(() => `${settings.value.fontSize}px`)
  const availableAiModels = computed(() => [
    'claude-3-5-sonnet',
    'gpt-4',
    'gpt-3.5-turbo',
    'gemini-pro',
  ])

  // Actions
  async function initialize() {
    isLoading.value = true
    error.value = null

    try {
      // Load settings from backend
      try {
        const backendSettings = await invoke('get_settings')
        if (backendSettings) {
          settings.value = {
            ...settings.value,
            ...backendSettings,
          }
        }
      } catch (err) {
        console.warn('Failed to load settings from backend, using defaults:', err)
      }

      // Load workspaces
      try {
        const workspaceList = await invoke('get_workspaces')
        workspaces.value = workspaceList as Workspace[]
      } catch (err) {
        console.warn('Failed to load workspaces from backend:', err)
      }

      // Load AI models
      try {
        const aiModels = await invoke('get_ai_models')
        if (aiModels && Array.isArray(aiModels) && aiModels.length > 0) {
          currentAiModel.value = aiModels[0] as string
        }
      } catch (err) {
        console.warn('Failed to load AI models from backend:', err)
      }

      isConnected.value = true
    } catch (err) {
      error.value = err instanceof Error ? err.message : '初始化失败'
      isConnected.value = false
      console.error('Failed to initialize app:', err)
    } finally {
      isLoading.value = false
    }
  }

  async function loadSettings() {
    try {
      const backendSettings = await invoke('get_settings')
      if (backendSettings) {
        settings.value = {
          ...settings.value,
          ...backendSettings,
        }
      }
    } catch (err) {
      console.error('Failed to load settings:', err)
    }
  }

  async function saveSettings() {
    try {
      await invoke('save_settings', { config: settings.value })
    } catch (err) {
      console.error('Failed to save settings:', err)
      throw err
    }
  }

  async function switchWorkspace(workspaceName: string) {
    try {
      await invoke('switch_workspace', { name: workspaceName })
      currentWorkspace.value = workspaceName
      settings.value.workspace = workspaceName
      await saveSettings()
    } catch (err) {
      console.error('Failed to switch workspace:', err)
      throw err
    }
  }

  async function createWorkspace(name: string, path?: string) {
    try {
      await invoke('create_workspace', { name })

      const newWorkspace: Workspace = {
        id: Date.now().toString(),
        name,
        path: path || `./workspaces/${name}`,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      }

      workspaces.value.push(newWorkspace)
      return newWorkspace
    } catch (err) {
      console.error('Failed to create workspace:', err)
      throw err
    }
  }

  async function deleteWorkspace(name: string) {
    try {
      await invoke('delete_workspace', { name })
      workspaces.value = workspaces.value.filter(w => w.name !== name)

      if (currentWorkspace.value === name) {
        currentWorkspace.value = 'default'
        settings.value.workspace = 'default'
        await saveSettings()
      }
    } catch (err) {
      console.error('Failed to delete workspace:', err)
      throw err
    }
  }

  function setCurrentFile(filePath: string | null) {
    currentFile.value = filePath
  }

  function setCurrentAiModel(model: string) {
    currentAiModel.value = model
    settings.value.aiModel = model
    saveSettings()
  }

  function setTheme(theme: 'light' | 'dark') {
    settings.value.theme = theme
    saveSettings()
  }

  function setFontSize(size: number) {
    settings.value.fontSize = size
    saveSettings()
  }

  function setAutoSave(enabled: boolean) {
    settings.value.autoSave = enabled
    saveSettings()
  }

  function setTerminalShell(shell: string) {
    settings.value.terminalShell = shell
    saveSettings()
  }

  function setCurrentWorkspace(workspace: string) {
    currentWorkspace.value = workspace
    settings.value.workspace = workspace
    saveSettings()
  }

  function setUnlistenFunctions(functions: UnlistenFn[]) {
    unlistenFunctions.value = functions
  }

  function clearError() {
    error.value = null
  }

  // Cleanup
  function cleanup() {
    // Unlisten all events
    unlistenFunctions.value.forEach(unlisten => unlisten())
    unlistenFunctions.value = []
  }

  return {
    // State
    currentWorkspace,
    currentFile,
    currentAiModel,
    isConnected,
    isLoading,
    error,
    settings,
    workspaces,

    // Getters
    themeClass,
    fontSizeStyle,
    availableAiModels,

    // Actions
    initialize,
    loadSettings,
    saveSettings,
    switchWorkspace,
    createWorkspace,
    deleteWorkspace,
    setCurrentFile,
    setCurrentAiModel,
    setTheme,
    setFontSize,
    setAutoSave,
    setTerminalShell,
    setCurrentWorkspace,
    setUnlistenFunctions,
    clearError,
    cleanup,
  }
})