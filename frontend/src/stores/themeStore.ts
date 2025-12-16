import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'
import type { Theme, ColorScheme, ThemeColors } from '@/utils/types'

export const useThemeStore = defineStore('theme', () => {
  // State
  const currentTheme = ref<Theme>('light')
  const colorScheme = ref<ColorScheme>('system')
  const themeColors = ref<ThemeColors>({
    light: {
      primary: '#2563eb',
      secondary: '#7c3aed',
      success: '#10b981',
      warning: '#f59e0b',
      danger: '#ef4444',
      background: '#ffffff',
      surface: '#f8fafc',
      text: '#1e293b',
      textSecondary: '#64748b'
    },
    dark: {
      primary: '#3b82f6',
      secondary: '#8b5cf6',
      success: '#10b981',
      warning: '#f59e0b',
      danger: '#ef4444',
      background: '#0f172a',
      surface: '#1e293b',
      text: '#f1f5f9',
      textSecondary: '#94a3b8'
    }
  })
  const customThemes = ref<Record<string, ThemeColors>>({})

  // Computed
  const isDarkMode = computed(() => {
    if (colorScheme.value === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches
    }
    return colorScheme.value === 'dark'
  })

  const currentColors = computed(() => {
    return themeColors.value[currentTheme.value]
  })

  const allThemes = computed(() => {
    const themes: Record<string, ThemeColors> = {
      light: themeColors.value.light,
      dark: themeColors.value.dark,
      ...customThemes.value
    }
    return themes
  })

  // Actions
  const setTheme = (theme: Theme) => {
    currentTheme.value = theme
    applyTheme()
    saveToStorage()
  }

  const setColorScheme = (scheme: ColorScheme) => {
    colorScheme.value = scheme
    applyTheme()
    saveToStorage()
  }

  const toggleTheme = () => {
    setTheme(currentTheme.value === 'light' ? 'dark' : 'light')
  }

  const updateThemeColors = (theme: Theme, colors: Partial<ThemeColors[Theme]>) => {
    themeColors.value[theme] = { ...themeColors.value[theme], ...colors }
    if (theme === currentTheme.value) {
      applyTheme()
    }
    saveToStorage()
  }

  const createCustomTheme = (name: string, colors: ThemeColors[Theme]) => {
    customThemes.value[name] = {
      light: { ...themeColors.value.light, ...colors },
      dark: { ...themeColors.value.dark, ...colors }
    }
    saveToStorage()
  }

  const deleteCustomTheme = (name: string) => {
    delete customThemes.value[name]
    saveToStorage()
  }

  const applyTheme = () => {
    const themeToApply = colorScheme.value === 'system'
      ? (isDarkMode.value ? 'dark' : 'light')
      : colorScheme.value

    document.documentElement.classList.toggle('dark', themeToApply === 'dark')

    const colors = themeColors.value[themeToApply]
    setCSSVariables(colors)
  }

  const setCSSVariables = (colors: ThemeColors[Theme]) => {
    const root = document.documentElement
    Object.entries(colors).forEach(([key, value]) => {
      root.style.setProperty(`--color-${key}`, value)
    })
  }

  const saveToStorage = () => {
    localStorage.setItem('theme', currentTheme.value)
    localStorage.setItem('colorScheme', colorScheme.value)
    localStorage.setItem('themeColors', JSON.stringify(themeColors.value))
    localStorage.setItem('customThemes', JSON.stringify(customThemes.value))
  }

  const loadFromStorage = () => {
    const savedTheme = localStorage.getItem('theme') as Theme
    const savedColorScheme = localStorage.getItem('colorScheme') as ColorScheme
    const savedThemeColors = localStorage.getItem('themeColors')
    const savedCustomThemes = localStorage.getItem('customThemes')

    if (savedTheme) currentTheme.value = savedTheme
    if (savedColorScheme) colorScheme.value = savedColorScheme
    if (savedThemeColors) {
      try {
        themeColors.value = { ...themeColors.value, ...JSON.parse(savedThemeColors) }
      } catch (e) {
        console.error('Failed to parse theme colors:', e)
      }
    }
    if (savedCustomThemes) {
      try {
        customThemes.value = JSON.parse(savedCustomThemes)
      } catch (e) {
        console.error('Failed to parse custom themes:', e)
      }
    }

    applyTheme()
  }

  const resetToDefaults = () => {
    currentTheme.value = 'light'
    colorScheme.value = 'system'
    themeColors.value = {
      light: {
        primary: '#2563eb',
        secondary: '#7c3aed',
        success: '#10b981',
        warning: '#f59e0b',
        danger: '#ef4444',
        background: '#ffffff',
        surface: '#f8fafc',
        text: '#1e293b',
        textSecondary: '#64748b'
      },
      dark: {
        primary: '#3b82f6',
        secondary: '#8b5cf6',
        success: '#10b981',
        warning: '#f59e0b',
        danger: '#ef4444',
        background: '#0f172a',
        surface: '#1e293b',
        text: '#f1f5f9',
        textSecondary: '#94a3b8'
      }
    }
    customThemes.value = {}
    applyTheme()
    saveToStorage()
  }

  // Watch for system color scheme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  const handleSystemThemeChange = () => {
    if (colorScheme.value === 'system') {
      applyTheme()
    }
  }

  // Initialize
  const initialize = () => {
    loadFromStorage()
    mediaQuery.addEventListener('change', handleSystemThemeChange)
  }

  // Cleanup
  const cleanup = () => {
    mediaQuery.removeEventListener('change', handleSystemThemeChange)
  }

  return {
    // State
    currentTheme,
    colorScheme,
    themeColors,
    customThemes,

    // Computed
    isDarkMode,
    currentColors,
    allThemes,

    // Actions
    setTheme,
    setColorScheme,
    toggleTheme,
    updateThemeColors,
    createCustomTheme,
    deleteCustomTheme,
    applyTheme,
    resetToDefaults,
    initialize,
    cleanup
  }
})