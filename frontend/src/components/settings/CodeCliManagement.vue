<script setup lang="ts">
import { Plus, Delete, Edit } from '@element-plus/icons-vue';
import {
  ElButton,
  ElTable,
  ElTableColumn,
  ElDialog,
  ElForm,
  ElFormItem,
  ElInput,
  ElMessage,
} from 'element-plus';
import { ref, onMounted } from 'vue';
import { getSettings, saveSettings as saveTauriSettings } from '@/services/tauri/commands';

// Code CLIs
const codeClis = ref([
  { name: 'OpenAI-Codex', command: '/usr/bin/codex', args: '--model gpt-4' },
  { name: 'Local-Coder', command: '/usr/local/bin/coder', args: '--local' },
]);

const newCodeCli = ref({ name: '', command: '', args: '' });
const showCodeCliDialog = ref(false);
const editingCodeCliIndex = ref<number | null>(null);

// Code CLI operations
function addCodeCli() {
  if (!newCodeCli.value.name.trim()) {
    ElMessage.warning('请输入CLI名称');
    return;
  }
  if (!newCodeCli.value.command.trim()) {
    ElMessage.warning('请输入命令路径');
    return;
  }

  if (editingCodeCliIndex.value !== null) {
    // 编辑模式
    codeClis.value[editingCodeCliIndex.value] = { ...newCodeCli.value };
    ElMessage.success('Code CLI已更新');
    editingCodeCliIndex.value = null;
  } else {
    // 新增模式
    codeClis.value.push({ ...newCodeCli.value });
    ElMessage.success('Code CLI已添加');
  }
  newCodeCli.value = { name: '', command: '', args: '' };
  showCodeCliDialog.value = false;
}

function editCodeCli(index: number) {
  const codeCli = codeClis.value[index];
  if (codeCli) {
    editingCodeCliIndex.value = index;
    newCodeCli.value = {
      name: codeCli.name,
      command: codeCli.command,
      args: codeCli.args,
    };
    showCodeCliDialog.value = true;
  }
}

function removeCodeCli(index: number) {
  const codeCli = codeClis.value[index];
  if (confirm(`确定要删除 Code CLI "${codeCli?.name}" 吗？`)) {
    codeClis.value.splice(index, 1);
    ElMessage.success('Code CLI已删除');
  }
}

function openCodeCliDialog() {
  editingCodeCliIndex.value = null;
  newCodeCli.value = { name: '', command: '', args: '' };
  showCodeCliDialog.value = true;
}

// Save settings
async function saveSettings() {
  try {
    const settingsToSave: Record<string, any> = {};

    // 保存 Code CLI 配置
    codeClis.value.forEach((cli, index) => {
      settingsToSave[`cli.code.${index}`] = JSON.stringify({
        name: cli.name,
        command: cli.command,
        args: cli.args,
      });
    });

    await saveTauriSettings(settingsToSave);
    ElMessage.success('Code CLI 配置已保存');
  } catch (error) {
    console.error('Failed to save Code CLIs:', error);
    ElMessage.error('保存失败: ' + (error as Error).message);
  }
}

// Load settings
async function loadSettings() {
  try {
    const settings = await getSettings();

    // 加载 Code CLI 配置
    const loadedCodeClis: typeof codeClis.value = [];
    Object.keys(settings).forEach(key => {
      if (key.startsWith('cli.code.')) {
        const cliData = typeof settings[key] === 'string'
          ? JSON.parse(settings[key])
          : settings[key];
        loadedCodeClis.push({
          name: cliData.name || '',
          command: cliData.command || '',
          args: cliData.args || '',
        });
      }
    });
    if (loadedCodeClis.length > 0) {
      codeClis.value = loadedCodeClis;
    }
  } catch (error) {
    console.error('Failed to load Code CLIs:', error);
    ElMessage.warning('加载设置失败，使用默认值');
  }
}

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="code-cli-management">
    <div class="settings-header">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-2xl font-bold">Code CLI 管理</h2>
          <p class="text-gray-500 mt-2">配置和管理代码辅助 CLI 工具</p>
        </div>
        <ElButton
          type="primary"
          :icon="Plus"
          @click="openCodeCliDialog"
        >
          添加 Code CLI
        </ElButton>
      </div>
    </div>

    <div class="settings-content bg-surface rounded-lg border border-border p-6 shadow-sm mt-6">
      <ElTable
        v-if="codeClis.length > 0"
        :data="codeClis"
        style="width: 100%"
      >
        <ElTableColumn
          prop="name"
          label="CLI名称"
          width="200"
        />
        <ElTableColumn
          prop="command"
          label="命令路径"
        />
        <ElTableColumn
          prop="args"
          label="参数"
          width="200"
        />
        <ElTableColumn
          label="操作"
          width="150"
          align="center"
        >
          <template #default="{ $index }">
            <ElButton
              size="small"
              :icon="Edit"
              text
              @click="editCodeCli($index)"
            >
              编辑
            </ElButton>
            <ElButton
              size="small"
              type="danger"
              :icon="Delete"
              text
              @click="removeCodeCli($index)"
            />
          </template>
        </ElTableColumn>
      </ElTable>
      <div v-else class="empty-state">
        <p class="text-gray-500 text-center py-12">暂无Code CLI，点击右上方按钮添加</p>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center justify-end space-x-4 mt-6">
      <ElButton
        type="primary"
        @click="saveSettings"
      >
        保存设置
      </ElButton>
    </div>

    <!-- Dialog -->
    <ElDialog
      v-model="showCodeCliDialog"
      :title="editingCodeCliIndex !== null ? '编辑 Code CLI' : '添加 Code CLI'"
      width="500px"
    >
      <ElForm
        :model="newCodeCli"
        label-width="100px"
      >
        <ElFormItem label="CLI名称">
          <ElInput v-model="newCodeCli.name" placeholder="例: OpenAI-Codex" />
        </ElFormItem>
        <ElFormItem label="命令路径">
          <ElInput v-model="newCodeCli.command" placeholder="例: /usr/bin/codex" />
        </ElFormItem>
        <ElFormItem label="参数">
          <ElInput v-model="newCodeCli.args" placeholder="例: --model gpt-4 (可选)" />
        </ElFormItem>
      </ElForm>
      <template #footer>
        <span class="dialog-footer">
          <ElButton @click="showCodeCliDialog = false">取消</ElButton>
          <ElButton
            type="primary"
            @click="addCodeCli"
          >确定</ElButton>
        </span>
      </template>
    </ElDialog>
  </div>
</template>

<style scoped>
.code-cli-management {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.settings-header {
  border-bottom: 1px solid var(--color-border);
  padding-bottom: 16px;
}

.empty-state {
  border: 1px dashed var(--color-border);
  border-radius: 4px;
  background-color: rgba(0, 0, 0, 0.02);
}

:deep(.el-table) {
  background: transparent;
}

:deep(.el-table th) {
  background-color: var(--color-surface);
}
</style>
