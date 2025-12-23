import { invoke } from '@tauri-apps/api/core';

import type {
  FileEntry,
  AIModel,
  Workspace,
  CommandResult,
  ApiResponse,
} from '@/utils/types';

// File system commands
export async function readFile(path: string): Promise<string> {
  return invoke('read_file', { path });
}

export async function writeFile(path: string, content: string): Promise<void> {
  return invoke('write_file', { path, content });
}

export async function listFiles(path: string): Promise<FileEntry[]> {
  return invoke('list_files', { path });
}

export async function createFile(path: string): Promise<void> {
  return invoke('create_file', { path });
}

export async function deleteFile(path: string): Promise<void> {
  return invoke('delete_file', { path });
}

export async function renameFile(oldPath: string, newPath: string): Promise<void> {
  return invoke('rename_file', { oldPath, newPath });
}

export async function createDirectory(path: string): Promise<void> {
  return invoke('create_directory', { path });
}

export async function listDirectories(path: string): Promise<FileEntry[]> {
  return invoke('list_directories', { path });
}

export async function deleteDirectory(path: string): Promise<void> {
  return invoke('delete_directory', { path });
}

// AI chat commands
// Note: backend `send_chat_message` currently returns a plain string
// response and accepts `message` and optional `context_files`.
export async function sendChatMessage(
  message: string,
  contextFiles?: string[]
): Promise<string> {
  return invoke('send_chat_message', { message, context_files: contextFiles });
}

// Streaming AI chat command: backend will emit incremental
// `ai-response` events while returning a requestId immediately.
export async function sendChatMessageStreaming(
  message: string,
  contextFiles?: string[]
): Promise<string> {
  return invoke('send_chat_message_streaming', { message, context_files: contextFiles });
}

export async function getAIModels(): Promise<AIModel[]> {
  return invoke('get_ai_models');
}

export async function setAIModel(modelId: string): Promise<void> {
  return invoke('set_ai_model', { modelId });
}

// Terminal commands
export async function executeCommand(
  command: string,
  args?: string[],
  cwd?: string
): Promise<CommandResult> {
  return invoke('execute_command', { command, args, cwd });
}

export async function executeTerminalCommand(
  sessionId: string,
  shell: string,
  command: string
): Promise<string> {
  return invoke('execute_terminal_command', {
    sessionId,
    shell,
    command,
  });
}

export async function spawnTerminal(cwd?: string): Promise<string> {
  return invoke('spawn_terminal', { cwd });
}

export async function killTerminal(sessionId: string): Promise<void> {
  return invoke('kill_terminal', { terminal_id: sessionId });
}

// Settings commands
export async function getSettings(): Promise<Record<string, any>> {
  return invoke('get_settings');
}

export async function saveSettings(settings: Record<string, any>): Promise<void> {
  return invoke('save_settings', { settings });
}

export async function resetSettings(): Promise<void> {
  return invoke('reset_settings');
}

// Workspace commands
export async function getWorkspaces(): Promise<Workspace[]> {
  return invoke('get_workspaces');
}

export async function createWorkspace(name: string, path: string): Promise<Workspace> {
  return invoke('create_workspace', { name, path });
}

export async function switchWorkspace(workspaceId: string): Promise<void> {
  return invoke('switch_workspace', { workspaceId });
}

export async function deleteWorkspace(workspaceId: string): Promise<void> {
  return invoke('delete_workspace', { workspaceId });
}

// System commands
export async function getSystemInfo(): Promise<Record<string, any>> {
  return invoke('get_system_info');
}

export async function getLogs(limit?: number): Promise<string[]> {
  return invoke('get_logs', { limit });
}

export async function clearLogs(): Promise<void> {
  return invoke('clear_logs');
}

// Utility commands
export async function openFileDialog(options?: {
  multiple?: boolean;
  directory?: boolean;
  filters?: Array<{ name: string; extensions: string[] }>;
}): Promise<string | string[]> {
  return invoke('open_file_dialog', { options });
}

export async function saveFileDialog(options?: {
  defaultPath?: string;
  filters?: Array<{ name: string; extensions: string[] }>;
}): Promise<string> {
  return invoke('save_file_dialog', { options });
}

export async function showNotification(title: string, body?: string): Promise<void> {
  return invoke('show_notification', { title, body });
}

export async function showDialog(options: {
  title?: string;
  message: string;
  type?: 'info' | 'warning' | 'error' | 'question';
  buttons?: string[];
}): Promise<number> {
  return invoke('show_dialog', { options });
}

// Application commands
export async function getAppVersion(): Promise<string> {
  return invoke('get_app_version');
}

export async function restartApp(): Promise<void> {
  return invoke('restart_app');
}

export async function quitApp(): Promise<void> {
  return invoke('quit_app');
}

// Event commands
export async function subscribeToEvent(
  event: string,
  callback: (data: any) => void
): Promise<void> {
  return invoke('subscribe_to_event', { event, callback });
}

export async function unsubscribeFromEvent(event: string): Promise<void> {
  return invoke('unsubscribe_from_event', { event });
}

// Error handling wrapper
export async function withErrorHandling<T>(
  fn: () => Promise<T>,
  errorMessage?: string
): Promise<ApiResponse<T>> {
  try {
    const data = await fn();
    return { success: true, data };
  } catch (error: any) {
    console.error(errorMessage || 'Command failed:', error);
    return {
      success: false,
      error: error.message || 'Unknown error occurred',
      message: errorMessage,
    };
  }
}

// Batch operations
export async function batchReadFiles(paths: string[]): Promise<Record<string, string>> {
  const results: Record<string, string> = {};

  for (const path of paths) {
    try {
      results[path] = await readFile(path);
    } catch (error) {
      console.error(`Failed to read file ${path}:`, error);
    }
  }

  return results;
}

export async function batchWriteFiles(files: Record<string, string>): Promise<void> {
  const promises = Object.entries(files).map(([path, content]) =>
    writeFile(path, content).catch((error) => {
      console.error(`Failed to write file ${path}:`, error);
      throw error;
    })
  );

  await Promise.all(promises);
}

// Recent directories commands
export interface RecentDirectory {
  path: string;
  openedAt: string;
}

export async function addRecentDirectory(path: string): Promise<void> {
  return invoke('add_recent_directory', { path });
}

export async function getRecentDirectories(): Promise<RecentDirectory[]> {
  return invoke('get_recent_directories');
}

export async function clearRecentDirectories(): Promise<void> {
  return invoke('clear_recent_directories');
}
