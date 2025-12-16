// Application constants
export const APP_NAME = 'Code AI Assistant'
export const APP_VERSION = '0.1.0'
export const APP_DESCRIPTION = 'A desktop application with code editor, AI chat assistant, CLI output and terminal functionality'

// File system constants
export const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10MB
export const SUPPORTED_FILE_EXTENSIONS = [
  // Programming languages
  '.js', '.ts', '.jsx', '.tsx', '.py', '.java', '.cpp', '.c', '.cs', '.go', '.rs', '.rb', '.php',
  '.swift', '.kt', '.scala', '.pl', '.lua', '.r', '.sh', '.bash', '.zsh', '.fish',

  // Web technologies
  '.html', '.htm', '.css', '.scss', '.sass', '.less', '.json', '.xml', '.yaml', '.yml', '.toml',
  '.md', '.markdown',

  // Data formats
  '.csv', '.tsv', '.sql', '.db', '.sqlite',

  // Configuration
  '.env', '.config', '.conf', '.ini',

  // Documents
  '.txt', '.pdf', '.doc', '.docx', '.xls', '.xlsx', '.ppt', '.pptx',

  // Images
  '.png', '.jpg', '.jpeg', '.gif', '.svg', '.ico', '.bmp',

  // Archives
  '.zip', '.tar', '.gz', '.7z', '.rar'
]

export const DEFAULT_FILE_ENCODING = 'utf-8'
export const AUTO_SAVE_INTERVAL = 5000 // 5 seconds

// Editor constants
export const EDITOR_DEFAULT_FONT_SIZE = 14
export const EDITOR_MIN_FONT_SIZE = 8
export const EDITOR_MAX_FONT_SIZE = 32
export const EDITOR_DEFAULT_TAB_SIZE = 2
export const EDITOR_MIN_TAB_SIZE = 1
export const EDITOR_MAX_TAB_SIZE = 8
export const EDITOR_DEFAULT_WORD_WRAP = 'off'
export const EDITOR_DEFAULT_LINE_NUMBERS = 'on'

// Terminal constants
export const TERMINAL_DEFAULT_FONT_SIZE = 14
export const TERMINAL_MIN_FONT_SIZE = 8
export const TERMINAL_MAX_FONT_SIZE = 32
export const TERMINAL_DEFAULT_FONT_FAMILY = 'Consolas, Monaco, "Courier New", monospace'
export const TERMINAL_DEFAULT_CURSOR_STYLE = 'block'
export const TERMINAL_DEFAULT_CURSOR_BLINK = true
export const TERMINAL_MAX_HISTORY_LINES = 10000
export const TERMINAL_MAX_SESSIONS = 10

// AI constants
export const AI_DEFAULT_MODEL = 'claude-3-5-sonnet'
export const AI_MAX_TOKENS = 4096
export const AI_MIN_TOKENS = 1
export const AI_MAX_TOKENS_LIMIT = 16384
export const AI_DEFAULT_TEMPERATURE = 0.7
export const AI_MIN_TEMPERATURE = 0.0
export const AI_MAX_TEMPERATURE = 2.0
export const AI_DEFAULT_TOP_P = 0.9
export const AI_MIN_TOP_P = 0.0
export const AI_MAX_TOP_P = 1.0
export const AI_MAX_CONTEXT_LENGTH = 128000

export const AI_MODELS = [
  {
    id: 'claude-3-5-sonnet',
    name: 'Claude 3.5 Sonnet',
    provider: 'Anthropic',
    endpoint: 'https://api.anthropic.com/v1',
    maxTokens: 4096,
    temperature: 0.7,
    topP: 0.9,
    enabled: true
  },
  {
    id: 'gpt-4',
    name: 'GPT-4',
    provider: 'OpenAI',
    endpoint: 'https://api.openai.com/v1',
    maxTokens: 8192,
    temperature: 0.7,
    topP: 0.9,
    enabled: true
  },
  {
    id: 'gemini-pro',
    name: 'Gemini Pro',
    provider: 'Google',
    endpoint: 'https://generativelanguage.googleapis.com/v1',
    maxTokens: 2048,
    temperature: 0.7,
    topP: 0.9,
    enabled: true
  }
]

// Chat constants
export const CHAT_MAX_MESSAGE_LENGTH = 10000
export const CHAT_MAX_FILES_PER_MESSAGE = 10
export const CHAT_HISTORY_LIMIT = 100
export const CHAT_AUTO_SCROLL_THRESHOLD = 100
export const CHAT_TYPING_INDICATOR_DELAY = 1000 // 1 second

// Workspace constants
export const WORKSPACE_MAX_NAME_LENGTH = 100
export const WORKSPACE_MIN_NAME_LENGTH = 1
export const WORKSPACE_MAX_PATH_LENGTH = 500
export const WORKSPACE_MAX_COUNT = 50
export const WORKSPACE_DEFAULT_SETTINGS = {
  theme: 'light',
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
  }
}

// Settings constants
export const SETTINGS_STORAGE_KEY = 'app_settings'
export const SETTINGS_DEFAULT_THEME = 'light'
export const SETTINGS_DEFAULT_LANGUAGE = 'en'
export const SETTINGS_DEFAULT_AUTO_SAVE = true
export const SETTINGS_DEFAULT_AUTO_SAVE_DELAY = 1000
export const SETTINGS_DEFAULT_FORMAT_ON_SAVE = true

// Path constants
export const DEFAULT_DATA_DIRECTORY = '.code-ai-assistant'
export const DEFAULT_CONFIG_FILE = 'config.json'
export const DEFAULT_LOG_FILE = 'app.log'
export const DEFAULT_DATABASE_FILE = 'database.sqlite'

// Keyboard shortcuts
export const KEYBOARD_SHORTCUTS = {
  SAVE_FILE: 'Ctrl+S',
  SAVE_ALL_FILES: 'Ctrl+Shift+S',
  NEW_FILE: 'Ctrl+N',
  OPEN_FILE: 'Ctrl+O',
  CLOSE_FILE: 'Ctrl+W',
  CLOSE_ALL_FILES: 'Ctrl+Shift+W',
  UNDO: 'Ctrl+Z',
  REDO: 'Ctrl+Y',
  CUT: 'Ctrl+X',
  COPY: 'Ctrl+C',
  PASTE: 'Ctrl+V',
  SELECT_ALL: 'Ctrl+A',
  FIND: 'Ctrl+F',
  REPLACE: 'Ctrl+H',
  FIND_IN_FILES: 'Ctrl+Shift+F',
  TOGGLE_TERMINAL: 'Ctrl+`',
  TOGGLE_SIDEBAR: 'Ctrl+B',
  TOGGLE_SETTINGS: 'Ctrl+,',
  FORMAT_DOCUMENT: 'Shift+Alt+F',
  COMMENT_LINE: 'Ctrl+/',
  INCREASE_FONT_SIZE: 'Ctrl+=',
  DECREASE_FONT_SIZE: 'Ctrl+-',
  RESET_FONT_SIZE: 'Ctrl+0',
  ZOOM_IN: 'Ctrl+Shift+=',
  ZOOM_OUT: 'Ctrl+Shift+-',
  RESET_ZOOM: 'Ctrl+Shift+0'
}

// Event constants
export const EVENT_TYPES = {
  FILE_CREATED: 'file:created',
  FILE_MODIFIED: 'file:modified',
  FILE_DELETED: 'file:deleted',
  FILE_RENAMED: 'file:renamed',
  DIRECTORY_CREATED: 'directory:created',
  DIRECTORY_DELETED: 'directory:deleted',
  TERMINAL_OUTPUT: 'terminal:output',
  TERMINAL_SESSION_STARTED: 'terminal:session:started',
  TERMINAL_SESSION_ENDED: 'terminal:session:ended',
  CHAT_MESSAGE_RECEIVED: 'chat:message:received',
  CHAT_MESSAGE_SENT: 'chat:message:sent',
  SETTINGS_UPDATED: 'settings:updated',
  WORKSPACE_CHANGED: 'workspace:changed',
  APP_ERROR: 'app:error',
  APP_WARNING: 'app:warning',
  APP_INFO: 'app:info',
  APP_STATUS: 'app:status'
}

// Error messages
export const ERROR_MESSAGES = {
  FILE_NOT_FOUND: 'File not found',
  FILE_READ_ERROR: 'Failed to read file',
  FILE_WRITE_ERROR: 'Failed to write file',
  FILE_DELETE_ERROR: 'Failed to delete file',
  FILE_TOO_LARGE: 'File is too large',
  FILE_UNSUPPORTED_FORMAT: 'Unsupported file format',
  DIRECTORY_NOT_FOUND: 'Directory not found',
  DIRECTORY_CREATE_ERROR: 'Failed to create directory',
  DIRECTORY_DELETE_ERROR: 'Failed to delete directory',
  AI_SERVICE_ERROR: 'AI service error',
  AI_MODEL_NOT_FOUND: 'AI model not found',
  AI_API_KEY_MISSING: 'API key is missing',
  AI_RATE_LIMIT_EXCEEDED: 'Rate limit exceeded',
  TERMINAL_SPAWN_ERROR: 'Failed to spawn terminal',
  TERMINAL_KILL_ERROR: 'Failed to kill terminal',
  TERMINAL_COMMAND_ERROR: 'Command execution failed',
  SETTINGS_LOAD_ERROR: 'Failed to load settings',
  SETTINGS_SAVE_ERROR: 'Failed to save settings',
  WORKSPACE_CREATE_ERROR: 'Failed to create workspace',
  WORKSPACE_SWITCH_ERROR: 'Failed to switch workspace',
  WORKSPACE_DELETE_ERROR: 'Failed to delete workspace',
  NETWORK_ERROR: 'Network error',
  PERMISSION_ERROR: 'Permission denied',
  VALIDATION_ERROR: 'Validation error',
  UNKNOWN_ERROR: 'An unknown error occurred'
}

// Success messages
export const SUCCESS_MESSAGES = {
  FILE_SAVED: 'File saved successfully',
  FILE_DELETED: 'File deleted successfully',
  DIRECTORY_CREATED: 'Directory created successfully',
  DIRECTORY_DELETED: 'Directory deleted successfully',
  AI_MESSAGE_SENT: 'Message sent successfully',
  AI_MODEL_CHANGED: 'AI model changed successfully',
  TERMINAL_SPAWNED: 'Terminal spawned successfully',
  TERMINAL_KILLED: 'Terminal killed successfully',
  SETTINGS_SAVED: 'Settings saved successfully',
  SETTINGS_RESET: 'Settings reset to defaults',
  WORKSPACE_CREATED: 'Workspace created successfully',
  WORKSPACE_SWITCHED: 'Workspace switched successfully',
  WORKSPACE_DELETED: 'Workspace deleted successfully'
}

// Validation patterns
export const VALIDATION_PATTERNS = {
  EMAIL: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  URL: /^(https?:\/\/)?([\da-z.-]+)\.([a-z.]{2,6})([/\w .-]*)*\/?$/,
  FILENAME: /^[^\\/:*?"<>|]+$/,
  PATH: /^[^\\:*?"<>|]+$/,
  HEX_COLOR: /^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/,
  UUID: /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i
}

// Time constants
export const TIME = {
  SECOND: 1000,
  MINUTE: 60 * 1000,
  HOUR: 60 * 60 * 1000,
  DAY: 24 * 60 * 60 * 1000,
  WEEK: 7 * 24 * 60 * 60 * 1000,
  MONTH: 30 * 24 * 60 * 60 * 1000,
  YEAR: 365 * 24 * 60 * 60 * 1000
}

// UI constants
export const UI = {
  TOAST_DURATION: 5000,
  TOAST_MAX_COUNT: 5,
  MODAL_ANIMATION_DURATION: 300,
  SIDEBAR_WIDTH: 250,
  SIDEBAR_MIN_WIDTH: 200,
  SIDEBAR_MAX_WIDTH: 400,
  HEADER_HEIGHT: 48,
  FOOTER_HEIGHT: 32,
  BORDER_RADIUS: 8,
  BOX_SHADOW: '0 1px 3px rgba(0, 0, 0, 0.1)',
  BOX_SHADOW_LG: '0 4px 6px rgba(0, 0, 0, 0.1)',
  BOX_SHADOW_XL: '0 10px 15px rgba(0, 0, 0, 0.1)',
  TRANSITION_DURATION: 200,
  Z_INDEX: {
    DROPDOWN: 1000,
    MODAL: 1050,
    TOOLTIP: 1100,
    NOTIFICATION: 1150
  }
}

// API constants
export const API = {
  BASE_URL: '',
  TIMEOUT: 30000,
  RETRY_COUNT: 3,
  RETRY_DELAY: 1000,
  HEADERS: {
    'Content-Type': 'application/json',
    'Accept': 'application/json'
  }
}

// Local storage keys
export const STORAGE_KEYS = {
  APP_SETTINGS: 'app_settings',
  USER_PREFERENCES: 'user_preferences',
  CURRENT_USER: 'current_user',
  CURRENT_WORKSPACE: 'current_workspace',
  WORKSPACES: 'workspaces',
  CHAT_HISTORY: 'chat_history',
  RECENT_FILES: 'recent_files',
  THEME: 'theme',
  LANGUAGE: 'language',
  SIDEBAR_STATE: 'sidebar_state',
  TERMINAL_HISTORY: 'terminal_history'
}