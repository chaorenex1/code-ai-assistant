import { defineStore } from 'pinia';
import { ref } from 'vue';

import {
  sendChatMessageStreaming,
  loadChatSessions,
  deleteChatSession,
  updateChatSessionName,
  cancelStreamingRequest,
} from '@/services/tauri/commands';
import type {
  ChatMessage,
  ChatSession,
  SendMessageOptions,
  AiResponseEventPayload,
  FileMetadata,
} from '@/utils/types';

export const useChatStore = defineStore('chat', () => {
  const messages = ref<ChatMessage[]>([]);
  const associatedFiles = ref<string[]>([]);
  const currentSessionId = ref<string>('');
  const currentRequestId = ref<string>('');
  const currentCodeCli = ref<string>('');
  const isStreaming = ref(false);
  const sessions = ref<ChatSession[]>([]);
  const isSessionsLoading = ref(false);
  const codeCliChanged = ref(false);
  const codeCliTaskIds = ref<Record<string, string>>({});
  const pendingCodeCliByRequestId = new Map<string, string>();
  const pendingUserMessageId = ref<string>('');

  function setAssociatedFiles(paths: string[]) {
    associatedFiles.value = [...paths];
  }

  function removeAssociatedFile(index: number) {
    associatedFiles.value.splice(index, 1);
  }

  function getCurrentSessionId(): string {
    return currentSessionId.value;
  }

  function getCurrentCodeCli(): string {
    return currentCodeCli.value;
  }

  function setCurrentCodeCli(codeCli: string) {
    currentCodeCli.value = codeCli;
  }

  async function sendMessage(options: SendMessageOptions): Promise<void> {
    const content = options.content.trim();
    if (!content) {
      return;
    }

    const clipboardAttachments = options.clipboardAttachments ?? [];
    const files = [...options.files, ...clipboardAttachments.map((attachment) => attachment.path)];
    const fileMetadataMap = clipboardAttachments.reduce<Record<string, FileMetadata>>(
      (acc, attachment) => {
        acc[attachment.path] = {
          width: attachment.width,
          height: attachment.height,
          preview: attachment.preview,
        };
        return acc;
      },
      {}
    );
    const fileMetadata = Object.keys(fileMetadataMap).length ? fileMetadataMap : undefined;
    const timestamp = new Date().toISOString();
    const model = options.model;
    const codeCli = options.codeCli;
    const resumeTaskId = codeCli ? codeCliTaskIds.value[codeCli] : undefined;
    console.debug('sendMessage', { content, files, model, codeCli, resumeTaskId, codeCliTaskIds, codeCliChanged });
    if (codeCli) {
      codeCliChanged.value = !resumeTaskId;
      currentCodeCli.value = codeCli;
    } else {
      codeCliChanged.value = false;
      currentCodeCli.value = '';
    }

    const userMessageId = `${Date.now()}-user`;
    messages.value.push({
      id: userMessageId,
      role: 'user',
      content,
      timestamp,
      files,
      model,
      sessionId: currentSessionId.value || '',
      workspaceId: options.workspaceId,
      fileMetadata,
    });
    pendingUserMessageId.value = userMessageId;

    try {
      isStreaming.value = true;
      const backendSessionId = currentSessionId.value || null;
      const requestId = await sendChatMessageStreaming(
        content,
        files,
        options.codeCli,
        model,
        backendSessionId,
        options.workspaceId,
        options.workspaceDir,
        codeCliChanged.value,
        resumeTaskId || null
      );
      currentRequestId.value = requestId;
      if (codeCli) {
        pendingCodeCliByRequestId.set(requestId, codeCli);
      }
      messages.value.push({
        id: requestId,
        role: 'assistant',
        content: '',
        timestamp: new Date().toISOString(),
        files,
        model,
        sessionId: currentSessionId.value || '',
        workspaceId: options.workspaceId,
      });
    } catch (error) {
      isStreaming.value = false;
      currentRequestId.value = '';
      console.error('Failed to send message:', error);
      throw error;
    }
  }

  function handleAiResponse(payload: AiResponseEventPayload): void {
    if (!payload?.request_id) {
      return;
    }
    const targetIndex = messages.value.findIndex((msg) => msg.id === payload.request_id);
    if (targetIndex === -1) {
      return;
    }

    const message = messages.value[targetIndex];
    if (message) {
      message.content += payload.delta || '';
    }

    if (payload.done) {
      const pendingCodeCli = pendingCodeCliByRequestId.get(payload.request_id);
      if (pendingCodeCli) {
        pendingCodeCliByRequestId.delete(payload.request_id);
        if (payload.code_cli_task_id) {
          codeCliTaskIds.value = {
            ...codeCliTaskIds.value,
            [pendingCodeCli]: payload.code_cli_task_id,
          };
        }
      }
      void finalizeStreaming(payload.session_id || '');
    }
  }

  async function finalizeStreaming(newSessionId: string) {
    isStreaming.value = false;
    currentRequestId.value = '';
    pendingUserMessageId.value = '';
    if (newSessionId) {
      currentSessionId.value = newSessionId as string;
    }
    // await autoSaveChatSession();
  }

  // async function autoSaveChatSession() {
  //   if (!messages.value.length) {
  //     return;
  //   }

  //   try {
  //     const session = await saveChatSession(
  //       currentSessionId.value,
  //       null,
  //       messages.value,
  //       currentSessionId.value
  //     );
  //     currentSessionId.value = session.id;
  //   } catch (error) {
  //     console.error('Failed to auto-save chat session:', error);
  //   }
  // }

  function loadSessionFromHistory(session: ChatSession) {
    messages.value = [...session.messages];
    currentSessionId.value = session.id;
    associatedFiles.value = [];
    codeCliTaskIds.value = { ...(session.codeCliTaskIds || {}) };
    pendingCodeCliByRequestId.clear();
    pendingUserMessageId.value = '';
  }

  function clearChat() {
    messages.value = [];
    associatedFiles.value = [];
    currentSessionId.value = '';
    currentRequestId.value = '';
    isStreaming.value = false;
    codeCliTaskIds.value = {};
    pendingCodeCliByRequestId.clear();
    pendingUserMessageId.value = '';
  }

  async function fetchSessions(workspaceId: string, limit = 50): Promise<ChatSession[]> {
    isSessionsLoading.value = true;
    try {
      sessions.value = await loadChatSessions(workspaceId, limit);
      return sessions.value;
    } finally {
      isSessionsLoading.value = false;
    }
  }

  async function removeSession(sessionId: string): Promise<void> {
    await deleteChatSession(sessionId);
    sessions.value = sessions.value.filter((session) => session.id !== sessionId);

    if (currentSessionId.value === sessionId) {
      currentSessionId.value = '';
      codeCliTaskIds.value = {};
      pendingCodeCliByRequestId.clear();
      pendingUserMessageId.value = '';
    }
  }

  async function renameSession(sessionId: string, name: string): Promise<ChatSession> {
    const updated = await updateChatSessionName(sessionId, name);
    const index = sessions.value.findIndex((session) => session.id === sessionId);
    if (index !== -1) {
      sessions.value[index] = updated;
    }
    if (currentSessionId.value === sessionId) {
      currentSessionId.value = updated.id;
    }
    return updated;
  }

  async function cancelStreaming(): Promise<void> {
    if (!currentRequestId.value) {
      return;
    }
    try {
      await cancelStreamingRequest(currentRequestId.value);
    } catch (error) {
      console.warn('Failed to cancel streaming:', error);
    } finally {
      rollbackStreaming();
    }
  }

  function rollbackStreaming() {
    if (!currentRequestId.value) {
      return;
    }

    const assistantIndex = messages.value.findIndex((msg) => msg.id === currentRequestId.value);
    if (assistantIndex !== -1) {
      messages.value.splice(assistantIndex, 1);
    }

    if (pendingUserMessageId.value) {
      const userIndex = messages.value.findIndex((msg) => msg.id === pendingUserMessageId.value);
      if (userIndex !== -1) {
        messages.value.splice(userIndex, 1);
      }
    }

    isStreaming.value = false;
    currentRequestId.value = '';
    pendingUserMessageId.value = '';
  }

  return {
    // State
    messages,
    associatedFiles,
    currentSessionId,
    currentRequestId,
    isStreaming,
    sessions,
    isSessionsLoading,
    codeCliTaskIds,

    // Actions
    getCurrentSessionId,
    getCurrentCodeCli,
    setCurrentCodeCli,
    setAssociatedFiles,
    removeAssociatedFile,
    sendMessage,
    handleAiResponse,
    loadSessionFromHistory,
    clearChat,
    fetchSessions,
    removeSession,
    renameSession,
    cancelStreaming,
  };
});
