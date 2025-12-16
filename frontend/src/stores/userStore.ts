import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User, UserPreferences, AuthState } from '@/utils/types'

export const useUserStore = defineStore('user', () => {
  // State
  const currentUser = ref<User | null>(null)
  const isAuthenticated = computed(() => currentUser.value !== null)
  const authState = ref<AuthState>('idle')
  const preferences = ref<UserPreferences>({
    theme: 'system',
    language: 'en',
    editor: {
      fontSize: 14,
      tabSize: 2,
      wordWrap: 'off',
      minimap: { enabled: true },
      lineNumbers: 'on'
    },
    terminal: {
      fontSize: 14,
      fontFamily: 'Consolas, Monaco, "Courier New", monospace',
      cursorStyle: 'block'
    },
    notifications: {
      enabled: true,
      sound: true,
      desktop: true
    },
    shortcuts: {
      saveFile: 'Ctrl+S',
      saveAllFiles: 'Ctrl+Shift+S',
      toggleTerminal: 'Ctrl+`',
      toggleSidebar: 'Ctrl+B',
      findInFiles: 'Ctrl+Shift+F',
      formatDocument: 'Shift+Alt+F'
    }
  })

  // Actions
  const setUser = (user: User | null) => {
    currentUser.value = user
    if (user) {
      localStorage.setItem('currentUser', JSON.stringify(user))
    } else {
      localStorage.removeItem('currentUser')
    }
  }

  const login = async (email: string, password: string): Promise<boolean> => {
    try {
      authState.value = 'loading'
      // TODO: Implement actual login logic with backend
      // For now, simulate login
      await new Promise(resolve => setTimeout(resolve, 1000))

      const user: User = {
        id: crypto.randomUUID(),
        email,
        name: email.split('@')[0],
        avatar: `https://ui-avatars.com/api/?name=${encodeURIComponent(email)}&background=random`,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        lastLoginAt: new Date().toISOString()
      }

      setUser(user)
      authState.value = 'authenticated'
      return true
    } catch (error) {
      console.error('Login failed:', error)
      authState.value = 'error'
      return false
    }
  }

  const logout = () => {
    setUser(null)
    authState.value = 'idle'
  }

  const updateProfile = async (updates: Partial<User>): Promise<boolean> => {
    if (!currentUser.value) return false

    try {
      authState.value = 'loading'
      // TODO: Implement actual update logic with backend
      await new Promise(resolve => setTimeout(resolve, 500))

      currentUser.value = {
        ...currentUser.value,
        ...updates,
        updatedAt: new Date().toISOString()
      }

      setUser(currentUser.value)
      authState.value = 'authenticated'
      return true
    } catch (error) {
      console.error('Profile update failed:', error)
      authState.value = 'error'
      return false
    }
  }

  const updatePreferences = (newPreferences: Partial<UserPreferences>) => {
    preferences.value = { ...preferences.value, ...newPreferences }
    localStorage.setItem('userPreferences', JSON.stringify(preferences.value))
  }

  const updateShortcut = (action: keyof UserPreferences['shortcuts'], shortcut: string) => {
    preferences.value.shortcuts[action] = shortcut
    updatePreferences({ shortcuts: preferences.value.shortcuts })
  }

  const resetPreferences = () => {
    preferences.value = {
      theme: 'system',
      language: 'en',
      editor: {
        fontSize: 14,
        tabSize: 2,
        wordWrap: 'off',
        minimap: { enabled: true },
        lineNumbers: 'on'
      },
      terminal: {
        fontSize: 14,
        fontFamily: 'Consolas, Monaco, "Courier New", monospace',
        cursorStyle: 'block'
      },
      notifications: {
        enabled: true,
        sound: true,
        desktop: true
      },
      shortcuts: {
        saveFile: 'Ctrl+S',
        saveAllFiles: 'Ctrl+Shift+S',
        toggleTerminal: 'Ctrl+`',
        toggleSidebar: 'Ctrl+B',
        findInFiles: 'Ctrl+Shift+F',
        formatDocument: 'Shift+Alt+F'
      }
    }
    localStorage.setItem('userPreferences', JSON.stringify(preferences.value))
  }

  // Initialize
  const initialize = () => {
    // Load user from localStorage
    const savedUser = localStorage.getItem('currentUser')
    if (savedUser) {
      try {
        currentUser.value = JSON.parse(savedUser)
        authState.value = 'authenticated'
      } catch (e) {
        console.error('Failed to parse saved user:', e)
      }
    }

    // Load preferences from localStorage
    const savedPreferences = localStorage.getItem('userPreferences')
    if (savedPreferences) {
      try {
        preferences.value = { ...preferences.value, ...JSON.parse(savedPreferences) }
      } catch (e) {
        console.error('Failed to parse saved preferences:', e)
      }
    }
  }

  // Computed
  const userInitials = computed(() => {
    if (!currentUser.value?.name) return ''
    return currentUser.value.name
      .split(' ')
      .map(word => word[0])
      .join('')
      .toUpperCase()
      .slice(0, 2)
  })

  const userDisplayName = computed(() => {
    return currentUser.value?.name || currentUser.value?.email || 'Guest'
  })

  const isAdmin = computed(() => {
    return currentUser.value?.role === 'admin'
  })

  return {
    // State
    currentUser,
    isAuthenticated,
    authState,
    preferences,

    // Computed
    userInitials,
    userDisplayName,
    isAdmin,

    // Actions
    setUser,
    login,
    logout,
    updateProfile,
    updatePreferences,
    updateShortcut,
    resetPreferences,
    initialize
  }
})