<script setup lang="ts">
import { Folder, Document, Plus, Refresh, Search } from '@element-plus/icons-vue';
import { ElTree, ElInput, ElButton, ElIcon, ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted, watch } from 'vue';

import { useFileStore } from '@/stores/filesStore';
import { useAppStore } from '@/stores/workspaceStore';

const fileStore = useFileStore();
const appStore = useAppStore();

const searchQuery = ref('');
const treeRef = ref<InstanceType<typeof ElTree> | null>(null);

interface FileNode {
  name: string;
  path: string;
  isDirectory: boolean;
  size?: number;
  modified?: string;
  isLeaf?: boolean;
}

const treeData = ref<FileNode[]>([]);

const defaultProps = {
  children: 'children',
  label: 'name',
  isLeaf: 'isLeaf',
};

// 初始化根目录列表
onMounted(async () => {
  // 如果已经在其他地方加载过目录（例如最近目录或设置面板），直接使用现有数据
  if (!fileStore.files.length) {
    await fileStore.loadDirectory();
  }

  treeData.value = fileStore.files.map((file) => ({
    name: file.name,
    path: file.path,
    isDirectory: file.isDirectory,
    size: file.size,
    modified: file.modified,
    isLeaf: !file.isDirectory,
  }));
});

// 根据文件列表变化更新根节点
watch(
  () => fileStore.files,
  (files) => {
    treeData.value = files.map((file) => ({
      name: file.name,
      path: file.path,
      isDirectory: file.isDirectory,
      size: file.size,
      modified: file.modified,
      isLeaf: !file.isDirectory,
    }));
  },
  { deep: true }
);

// 懒加载子目录（根节点完全由 treeData 提供）
async function loadNode(node: any, resolve: (data: FileNode[]) => void) {
  const data = node.data as FileNode | undefined;

  // 根节点使用 :data="treeData"，这里不再处理
  if (!data) {
    return resolve([]);
  }

  if (!data.isDirectory) {
    return resolve([]);
  }

  try {
    const fileList = (await invoke('list_files', { path: data.path })) as Array<{
      name: string;
      path: string;
      is_directory: boolean;
      size: number;
      modified?: string;
    }>;

    const children: FileNode[] = fileList.map((file) => ({
      name: file.name,
      path: file.path,
      isDirectory: file.is_directory,
      size: file.size,
      modified: file.modified,
      isLeaf: !file.is_directory,
    }));

    resolve(children);
  } catch (error) {
    console.error('加载子目录失败', error);
    resolve([]);
  }
}

// 过滤节点（搜索）
function filterNode(value: string, data: any) {
  if (!value) return true;
  return data.name.toLowerCase().includes(value.toLowerCase());
}

watch(searchQuery, (val) => {
  treeRef.value?.filter(val);
});

// 处理节点点击：目录展开/收起，文件在编辑器打开
async function handleNodeClick(data: FileNode, node: any) {
  if (data.isDirectory) {
    node.expanded = !node.expanded;
  } else {
    try {
      // 显示加载状态
      fileStore.isLoading = true;
      await fileStore.openFile(data.path);
      appStore.setCurrentFile(data.path);
    } catch (error) {
      console.error('打开文件失败', error);
      ElMessage.error(
        (error instanceof Error && error.message) || '打开文件失败（可能不是文本文件或编码不兼容）'
      );
    } finally {
      // 确保加载状态被清除
      fileStore.isLoading = false;
    }
  }
}

// Create new file/folder
async function createNew(isDirectory = false) {
  const name = prompt(`请输入${isDirectory ? '文件夹' : '文件'}名称:`);
  if (name) {
    await fileStore.createFile(name, isDirectory);
  }
}

// Refresh directory
function refreshDirectory() {
  fileStore.loadDirectory(fileStore.currentDirectory);
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Toolbar -->
    <div class="border-b border-border bg-surface p-2">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center space-x-1">
          <ElButton
            :icon="Plus"
            size="small"
            text
            @click="createNew(false)"
          >
            文件
          </ElButton>
          <ElButton
            :icon="Folder"
            size="small"
            text
            @click="createNew(true)"
          >
            文件夹
          </ElButton>
        </div>
        <ElButton
          :icon="Refresh"
          size="small"
          text
          @click="refreshDirectory"
        />
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
      <div
        v-if="fileStore.isLoading"
        class="flex flex-col items-center justify-center h-full"
      >
        <div
          class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary-500"
        />
        <p class="mt-2 text-sm text-text-secondary">
          加载中...
        </p>
      </div>

      <div
        v-else-if="fileStore.error && !treeData.length"
        class="text-center p-4 text-error"
      >
        {{ fileStore.error }}
      </div>

      <ElTree
        v-else
        ref="treeRef"
        :data="treeData"
        :props="defaultProps"
        node-key="path"
        lazy
        highlight-current
        :load="loadNode"
        :filter-node-method="filterNode"
        @node-click="handleNodeClick"
      >
        <template #default="{ data }">
          <div class="flex items-center px-1 py-1">
            <ElIcon
              :size="16"
              class="mr-2"
            >
              <Folder v-if="data.isDirectory" />
              <Document v-else />
            </ElIcon>
            <span class="flex-1 truncate">{{ data.name }}</span>
          </div>
        </template>
      </ElTree>
    </div>

    <!-- Current Path -->
    <div class="border-t border-border bg-surface p-2 text-xs text-text-secondary">
      <div
        class="truncate"
        :title="fileStore.currentDirectory"
      >
        当前目录: {{ fileStore.currentDirectory }}
      </div>
    </div>
  </div>
</template>

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
