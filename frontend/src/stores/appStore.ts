import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Workspace, AppSettings, Theme, EditorSettings } from '@/utils/types'

export const useAppStore = defineStore('app', () => {
  // State
  const currentWorkspace = ref<Workspace | null>(null)
  const workspaces = ref<Workspace[]>([])
  const settings = ref<AppSettings>({
    theme: 'light',
    editor: {
      fontSize: 14,
      tabSize: 2,
      wordWrap: 'off',
      minimap: { enabled: true },
      lineNumbers: 'on',
      autoSave: true,
      autoSaveDelay: 1000,
      formatOnSave: true
    },
    terminal: {
      fontSize: 14,
      fontFamily: 'Consolas, Monaco, "Courier New", monospace',
      cursorStyle: 'block',
      cursorBlink: true
    },
    chat: {
      autoScroll: true,
      markdownPreview: true,
      sendWithEnter: true
    },
    ai: {
      defaultModel: 'claude-3-5-sonnet',
      maxTokens: 4096,
      temperature: 0.7,
      topP: 0.9
    },
    paths: {
      nodejs: '',
      python: '',
      git: '',
      dataDirectory: ''
    }
  })
  const theme = ref<Theme>('light')
  const isDarkMode = computed(() => theme.value === 'dark')
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const notifications = ref<Notification[]>([])

  // Actions
  const setTheme = (newTheme: Theme) => {
    theme.value = newTheme
    settings.value.theme = newTheme
    document.documentElement.classList.toggle('dark', newTheme === 'dark')
    // Save to localStorage
    localStorage.setItem('theme', newTheme)
  }

  const toggleTheme = () => {
    setTheme(theme.value === 'light' ? 'dark' : 'light')
  }

  const setWorkspace = (workspace: Workspace) => {
    currentWorkspace.value = workspace
    // Save to localStorage
    localStorage.setItem('currentWorkspace', JSON.stringify(workspace))
  }

  const createWorkspace = (name: string, path: string): Workspace => {
    const newWorkspace: Workspace = {
      id: crypto.randomUUID(),
      name,
      path,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      settings: {}
    }
    workspaces.value.push(newWorkspace)
    return newWorkspace
  }

  const deleteWorkspace = (workspaceId: string) => {
    const index = workspaces.value.findIndex(w => w.id === workspaceId)
    if (index !== -1) {
      workspaces.value.splice(index, 1)
      if (currentWorkspace.value?.id === workspaceId) {
        currentWorkspace.value = workspaces.value[0] || null
      }
    }
  }

  const updateSettings = (newSettings: Partial<AppSettings>) => {
    settings.value = { ...settings.value, ...newSettings }
    // Save to localStorage
    localStorage.setItem('appSettings', JSON.stringify(settings.value))
  }

  const updateEditorSettings = (editorSettings: Partial<EditorSettings>) => {
    settings.value.editor = { ...settings.value.editor, ...editorSettings }
    updateSettings({ editor: settings.value.editor })
  }

  const setLoading = (loading: boolean) => {
    isLoading.value = loading
  }

  const setError = (message: string | null) => {
    error.value = message
  }

  const addNotification = (notification: Omit<Notification, 'id' | 'timestamp'>) => {
    const newNotification: Notification = {
      ...notification,
      id: crypto.randomUUID(),
      timestamp: new Date().toISOString()
    }
    notifications.value.push(newNotification)

    // Auto remove after 5 seconds
    setTimeout(() => {
      removeNotification(newNotification.id)
    }, 5000)
  }

  const removeNotification = (notificationId: string) => {
    const index = notifications.value.findIndex(n => n.id === notificationId)
    if (index !== -1) {
      notifications.value.splice(index, 1)
    }
  }

  const clearNotifications = () => {
    notifications.value = []
  }

  // Initialize
  const initialize = () => {
    // Load theme from localStorage
    const savedTheme = localStorage.getItem('theme') as Theme
    if (savedTheme) {
      setTheme(savedTheme)
    }

    // Load settings from localStorage
    const savedSettings = localStorage.getItem('appSettings')
    if (savedSettings) {
      try {
        settings.value = { ...settings.value, ...JSON.parse(savedSettings) }
      } catch (e) {
        console.error('Failed to parse saved settings:', e)
      }
    }

    // Load workspaces from localStorage
    const savedWorkspaces = localStorage.getItem('workspaces')
    if (savedWorkspaces) {
      try {
        workspaces.value = JSON.parse(savedWorkspaces)
      } catch (e) {
        console.error('Failed to parse saved workspaces:', e)
      }
    }

    // Load current workspace from localStorage
    const savedCurrentWorkspace = localStorage.getItem('currentWorkspace')
    if (savedCurrentWorkspace) {
      try {
        currentWorkspace.value = JSON.parse(savedCurrentWorkspace)
      } catch (e) {
        console.error('Failed to parse current workspace:', e)
      }
    }
  }

  // Save state
  const saveState = () => {
    localStorage.setItem('workspaces', JSON.stringify(workspaces.value))
    localStorage.setItem('currentWorkspace', JSON.stringify(currentWorkspace.value))
    localStorage.setItem('appSettings', JSON.stringify(settings.value))
  }

  // Reset to defaults
  const resetToDefaults = () => {
    settings.value = {
      theme: 'light',
      editor: {
        fontSize: 14,
        tabSize: 2,
        wordWrap: 'off',
        minimap: { enabled: true },
        lineNumbers: 'on',
        autoSave: true,
        autoSaveDelay: 1000,
        formatOnSave: true
      },
      terminal: {
        fontSize: 14,
        fontFamily: 'Consolas, Monaco, "Courier New", monospace',
        cursorStyle: 'block',
        cursorBlink: true
      },
      chat: {
        autoScroll: true,
        markdownPreview: true,
        sendWithEnter: true
      },
      ai: {
        defaultModel: 'claude-3-5-sonnet',
        maxTokens: 4096,
        temperature: 0.7,
        topP: 0.9
      },
      paths: {
        nodejs: '',
        python: '',
        git: '',
        dataDirectory: ''
      }
    }
    theme.value = 'light'
    document.documentElement.classList.remove('dark')
    saveState()
  }

  return {
    // State
    currentWorkspace,
    workspaces,
    settings,
    theme,
    isDarkMode,
    isLoading,
    error,
    notifications,

    // Actions
    setTheme,
    toggleTheme,
    setWorkspace,
    createWorkspace,
    deleteWorkspace,
    updateSettings,
    updateEditorSettings,
    setLoading,
    setError,
    addNotification,
    removeNotification,
    clearNotifications,
    initialize,
    saveState,
    resetToDefaults
  }
})

// Types
interface Notification {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message: string
  timestamp: string
}