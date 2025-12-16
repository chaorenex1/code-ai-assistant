<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Folder, Document, Plus, Refresh, Search } from '@element-plus/icons-vue'
import { ElTree, ElInput, ElButton, ElIcon, ElDropdown, ElDropdownMenu, ElDropdownItem } from 'element-plus'
import { useFileStore } from '../../stores/modules/files'

const fileStore = useFileStore()
const searchQuery = ref('')

// Load directory on mount
onMounted(() => {
  fileStore.loadDirectory()
})

// Handle node click
function handleNodeClick(data: any) {
  if (data.isDirectory) {
    fileStore.loadDirectory(data.path)
  } else {
    fileStore.openFile(data.path)
  }
}

// Handle context menu
function handleContextMenu(event: MouseEvent, node: any) {
  event.preventDefault()
  // TODO: Implement context menu
  console.log('Context menu:', node)
}

// Create new file/folder
async function createNew(isDirectory = false) {
  const name = prompt(`请输入${isDirectory ? '文件夹' : '文件'}名称:`)
  if (name) {
    await fileStore.createFile(name, isDirectory)
  }
}

// Refresh directory
function refreshDirectory() {
  fileStore.loadDirectory(fileStore.currentDirectory)
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Toolbar -->
    <div class="border-b border-border bg-surface p-2">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center space-x-1">
          <ElButton :icon="Plus" size="small" text @click="createNew(false)">
            文件
          </ElButton>
          <ElButton :icon="Folder" size="small" text @click="createNew(true)">
            文件夹
          </ElButton>
        </div>
        <ElButton :icon="Refresh" size="small" text @click="refreshDirectory" />
      </div>

      <ElInput
        v-model="searchQuery"
        :prefix-icon="Search"
        placeholder="搜索文件..."
        size="small"
        clearable
      />
    </div>

    <!-- File Tree -->
    <div class="flex-1 overflow-auto p-2">
      <div v-if="fileStore.isLoading" class="flex-col-center h-full">
        <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary-500"></div>
        <p class="mt-2 text-sm text-text-secondary">加载中...</p>
      </div>

      <div v-else-if="fileStore.error" class="text-center p-4 text-error">
        {{ fileStore.error }}
      </div>

      <div v-else-if="fileStore.files.length === 0" class="text-center p-4 text-text-secondary">
        目录为空
      </div>

      <div v-else class="space-y-1">
        <div
          v-for="file in fileStore.files.filter(f =>
            searchQuery ? f.name.toLowerCase().includes(searchQuery.toLowerCase()) : true
          )"
          :key="file.path"
          class="flex items-center p-2 hover:bg-surface rounded cursor-pointer group"
          @click="handleNodeClick(file)"
          @contextmenu="handleContextMenu($event, file)"
        >
          <ElIcon :size="16" class="mr-2">
            <Folder v-if="file.isDirectory" />
            <Document v-else />
          </ElIcon>

          <span class="flex-1 text-ellipsis">{{ file.name }}</span>

          <div class="opacity-0 group-hover:opacity-100">
            <ElDropdown trigger="click" @command="(command) => handleContextCommand(command, file)">
              <span class="el-dropdown-link">
                <ElIcon :size="14"><More /></ElIcon>
              </span>
              <template #dropdown>
                <ElDropdownMenu>
                  <ElDropdownItem command="rename">重命名</ElDropdownItem>
                  <ElDropdownItem command="delete" divided>删除</ElDropdownItem>
                  <ElDropdownItem command="copy_path">复制路径</ElDropdownItem>
                  <ElDropdownItem command="open_terminal" v-if="file.isDirectory">在终端打开</ElDropdownItem>
                </ElDropdownMenu>
              </template>
            </ElDropdown>
          </div>
        </div>
      </div>
    </div>

    <!-- Current Path -->
    <div class="border-t border-border bg-surface p-2 text-xs text-text-secondary">
      <div class="text-ellipsis" :title="fileStore.currentDirectory">
        当前目录: {{ fileStore.currentDirectory }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
// Separate script block for component registration
import { More } from '@element-plus/icons-vue'

export default {
  components: {
    More,
  },
}
</script>

<style scoped>
:deep(.el-tree) {
  background: transparent;
}

:deep(.el-tree-node__content) {
  height: 32px;
}

:deep(.el-tree-node__content:hover) {
  background-color: var(--color-surface);
}
</style>