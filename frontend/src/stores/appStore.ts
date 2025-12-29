import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { AppSettings, Workspace } from '@/utils/types';
import { normalizePath, getDirectoryName } from '@/utils/pathUtils';
import {
  getSettings,
  saveSettings as saveSettingsCommand,
  getWorkspaces,
  deleteWorkspace as deleteWorkspaceCommand,
  createWorkspace as createWorkspaceCommand,
  switchWorkspace as switchWorkspaceCommand,
} from '@/services/tauri/commands';
// import {} from '@/services/tauri/events';

export const useAppStore = defineStore('app', () => {
  // State
  const defaultSettings = ref<AppSettings>({
    theme: 'light',
    colorScheme: 'light',
    editor: {
      fontSize: 14,
      tabSize: 4,
      wordWrap: 'off',
      minimap: { enabled: true },
      lineNumbers: 'on',
      autoSave: false,
      autoSaveDelay: 1000,
      formatOnSave: false,
      enableFileWatcher: true,
    },
    terminal: {
      fontSize: 14,
      fontFamily: 'Consolas, "Courier New", monospace',
      cursorStyle: 'block',
      cursorBlink: true,
    },
    chat: {
      autoScroll: true,
      markdownPreview: true,
      sendWithEnter: true,
      switchLineWithShiftEnter: true,
    },
    ai: {
      defaultModel: 'claude-3-5-sonnet',
      maxTokens: 2048,
      temperature: 0.7,
      topP: 1.0,
      model_list: ['claude-4', 'gpt-5', 'deepseek'],
      code_cli: ['claude-cli', 'codex-cli', 'gemini-cli'],
      useDirectCodeCli: false,
    },
    paths: {
      nodejs: '',
      python: '',
      git: '',
    },
    codeCli:[
      { name: 'claude-cli', command: '/usr/bin/claude', args: '' },
      { name: 'codex-cli', command: '/usr/bin/codex', args: '' },
      { name: 'gemini-cli', command: '/usr/bin/gemini', args: '' },
    ],
    models: [
      { id: 'claude-4-5', name: 'Claude 4.5', provider: 'Anthropic', endpoint: 'https://api.anthropic.com' },
      { id: 'gpt-5', name: 'GPT-5', provider: 'OpenAI', endpoint: 'https://api.openai.com' },
      { id: 'deepseek', name: 'Deepseek', provider: 'Deepseek', endpoint: 'https://api.deepseek.ai' },
    ],
    environmentVariables: [
      { name: 'API_KEY', value: '', isSecret: true },
      { name: 'PATH', value: '/usr/bin', isSecret: false },
    ],
    userPreferences: {
      currentModel: 'claude-3-5-sonnet',
      currentCodeCli: 'claude-cli',
      currentShell: 'bash',
    },

  } as AppSettings);
  const settings = ref<AppSettings>(defaultSettings.value);
  const workspaces = ref<Workspace[]>([]);

  const currentShell = ref<string>(settings.value.userPreferences.currentShell || '');
  const currentAiModel = ref<string>(settings.value.userPreferences.currentModel || '');
  const currentCodeCli = ref<string>(settings.value.userPreferences.currentCodeCli || '');
  const defaultWorkspace = ref<Workspace>({
    id: '',
    name: '',
    path: '',
    currentSessionId: '',
    isActive: false,
    createdAt: '',
    updatedAt: '',
    settings: {},
  });
  const currentWorkspace = ref<Workspace>(defaultWorkspace.value);
  const isConnected = ref(false);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  // Lightweight mode (enabled when minimized/hidden)
  const isLightweightMode = ref(false);
  const lightweightModeReason = ref<string | null>(null);

  // Getters
  const themeClass = computed(() => settings.value.theme);
  const fontSizeStyle = computed(() => `${settings.value.editor.fontSize}px`);
  const availableAiModels = computed(() => settings.value.ai.model_list);
  const availableCodeClis = computed(() => settings.value.ai.code_cli);
  const getCurrentWorkspace = computed(() => currentWorkspace.value);
  // Actions
  async function initialize() {
    isLoading.value = true;
    error.value = null;

    try {
      // Load settings from backend
      try {
        const backendSettings = await getSettings();
        if (backendSettings) {
          console.info('Loaded settings from backend:', backendSettings);
          settings.value = {
            ...settings.value,
            ...backendSettings,
          };
        } else {
          await saveSettings()
          console.warn('No settings received from backend, using defaults.');
        }
        currentAiModel.value = settings.value.userPreferences.currentModel || '';
        currentCodeCli.value = settings.value.userPreferences.currentCodeCli || '';
        currentShell.value = settings.value.userPreferences.currentShell || '';
      } catch (err) {
        console.warn('Failed to load settings from backend, using defaults:', err);
      }

      // Load workspaces
      try {
        // const backendCurrentWorkspaceData = await getCurrentWorkspaceCommand();
        // currentWorkspace.value = {
        //   ...currentWorkspace.value,
        //   ...backendCurrentWorkspaceData,
        // };
        // currentSessionId.value = currentWorkspace.value.currentSessionId || '';
        const backendWorkspaceList = await getWorkspaces();
        workspaces.value = backendWorkspaceList || [];
      } catch (err) {
        console.warn('Failed to load workspaces from backend:', err);
      }
      isConnected.value = true;
      saveToStorage();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '初始化失败';
      isConnected.value = false;
      console.error('Failed to initialize app:', err);
    } finally {
      isLoading.value = false;
    }
  }

  async function loadSettings() {
    try {
      const backendSettings = await invoke('get_settings');
      if (backendSettings) {
        settings.value = {
          ...settings.value,
          ...backendSettings,
        };
      }
    } catch (err) {
      console.error('Failed to load settings:', err);
    }
  }

  async function saveSettings(){
    await saveSettingsCommand(JSON.stringify(settings.value));
  }

  async function switchWorkspace(workspaceId: string) {
    try {
      const workspace: Workspace = await switchWorkspaceCommand(workspaceId);
      const oldWorkspace: Workspace | undefined = workspaces.value.find(
        (w) => w.id === currentWorkspace.value.id
      );

      // Update workspaces list: set old workspace to inactive
      if (oldWorkspace) {
        workspaces.value = workspaces.value.map((w) => {
          if (w.id === oldWorkspace.id) {
            return {
              ...w,
              isActive: false,
            };
          }
          return w;
        });
      }

      // Update current workspace
      if (currentWorkspace.value.id !== workspaceId) {
        currentWorkspace.value = {
          ...currentWorkspace.value,
          ...workspace,
        };
      }

      saveToStorage();
    } catch (err) {
      console.error('Failed to switch workspace:', err);
      throw err;
    }
  }

  async function createWorkspace(path: string, isActive: boolean) {
    try {
      // Generate a name for the workspace
      const normalizedPath = normalizePath(path);
      const name = getDirectoryName(normalizedPath);
      console.debug('Creating workspace with name:', name);
      const newWorkspace: Workspace = await createWorkspaceCommand(name, path, isActive);
      workspaces.value.push(newWorkspace);
      currentWorkspace.value = newWorkspace;
      saveToStorage();
      return newWorkspace;
    } catch (err) {
      console.error('Failed to create workspace:', err);
      throw err;
    }
  }

  async function deleteWorkspace(workspaceId: string) {
    try {
      await deleteWorkspaceCommand(workspaceId);
      workspaces.value = workspaces.value.filter((w) => w.id !== workspaceId);

      if (currentWorkspace.value.id === workspaceId) {
        currentWorkspace.value= {
          ...defaultWorkspace.value,
        }
      }
      saveToStorage();
    } catch (err) {
      console.error('Failed to delete workspace:', err);
      throw err;
    }
  }

  function setCurrentAiModel(model: string) {
    currentAiModel.value = model;
    settings.value.userPreferences.currentModel = model;
    saveToStorage();
  }

  function setCurrentCodeCli(cli: string) {
    currentCodeCli.value = cli;
    settings.value.userPreferences.currentCodeCli = cli;
    saveToStorage();
  }

  function setCurrentShell(shell: string) {
    currentShell.value = shell;
    settings.value.userPreferences.currentShell = shell;
    saveToStorage();
  }

  function setCurrentSessionId(sessionId: string) {
    currentWorkspace.value = {
      ...currentWorkspace.value,
      currentSessionId: sessionId,
    };
    saveToStorage();
  }

  async function resetToDefaults() {
    settings.value = defaultSettings.value;
    saveToStorage();
    await saveSettings();
  }

  function setLightweightMode(enabled: boolean, reason?: string) {
    if (isLightweightMode.value === enabled && (reason ?? null) === lightweightModeReason.value) {
      return;
    }

    isLightweightMode.value = enabled;
    lightweightModeReason.value = reason ?? null;
  }

  const saveToStorage = () => {
    localStorage.setItem('appSettings', JSON.stringify(settings.value));
    localStorage.setItem('currentWorkspace', JSON.stringify(currentWorkspace.value));
    localStorage.setItem('workspaces', JSON.stringify(workspaces.value));
    localStorage.setItem('currentAiModel', JSON.stringify(currentAiModel.value));
    localStorage.setItem('currentCodeCli', JSON.stringify(currentCodeCli.value));
    localStorage.setItem('currentShell', JSON.stringify(currentShell.value));
    localStorage.setItem('currentSessionId', JSON.stringify(currentWorkspace.value.currentSessionId));
  };

  // const loadFromStorage = () => {
  //   const storedSettings = localStorage.getItem('appSettings');
  //   const storedCurrentWorkspace = localStorage.getItem('currentWorkspace');
  //   const storedWorkspaces = localStorage.getItem('workspaces');
  //   const storedCurrentAiModel = localStorage.getItem('currentAiModel');
  //   const storedCurrentCodeCli = localStorage.getItem('currentCodeCli');
  //   const storedCurrentShell = localStorage.getItem('currentShell');
  //   if (storedCurrentWorkspace) {
  //     currentWorkspace.value = JSON.parse(storedCurrentWorkspace);
  //   }
  //   if (storedWorkspaces) {
  //     workspaces.value = JSON.parse(storedWorkspaces);
  //   }
  //   if (storedSettings) {
  //     settings.value = JSON.parse(storedSettings);
  //   }
  //   if (storedCurrentShell) {
  //     currentShell.value = JSON.parse(storedCurrentShell);
  //   }
  //   if (storedCurrentAiModel) {
  //     currentAiModel.value = JSON.parse(storedCurrentAiModel);
  //   }
  //   if (storedCurrentCodeCli) {
  //     currentCodeCli.value = JSON.parse(storedCurrentCodeCli);
  //   }
  // };

  return {
    // State
    currentShell,
    currentAiModel,
    currentCodeCli,
    currentWorkspace,
    isConnected,
    isLoading,
    error,
    isLightweightMode,
    lightweightModeReason,
    settings,
    workspaces,

    // Getters
    themeClass,
    fontSizeStyle,
    availableAiModels,
    availableCodeClis,
    getCurrentWorkspace,

    // Actions
    initialize,
    loadSettings,
    saveSettings,
    switchWorkspace,
    createWorkspace,
    deleteWorkspace,
    setCurrentAiModel,
    setCurrentCodeCli,
    setCurrentShell,
    resetToDefaults,
    setLightweightMode,
    setCurrentSessionId,
  };
});
