import type { Terminal } from '@xterm/xterm';

// Application types
export interface Workspace {
  id: string;
  name: string;
  path: string;
  currentSessionId: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
  settings: Record<string, any>;
}

export interface AppSettings {
  theme: Theme;
  colorScheme: ColorScheme;
  editor: EditorSettings;
  terminal: TerminalSettings;
  chat: ChatSettings;
  ai: AISettings;
  paths: PathSettings;
  models: AIModel[];
  codeCli: CodeCli[];
  environmentVariables: EnvironmentVariable[];
  userPreferences: UserPreference;
}

export interface EnvironmentVariable {
  name: string;
  value: string;
  isSecret: boolean;
}

export interface AIModel {
  id: string;
  name: string;
  provider: string;
  endpoint: string;
  apiKey?: string;
}

export interface CodeCli {
  name: string;
  command: string;
  args: string;
}

export interface EditorSettings {
  fontSize: number;
  tabSize: number;
  wordWrap: 'off' | 'on' | 'wordWrapColumn' | 'bounded';
  minimap: { enabled: boolean };
  lineNumbers: 'on' | 'off' | 'relative' | 'interval';
  autoSave: boolean;
  autoSaveDelay: number;
  formatOnSave: boolean;
  enableFileWatcher?: boolean;
}

export interface TerminalSettings {
  fontSize: number;
  fontFamily: string;
  cursorStyle: 'block' | 'underline' | 'bar';
  cursorBlink: boolean;
}

export interface ChatSettings {
  autoScroll: boolean;
  markdownPreview: boolean;
  sendWithEnter: boolean;
  switchLineWithShiftEnter: boolean;
}

export interface AISettings {
  defaultModel: string;
  maxTokens: number;
  temperature: number;
  topP: number;
  model_list?: string[];
  code_cli?: string[];
  useDirectCodeCli?: boolean;
}

export interface PathSettings {
  nodejs: string;
  python: string;
  git: string;
}

export type Theme = 'light' | 'dark';
export type ColorScheme = 'light' | 'dark' | 'system';

// User types
export interface User {
  id: string;
  email: string;
  name: string;
  avatar?: string;
  role?: 'user' | 'admin';
  createdAt: string;
  updatedAt: string;
  lastLoginAt?: string;
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'system';
  language: string;
  editor: {
    fontSize: number;
    tabSize: number;
    wordWrap: 'off' | 'on' | 'wordWrapColumn' | 'bounded';
    minimap: { enabled: boolean };
    lineNumbers: 'on' | 'off' | 'relative' | 'interval';
  };
  terminal: {
    fontSize: number;
    fontFamily: string;
    cursorStyle: 'block' | 'underline' | 'bar';
  };
  notifications: {
    enabled: boolean;
    sound: boolean;
    desktop: boolean;
  };
  shortcuts: {
    saveFile: string;
    saveAllFiles: string;
    toggleTerminal: string;
    toggleSidebar: string;
    findInFiles: string;
    formatDocument: string;
  };
}

export interface UserPreference {
  currentModel: string;
  currentCodeCli: string;
  currentShell: string;
}

export type AuthState = 'idle' | 'loading' | 'authenticated' | 'error';

// Theme types
export interface ThemeColors {
  primary: string;
  secondary: string;
  success: string;
  warning: string;
  danger: string;
  background: string;
  surface: string;
  text: string;
  textSecondary: string;
}

export interface ThemeColorSet {
  light: ThemeColors;
  dark: ThemeColors;
}

// File system types
export interface FileEntry {
  name: string;
  path: string;
  type: 'file' | 'directory';
  size?: number;
  modified: string;
  created: string;
  extension?: string;
}

export interface FileItem {
  name: string;
  path: string;
  isDirectory: boolean;
  size?: number;
  modified?: string;
  created?: string;
}

export interface FileContent {
  name: string;
  path: string;
  content: string;
  language?: string;
  modified: boolean;
  lineCount: number;
  size: number;
}

export interface SendMessageOptions {
  content: string;
  files: string[];
  codeCli: string;
  resumeSessionId?: string | '';
  workspaceId: string;
  workspaceDir?: string;
  model?: string;
  clipboardAttachments?: ClipboardAttachment[];
  directCli?: boolean;
  cliCommand?: string;
  cliArgs?: string[];
}

export interface ChatMessage {
  id: string;
  sessionId: string;
  workspaceId: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: string;
  files?: string[];
  model?: string;
  fileMetadata?: Record<string, FileMetadata>;
}

export interface ChatResponse {
  message: ChatMessage;
  usage?: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
}

export interface ClipboardAttachment {
  path: string;
  width: number;
  height: number;
  preview?: string;
}

export interface FileMetadata {
  width: number;
  height: number;
  preview?: string;
}

export interface ChatSession {
  id: string;
  name?: string;
  sessionId?: string;
  workspaceId: string;
  messages: ChatMessage[];
  createdAt: string;
  updatedAt: string;
  messageCount: number;
  firstMessagePreview: string;
  codeCliTaskIds?: Record<string, string>;
}

export interface AiResponseEventPayload {
  request_id: string;
  delta: string;
  done: boolean;
  session_id?: string | null;
  workspace_id?: string | null;
  code_cli_task_id?: string | null;
  timestamp: string;
}

export type BackendChatMessage = Partial<ChatMessage> & {
  session_id?: string;
  workspace_id?: string;
  timestamp?: string;
  file_metadata?: Record<string, FileMetadata>;
};

export type BackendChatSession = Partial<ChatSession> & {
  id: string;
  messages?: BackendChatMessage[];
  created_at?: string;
  updated_at?: string;
  message_count?: number;
  first_message_preview?: string;
  session_id?: string;
  workspace_id?: string;
  code_cli_task_ids?: Record<string, string>;
};

// Terminal types
export interface TerminalSession {
  id: string;
  title: string;
  cwd: string;
  pid?: number;
  createdAt: string;
  lastActivity: string;
}

export interface TerminalOutput {
  sessionId: string;
  data: string;
  type: 'stdout' | 'stderr' | 'stdin';
  timestamp: string;
}

// Command types
export interface CommandResult {
  success: boolean;
  output: string;
  error?: string;
  exitCode?: number;
  duration: number;
}

// Event types
export interface AppEvent {
  type: string;
  data: any;
  timestamp: string;
}

// Configuration types
export interface AppConfig {
  version: string;
  environment: 'development' | 'production' | 'test';
  features: {
    ai: boolean;
    terminal: boolean;
    fileSystem: boolean;
    git: boolean;
  };
}

// API response types
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export interface TerminalTab {
  id: string;
  name: string;
  sessionId: string;
  terminal: Terminal;
}

// Utility types
export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;
export type Nullable<T> = T | null;
export type Maybe<T> = T | undefined;
