// Application types
export interface Workspace {
  id: string
  name: string
  path: string
  createdAt: string
  updatedAt: string
  settings: Record<string, any>
}

export interface AppSettings {
  theme: Theme
  editor: EditorSettings
  terminal: TerminalSettings
  chat: ChatSettings
  ai: AISettings
  paths: PathSettings
}

export interface EditorSettings {
  fontSize: number
  tabSize: number
  wordWrap: 'off' | 'on' | 'wordWrapColumn' | 'bounded'
  minimap: { enabled: boolean }
  lineNumbers: 'on' | 'off' | 'relative' | 'interval'
  autoSave: boolean
  autoSaveDelay: number
  formatOnSave: boolean
}

export interface TerminalSettings {
  fontSize: number
  fontFamily: string
  cursorStyle: 'block' | 'underline' | 'bar'
  cursorBlink: boolean
}

export interface ChatSettings {
  autoScroll: boolean
  markdownPreview: boolean
  sendWithEnter: boolean
}

export interface AISettings {
  defaultModel: string
  maxTokens: number
  temperature: number
  topP: number
}

export interface PathSettings {
  nodejs: string
  python: string
  git: string
  dataDirectory: string
}

export type Theme = 'light' | 'dark'
export type ColorScheme = 'light' | 'dark' | 'system'

// User types
export interface User {
  id: string
  email: string
  name: string
  avatar?: string
  role?: 'user' | 'admin'
  createdAt: string
  updatedAt: string
  lastLoginAt?: string
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'system'
  language: string
  editor: {
    fontSize: number
    tabSize: number
    wordWrap: 'off' | 'on' | 'wordWrapColumn' | 'bounded'
    minimap: { enabled: boolean }
    lineNumbers: 'on' | 'off' | 'relative' | 'interval'
  }
  terminal: {
    fontSize: number
    fontFamily: string
    cursorStyle: 'block' | 'underline' | 'bar'
  }
  notifications: {
    enabled: boolean
    sound: boolean
    desktop: boolean
  }
  shortcuts: {
    saveFile: string
    saveAllFiles: string
    toggleTerminal: string
    toggleSidebar: string
    findInFiles: string
    formatDocument: string
  }
}

export type AuthState = 'idle' | 'loading' | 'authenticated' | 'error'

// Theme types
export interface ThemeColors {
  primary: string
  secondary: string
  success: string
  warning: string
  danger: string
  background: string
  surface: string
  text: string
  textSecondary: string
}

export interface ThemeColorSet {
  light: ThemeColors
  dark: ThemeColors
}

// File system types
export interface FileEntry {
  name: string
  path: string
  type: 'file' | 'directory'
  size?: number
  modified: string
  created: string
  extension?: string
}

export interface FileContent {
  content: string
  language?: string
  lineCount: number
  size: number
}

// AI types
export interface AIModel {
  id: string
  name: string
  provider: string
  endpoint: string
  apiKey?: string
  maxTokens: number
  temperature: number
  topP: number
  enabled: boolean
}

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: string
  files?: string[]
  model?: string
}

export interface ChatResponse {
  message: ChatMessage
  usage?: {
    promptTokens: number
    completionTokens: number
    totalTokens: number
  }
}

// Terminal types
export interface TerminalSession {
  id: string
  title: string
  cwd: string
  pid?: number
  createdAt: string
  lastActivity: string
}

export interface TerminalOutput {
  sessionId: string
  data: string
  type: 'stdout' | 'stderr' | 'stdin'
  timestamp: string
}

// Command types
export interface CommandResult {
  success: boolean
  output: string
  error?: string
  exitCode?: number
  duration: number
}

// Event types
export interface AppEvent {
  type: string
  data: any
  timestamp: string
}

// Configuration types
export interface AppConfig {
  version: string
  environment: 'development' | 'production' | 'test'
  features: {
    ai: boolean
    terminal: boolean
    fileSystem: boolean
    git: boolean
  }
}

// API response types
export interface ApiResponse<T = any> {
  success: boolean
  data?: T
  error?: string
  message?: string
}

// Utility types
export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>
export type Nullable<T> = T | null
export type Maybe<T> = T | undefined