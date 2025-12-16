import { listen, emit, type Event } from '@tauri-apps/api/event'
import type { AppEvent, FileEntry, TerminalOutput, ChatMessage } from '@/utils/types'

// Event types
export type EventType =
  | 'file:created'
  | 'file:modified'
  | 'file:deleted'
  | 'file:renamed'
  | 'directory:created'
  | 'directory:deleted'
  | 'terminal:output'
  | 'terminal:session:started'
  | 'terminal:session:ended'
  | 'chat:message:received'
  | 'chat:message:sent'
  | 'settings:updated'
  | 'workspace:changed'
  | 'app:error'
  | 'app:warning'
  | 'app:info'
  | 'app:status'

// Event handlers
type EventHandler<T = any> = (data: T) => void
type EventHandlers = Record<string, EventHandler[]>

class EventService {
  private handlers: EventHandlers = {}
  private isInitialized = false

  // Initialize event listeners
  async initialize(): Promise<void> {
    if (this.isInitialized) return

    // Register all event listeners
    const events: EventType[] = [
      'file:created',
      'file:modified',
      'file:deleted',
      'file:renamed',
      'directory:created',
      'directory:deleted',
      'terminal:output',
      'terminal:session:started',
      'terminal:session:ended',
      'chat:message:received',
      'chat:message:sent',
      'settings:updated',
      'workspace:changed',
      'app:error',
      'app:warning',
      'app:info',
      'app:status'
    ]

    for (const event of events) {
      await this.registerTauriListener(event)
    }

    this.isInitialized = true
    console.log('Event service initialized')
  }

  // Register Tauri event listener
  private async registerTauriListener(event: EventType): Promise<void> {
    try {
      await listen(event, (event: Event<any>) => {
        this.handleEvent(event.event, event.payload)
      })
    } catch (error) {
      console.error(`Failed to register listener for ${event}:`, error)
    }
  }

  // Handle incoming event
  private handleEvent(event: string, data: any): void {
    const handlers = this.handlers[event] || []
    handlers.forEach(handler => {
      try {
        handler(data)
      } catch (error) {
        console.error(`Error in event handler for ${event}:`, error)
      }
    })
  }

  // Subscribe to event
  subscribe<T = any>(event: EventType, handler: EventHandler<T>): () => void {
    if (!this.handlers[event]) {
      this.handlers[event] = []
    }

    this.handlers[event].push(handler as EventHandler)

    // Return unsubscribe function
    return () => {
      this.unsubscribe(event, handler)
    }
  }

  // Unsubscribe from event
  unsubscribe<T = any>(event: EventType, handler: EventHandler<T>): void {
    if (!this.handlers[event]) return

    const index = this.handlers[event].indexOf(handler as EventHandler)
    if (index !== -1) {
      this.handlers[event].splice(index, 1)
    }
  }

  // Emit event
  async emit<T = any>(event: EventType, data?: T): Promise<void> {
    try {
      await emit(event, data)
    } catch (error) {
      console.error(`Failed to emit event ${event}:`, error)
    }
  }

  // File system events
  onFileCreated(handler: EventHandler<FileEntry>): () => void {
    return this.subscribe('file:created', handler)
  }

  onFileModified(handler: EventHandler<FileEntry>): () => void {
    return this.subscribe('file:modified', handler)
  }

  onFileDeleted(handler: EventHandler<{ path: string }>): () => void {
    return this.subscribe('file:deleted', handler)
  }

  onFileRenamed(handler: EventHandler<{ oldPath: string; newPath: string }>): () => void {
    return this.subscribe('file:renamed', handler)
  }

  onDirectoryCreated(handler: EventHandler<FileEntry>): () => void {
    return this.subscribe('directory:created', handler)
  }

  onDirectoryDeleted(handler: EventHandler<{ path: string }>): () => void {
    return this.subscribe('directory:deleted', handler)
  }

  // Terminal events
  onTerminalOutput(handler: EventHandler<TerminalOutput>): () => void {
    return this.subscribe('terminal:output', handler)
  }

  onTerminalSessionStarted(handler: EventHandler<{ sessionId: string }>): () => void {
    return this.subscribe('terminal:session:started', handler)
  }

  onTerminalSessionEnded(handler: EventHandler<{ sessionId: string }>): () => void {
    return this.subscribe('terminal:session:ended', handler)
  }

  // Chat events
  onChatMessageReceived(handler: EventHandler<ChatMessage>): () => void {
    return this.subscribe('chat:message:received', handler)
  }

  onChatMessageSent(handler: EventHandler<ChatMessage>): () => void {
    return this.subscribe('chat:message:sent', handler)
  }

  // Settings events
  onSettingsUpdated(handler: EventHandler<Record<string, any>>): () => void {
    return this.subscribe('settings:updated', handler)
  }

  onWorkspaceChanged(handler: EventHandler<{ workspaceId: string }>): () => void {
    return this.subscribe('workspace:changed', handler)
  }

  // Application events
  onAppError(handler: EventHandler<{ message: string; details?: any }>): () => void {
    return this.subscribe('app:error', handler)
  }

  onAppWarning(handler: EventHandler<{ message: string; details?: any }>): () => void {
    return this.subscribe('app:warning', handler)
  }

  onAppInfo(handler: EventHandler<{ message: string; details?: any }>): () => void {
    return this.subscribe('app:info', handler)
  }

  onAppStatus(handler: EventHandler<{ status: string; details?: any }>): () => void {
    return this.subscribe('app:status', handler)
  }

  // Cleanup
  cleanup(): void {
    this.handlers = {}
    this.isInitialized = false
  }
}

// Export singleton instance
export const eventService = new EventService()

// Helper functions for common events
export async function emitFileCreated(file: FileEntry): Promise<void> {
  await eventService.emit('file:created', file)
}

export async function emitFileModified(file: FileEntry): Promise<void> {
  await eventService.emit('file:modified', file)
}

export async function emitFileDeleted(path: string): Promise<void> {
  await eventService.emit('file:deleted', { path })
}

export async function emitFileRenamed(oldPath: string, newPath: string): Promise<void> {
  await eventService.emit('file:renamed', { oldPath, newPath })
}

export async function emitDirectoryCreated(directory: FileEntry): Promise<void> {
  await eventService.emit('directory:created', directory)
}

export async function emitDirectoryDeleted(path: string): Promise<void> {
  await eventService.emit('directory:deleted', { path })
}

export async function emitTerminalOutput(output: TerminalOutput): Promise<void> {
  await eventService.emit('terminal:output', output)
}

export async function emitTerminalSessionStarted(sessionId: string): Promise<void> {
  await eventService.emit('terminal:session:started', { sessionId })
}

export async function emitTerminalSessionEnded(sessionId: string): Promise<void> {
  await eventService.emit('terminal:session:ended', { sessionId })
}

export async function emitChatMessageReceived(message: ChatMessage): Promise<void> {
  await eventService.emit('chat:message:received', message)
}

export async function emitChatMessageSent(message: ChatMessage): Promise<void> {
  await eventService.emit('chat:message:sent', message)
}

export async function emitSettingsUpdated(settings: Record<string, any>): Promise<void> {
  await eventService.emit('settings:updated', settings)
}

export async function emitWorkspaceChanged(workspaceId: string): Promise<void> {
  await eventService.emit('workspace:changed', { workspaceId })
}

export async function emitAppError(message: string, details?: any): Promise<void> {
  await eventService.emit('app:error', { message, details })
}

export async function emitAppWarning(message: string, details?: any): Promise<void> {
  await eventService.emit('app:warning', { message, details })
}

export async function emitAppInfo(message: string, details?: any): Promise<void> {
  await eventService.emit('app:info', { message, details })
}

export async function emitAppStatus(status: string, details?: any): Promise<void> {
  await eventService.emit('app:status', { status, details })
}