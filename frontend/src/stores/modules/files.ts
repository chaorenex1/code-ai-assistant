import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface FileItem {
  name: string
  path: string
  isDirectory: boolean
  size?: number
  modified?: string
  created?: string
}

export interface FileContent {
  path: string
  content: string
  language?: string
  modified: boolean
}

export const useFileStore = defineStore('files', () => {
  // State
  const currentDirectory = ref<string>('.')
  const files = ref<FileItem[]>([])
  const openedFiles = ref<FileContent[]>([])
  const activeFileIndex = ref<number>(-1)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const activeFile = computed(() => {
    if (activeFileIndex.value >= 0 && activeFileIndex.value < openedFiles.value.length) {
      return openedFiles.value[activeFileIndex.value]
    }
    return null
  })

  const hasUnsavedChanges = computed(() => {
    return openedFiles.value.some(file => file.modified)
  })

  // Actions
  async function loadDirectory(path?: string) {
    try {
      isLoading.value = true
      error.value = null

      const targetPath = path || currentDirectory.value
      const fileList = await invoke('list_files', { path: targetPath }) as Array<{
        name: string
        path: string
        is_directory: boolean
        size: number
        modified?: string
      }>

      files.value = fileList.map(file => ({
        name: file.name,
        path: file.path,
        isDirectory: file.is_directory,
        size: file.size,
        modified: file.modified,
      }))

      currentDirectory.value = targetPath
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载目录失败'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  async function openFile(path: string) {
    try {
      error.value = null

      // Check if file is already opened
      const existingIndex = openedFiles.value.findIndex(file => file.path === path)
      if (existingIndex >= 0) {
        activeFileIndex.value = existingIndex
        return openedFiles.value[existingIndex]
      }

      // Read file content
      const content = await invoke('read_file', { path })

      const fileContent: FileContent = {
        path,
        content: content as string,
        language: getLanguageFromPath(path),
        modified: false,
      }

      openedFiles.value.push(fileContent)
      activeFileIndex.value = openedFiles.value.length - 1

      return fileContent
    } catch (err) {
      error.value = err instanceof Error ? err.message : '打开文件失败'
      throw err
    }
  }

  async function saveFile(content?: string) {
    try {
      error.value = null

      const file = activeFile.value
      if (!file) {
        throw new Error('没有活动的文件')
      }

      const saveContent = content || file.content
      await invoke('write_file', {
        path: file.path,
        content: saveContent,
      })

      // Update file content and mark as saved
      if (activeFileIndex.value >= 0) {
        openedFiles.value[activeFileIndex.value] = {
          ...file,
          content: saveContent,
          modified: false,
        }
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '保存文件失败'
      throw err
    }
  }

  async function saveAllFiles() {
    try {
      error.value = null

      for (const file of openedFiles.value) {
        if (file.modified) {
          await invoke('write_file', {
            path: file.path,
            content: file.content,
          })
          file.modified = false
        }
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '保存所有文件失败'
      throw err
    }
  }

  async function createFile(name: string, isDirectory = false) {
    try {
      error.value = null

      const path = `${currentDirectory.value}/${name}`

      if (isDirectory) {
        await invoke('create_directory', { path })
      } else {
        await invoke('create_file', { path })
      }

      // Reload directory
      await loadDirectory(currentDirectory.value)

      // If it's a file, open it
      if (!isDirectory) {
        await openFile(path)
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '创建文件失败'
      throw err
    }
  }

  async function deleteFile(path: string) {
    try {
      error.value = null

      // Check if file is opened
      const fileIndex = openedFiles.value.findIndex(file => file.path === path)
      if (fileIndex >= 0) {
        // Close file if it's opened
        closeFile(path)
      }

      // Delete file
      await invoke('delete_file', { path })

      // Reload directory
      await loadDirectory(currentDirectory.value)
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除文件失败'
      throw err
    }
  }

  async function renameFile(oldPath: string, newName: string) {
    try {
      error.value = null

      const newPath = `${currentDirectory.value}/${newName}`
      await invoke('rename_file', { oldPath, newPath })

      // Update opened files
      const fileIndex = openedFiles.value.findIndex(file => file.path === oldPath)
      if (fileIndex >= 0) {
        openedFiles.value[fileIndex].path = newPath
      }

      // Reload directory
      await loadDirectory(currentDirectory.value)
    } catch (err) {
      error.value = err instanceof Error ? err.message : '重命名文件失败'
      throw err
    }
  }

  function updateFileContent(content: string) {
    if (activeFileIndex.value >= 0) {
      openedFiles.value[activeFileIndex.value] = {
        ...openedFiles.value[activeFileIndex.value],
        content,
        modified: true,
      }
    }
  }

  function closeFile(path: string) {
    const fileIndex = openedFiles.value.findIndex(file => file.path === path)
    if (fileIndex >= 0) {
      openedFiles.value.splice(fileIndex, 1)

      // Update active file index
      if (activeFileIndex.value >= fileIndex) {
        activeFileIndex.value = Math.max(0, activeFileIndex.value - 1)
      }
    }
  }

  function closeAllFiles() {
    openedFiles.value = []
    activeFileIndex.value = -1
  }

  function setActiveFile(index: number) {
    if (index >= 0 && index < openedFiles.value.length) {
      activeFileIndex.value = index
    }
  }

  function clearError() {
    error.value = null
  }

  // Helper functions
  function getLanguageFromPath(path: string): string {
    const extension = path.split('.').pop()?.toLowerCase() || ''

    const languageMap: Record<string, string> = {
      'js': 'javascript',
      'ts': 'typescript',
      'jsx': 'javascript',
      'tsx': 'typescript',
      'vue': 'vue',
      'html': 'html',
      'css': 'css',
      'scss': 'scss',
      'json': 'json',
      'md': 'markdown',
      'py': 'python',
      'rs': 'rust',
      'go': 'go',
      'java': 'java',
      'cpp': 'cpp',
      'c': 'c',
      'h': 'c',
      'hpp': 'cpp',
      'sh': 'shell',
      'bash': 'shell',
      'toml': 'toml',
      'yaml': 'yaml',
      'yml': 'yaml',
      'xml': 'xml',
    }

    return languageMap[extension] || 'plaintext'
  }

  return {
    // State
    currentDirectory,
    files,
    openedFiles,
    activeFileIndex,
    isLoading,
    error,

    // Getters
    activeFile,
    hasUnsavedChanges,

    // Actions
    loadDirectory,
    openFile,
    saveFile,
    saveAllFiles,
    createFile,
    deleteFile,
    renameFile,
    updateFileContent,
    closeFile,
    closeAllFiles,
    setActiveFile,
    clearError,
  }
})