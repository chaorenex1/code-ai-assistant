<script setup lang="ts">
import { ElForm, ElFormItem, ElInput, ElButton, ElMessage } from 'element-plus';
import { ref, onMounted } from 'vue';
import { getSettings, saveSettings as saveTauriSettings } from '@/services/tauri/commands';

// CLI tool paths
const cliPaths = ref({
  nodejs: '/usr/bin/node',
  python: '/usr/bin/python3',
  git: '/usr/bin/git',
});

// Save settings
async function saveSettings() {
  try {
    const settingsToSave: Record<string, any> = {
      'cli.nodejs': cliPaths.value.nodejs,
      'cli.python': cliPaths.value.python,
      'cli.git': cliPaths.value.git,
    };

    await saveTauriSettings(settingsToSave);
    ElMessage.success('CLI 工具路径已保存');
  } catch (error) {
    console.error('Failed to save CLI paths:', error);
    ElMessage.error('保存失败: ' + (error as Error).message);
  }
}

// Reset to defaults
async function resetSettings() {
  if (confirm('确定要重置 CLI 工具路径为默认值吗？')) {
    try {
      cliPaths.value = {
        nodejs: '/usr/bin/node',
        python: '/usr/bin/python3',
        git: '/usr/bin/git',
      };
      await saveSettings();
      ElMessage.success('已重置为默认值');
    } catch (error) {
      console.error('Failed to reset CLI paths:', error);
      ElMessage.error('重置失败: ' + (error as Error).message);
    }
  }
}

// Load settings
async function loadSettings() {
  try {
    const settings = await getSettings();

    if (settings['cli.nodejs']) {
      cliPaths.value.nodejs = settings['cli.nodejs'];
    }
    if (settings['cli.python']) {
      cliPaths.value.python = settings['cli.python'];
    }
    if (settings['cli.git']) {
      cliPaths.value.git = settings['cli.git'];
    }
  } catch (error) {
    console.error('Failed to load CLI paths:', error);
    ElMessage.warning('加载设置失败，使用默认值');
  }
}

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="cli-paths-settings">
    <div class="settings-header">
      <h2 class="text-2xl font-bold">CLI 工具路径设置</h2>
      <p class="text-gray-500 mt-2">配置常用命令行工具的路径</p>
    </div>

    <div class="settings-content bg-surface rounded-lg border border-border p-6 shadow-sm mt-6">
      <ElForm label-width="120px">
        <ElFormItem label="Node.js">
          <ElInput
            v-model="cliPaths.nodejs"
            placeholder="例: /usr/bin/node 或 node"
          />
          <div class="text-sm text-gray-500 mt-1">Node.js 可执行文件路径</div>
        </ElFormItem>

        <ElFormItem label="Python">
          <ElInput
            v-model="cliPaths.python"
            placeholder="例: /usr/bin/python3 或 python"
          />
          <div class="text-sm text-gray-500 mt-1">Python 可执行文件路径</div>
        </ElFormItem>

        <ElFormItem label="Git">
          <ElInput
            v-model="cliPaths.git"
            placeholder="例: /usr/bin/git 或 git"
          />
          <div class="text-sm text-gray-500 mt-1">Git 可执行文件路径</div>
        </ElFormItem>
      </ElForm>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center justify-end space-x-4 mt-6">
      <ElButton @click="resetSettings">
        恢复默认
      </ElButton>
      <ElButton
        type="primary"
        @click="saveSettings"
      >
        保存设置
      </ElButton>
    </div>
  </div>
</template>

<style scoped>
.cli-paths-settings {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.settings-header {
  border-bottom: 1px solid var(--color-border);
  padding-bottom: 16px;
}

:deep(.el-form-item) {
  margin-bottom: 24px;
}
</style>
