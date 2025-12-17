<script setup lang="ts">
import { Folder, Plus, Delete, Edit } from '@element-plus/icons-vue';
import {
  ElForm,
  ElFormItem,
  ElInput,
  ElButton,
  ElSwitch,
  ElTable,
  ElTableColumn,
  ElDialog,
  ElMessage,
} from 'element-plus';
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { open as openDialog } from '@tauri-apps/plugin-dialog';

import { useAppStore } from '@/stores/workspaceStore';
import { useFileStore } from '@/stores/filesStore';
import { getSettings, saveSettings as saveTauriSettings, addRecentDirectory } from '@/services/tauri/commands';

const router = useRouter();
const fileStore = useFileStore();

const appStore = useAppStore();

// Workspace settings
const dataDirectory = ref('~/.code-ai-assistant');

// CLI tool paths
const cliPaths = ref({
  nodejs: '/usr/bin/node',
  python: '/usr/bin/python3',
  git: '/usr/bin/git',
});

// Environment variables
const envVars = ref([
  { name: 'API_KEY', value: 'sk-...****************', isSecret: true },
  { name: 'PATH', value: '/usr/local/bin:/usr/bin:/bin', isSecret: false },
]);

const newEnvVar = ref({ name: '', value: '', isSecret: false });
const showEnvVarDialog = ref(false);
const editingEnvVarIndex = ref<number | null>(null);

// AI models
const aiModels = ref([
  { name: 'Claude-3.5-Sonnet', endpoint: 'api.anthropic.com/v1', apiKey: '********' },
  { name: 'GPT-4', endpoint: 'api.openai.com/v1', apiKey: '********' },
]);

const newAiModel = ref({ name: '', endpoint: '', apiKey: '' });
const showAiModelDialog = ref(false);
const editingAiModelIndex = ref<number | null>(null);

// Code CLIs
const codeClis = ref([
  { name: 'OpenAI-Codex', command: '/usr/bin/codex', args: '--model gpt-4' },
  { name: 'Local-Coder', command: '/usr/local/bin/coder', args: '--local' },
]);

const newCodeCli = ref({ name: '', command: '', args: '' });
const showCodeCliDialog = ref(false);
const editingCodeCliIndex = ref<number | null>(null);

// Save settings
async function saveSettings() {
  try {
    // 构建要保存的设置对象
    const settingsToSave: Record<string, any> = {
      'user.theme': appStore.settings.theme,
      'user.fontSize': appStore.settings.fontSize,
      'user.autoSave': appStore.settings.autoSave,
      'workspace.current': appStore.currentWorkspace,
      'workspace.dataDirectory': dataDirectory.value,
      'cli.nodejs': cliPaths.value.nodejs,
      'cli.python': cliPaths.value.python,
      'cli.git': cliPaths.value.git,
    };

    // 保存环境变量
    envVars.value.forEach((envVar) => {
      settingsToSave[`env.${envVar.name}`] = JSON.stringify({
        value: envVar.value,
        isSecret: envVar.isSecret,
      });
    });

    // 保存 AI 模型配置
    aiModels.value.forEach((model, index) => {
      settingsToSave[`ai.models.${index}`] = JSON.stringify({
        name: model.name,
        endpoint: model.endpoint,
        apiKey: model.apiKey,
      });
    });

    // 保存 Code CLI 配置
    codeClis.value.forEach((cli, index) => {
      settingsToSave[`cli.code.${index}`] = JSON.stringify({
        name: cli.name,
        command: cli.command,
        args: cli.args,
      });
    });

    await saveTauriSettings(settingsToSave);
    await appStore.saveSettings();
    ElMessage.success('设置已保存');
  } catch (error) {
    console.error('Failed to save settings:', error);
    ElMessage.error('保存设置失败: ' + (error as Error).message);
  }
}

// Reset to defaults
async function resetSettings() {
  if (confirm('确定要重置所有设置为默认值吗？')) {
    try {
      // 调用后端重置设置
      await saveTauriSettings({});

      // Reset to default settings
      appStore.settings.theme = 'light';
      appStore.settings.fontSize = 14;
      appStore.settings.autoSave = true;
      dataDirectory.value = '~/.code-ai-assistant';
      cliPaths.value = {
        nodejs: '/usr/bin/node',
        python: '/usr/bin/python3',
        git: '/usr/bin/git',
      };
      envVars.value = [];
      aiModels.value = [];
      codeClis.value = [];

      await appStore.saveSettings();
      await loadSettings();
      ElMessage.success('设置已重置为默认值');
    } catch (error) {
      console.error('Failed to reset settings:', error);
      ElMessage.error('重置设置失败: ' + (error as Error).message);
    }
  }
}

// Environment variable operations
function addEnvVar() {
  if (!newEnvVar.value.name.trim()) {
    ElMessage.warning('请输入变量名');
    return;
  }
  if (!newEnvVar.value.value.trim()) {
    ElMessage.warning('请输入变量值');
    return;
  }

  if (editingEnvVarIndex.value !== null) {
    // 编辑模式
    envVars.value[editingEnvVarIndex.value] = { ...newEnvVar.value };
    ElMessage.success('环境变量已更新');
    editingEnvVarIndex.value = null;
  } else {
    // 新增模式
    envVars.value.push({ ...newEnvVar.value });
    ElMessage.success('环境变量已添加');
  }
  newEnvVar.value = { name: '', value: '', isSecret: false };
  showEnvVarDialog.value = false;
}

function editEnvVar(index: number) {
  const envVar = envVars.value[index];
  if (envVar) {
    editingEnvVarIndex.value = index;
    newEnvVar.value = {
      name: envVar.name,
      value: envVar.value,
      isSecret: envVar.isSecret,
    };
    showEnvVarDialog.value = true;
  }
}

function removeEnvVar(index: number) {
  const envVar = envVars.value[index];
  if (confirm(`确定要删除环境变量 "${envVar?.name}" 吗？`)) {
    envVars.value.splice(index, 1);
    ElMessage.success('环境变量已删除');
  }
}

function openEnvVarDialog() {
  editingEnvVarIndex.value = null;
  newEnvVar.value = { name: '', value: '', isSecret: false };
  showEnvVarDialog.value = true;
}

// AI model operations
function addAiModel() {
  if (!newAiModel.value.name.trim()) {
    ElMessage.warning('请输入模型名称');
    return;
  }
  if (!newAiModel.value.endpoint.trim()) {
    ElMessage.warning('请输入API端点');
    return;
  }

  if (editingAiModelIndex.value !== null) {
    // 编辑模式
    const currentModel = aiModels.value[editingAiModelIndex.value];
    if (currentModel) {
      aiModels.value[editingAiModelIndex.value] = {
        ...newAiModel.value,
        // 如果没有输入新密钥，保留原密钥
        apiKey: newAiModel.value.apiKey || currentModel.apiKey,
      };
      ElMessage.success('模型已更新');
    }
    editingAiModelIndex.value = null;
  } else {
    // 新增模式
    if (!newAiModel.value.apiKey.trim()) {
      ElMessage.warning('请输入API密钥');
      return;
    }
    aiModels.value.push({ ...newAiModel.value });
    ElMessage.success('模型已添加');
  }
  newAiModel.value = { name: '', endpoint: '', apiKey: '' };
  showAiModelDialog.value = false;
}

function editAiModel(index: number) {
  const model = aiModels.value[index];
  if (model) {
    editingAiModelIndex.value = index;
    newAiModel.value = {
      name: model.name,
      endpoint: model.endpoint,
      apiKey: '', // 编辑时不显示原密钥
    };
    showAiModelDialog.value = true;
  }
}

function removeAiModel(index: number) {
  const model = aiModels.value[index];
  if (confirm(`确定要删除模型 "${model?.name}" 吗？`)) {
    aiModels.value.splice(index, 1);
    ElMessage.success('模型已删除');
  }
}

function openAiModelDialog() {
  editingAiModelIndex.value = null;
  newAiModel.value = { name: '', endpoint: '', apiKey: '' };
  showAiModelDialog.value = true;
}

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

// 选择工作目录
async function selectWorkspaceDirectory() {
  try {
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: '选择工作目录',
    });

    if (result && typeof result === 'string') {
      // 加载目录
      await fileStore.loadDirectory(result);

      // 保存到最近目录
      await addRecentDirectory(result);

      ElMessage.success('工作目录已设置: ' + result);

      // 跳转到 dashboard
      router.push('/dashboard');
    }
  } catch (error) {
    ElMessage.error('选择目录失败: ' + (error as Error).message);
    console.error('选择目录失败', error);
  }
}

// 从后端加载设置
async function loadSettings() {
  try {
    const settings = await getSettings();

    // 加载应用设置
    if (settings['user.theme']) {
      appStore.settings.theme = settings['user.theme'];
    }
    if (settings['user.fontSize']) {
      appStore.settings.fontSize = Number(settings['user.fontSize']);
    }
    if (settings['user.autoSave'] !== undefined) {
      appStore.settings.autoSave = settings['user.autoSave'];
    }
    if (settings['workspace.current']) {
      appStore.currentWorkspace = settings['workspace.current'];
    }
    if (settings['workspace.dataDirectory']) {
      dataDirectory.value = settings['workspace.dataDirectory'];
    }

    // 加载 CLI 路径
    if (settings['cli.nodejs']) {
      cliPaths.value.nodejs = settings['cli.nodejs'];
    }
    if (settings['cli.python']) {
      cliPaths.value.python = settings['cli.python'];
    }
    if (settings['cli.git']) {
      cliPaths.value.git = settings['cli.git'];
    }

    // 加载环境变量
    const loadedEnvVars: typeof envVars.value = [];
    Object.keys(settings).forEach(key => {
      if (key.startsWith('env.')) {
        const envName = key.substring(4);
        const envData = typeof settings[key] === 'string'
          ? JSON.parse(settings[key])
          : settings[key];
        loadedEnvVars.push({
          name: envName,
          value: envData.value || '',
          isSecret: envData.isSecret || false,
        });
      }
    });
    if (loadedEnvVars.length > 0) {
      envVars.value = loadedEnvVars;
    }

    // 加载 AI 模型配置
    const loadedAiModels: typeof aiModels.value = [];
    Object.keys(settings).forEach(key => {
      if (key.startsWith('ai.models.')) {
        const modelData = typeof settings[key] === 'string'
          ? JSON.parse(settings[key])
          : settings[key];
        loadedAiModels.push({
          name: modelData.name || '',
          endpoint: modelData.endpoint || '',
          apiKey: modelData.apiKey || '********',
        });
      }
    });
    if (loadedAiModels.length > 0) {
      aiModels.value = loadedAiModels;
    }

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

    console.log('Settings loaded successfully');
  } catch (error) {
    console.error('Failed to load settings:', error);
    ElMessage.warning('从后端加载设置失败，使用默认设置');
  }
}

// 组件挂载时加载设置
onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="settings-container space-y-4 max-h-full overflow-y-auto pb-4 pt-0">
    <!-- Workspace Settings -->
    <div class="bg-surface rounded-t-lg border-b border-border p-4 shadow-sm">
      <h2 class="text-lg font-semibold mb-3">
        工作区设置
      </h2>

      <ElForm label-width="120px">
        <ElFormItem label="选择工作目录">
          <div class="flex items-center space-x-2">
            <ElButton
              type="primary"
              :icon="Folder"
              @click="selectWorkspaceDirectory"
            >
              选择目录
            </ElButton>
            <span v-if="fileStore.currentDirectory" class="text-sm text-gray-500">
              当前: {{ fileStore.currentDirectory }}
            </span>
          </div>
        </ElFormItem>
      </ElForm>
    </div>

    <!-- CLI Tool Paths -->
    <div class="bg-surface border-b border-border p-4 shadow-sm">
      <h2 class="text-lg font-semibold mb-3">
        CLI 工具路径设置
      </h2>

      <ElForm label-width="120px">
        <ElFormItem label="Node.js">
          <ElInput
            v-model="cliPaths.nodejs"
            placeholder="例: /usr/bin/node 或 node"
          />
        </ElFormItem>

        <ElFormItem label="Python">
          <ElInput
            v-model="cliPaths.python"
            placeholder="例: /usr/bin/python3 或 python"
          />
        </ElFormItem>

        <ElFormItem label="Git">
          <ElInput
            v-model="cliPaths.git"
            placeholder="例: /usr/bin/git 或 git"
          />
        </ElFormItem>
      </ElForm>
    </div>

    <!-- Environment Variables -->
    <div class="bg-surface border-b border-border p-4 shadow-sm">
      <div class="flex items-center justify-between mb-3">
        <h2 class="text-lg font-semibold">
          环境变量设置
        </h2>
        <ElButton
          type="primary"
          :icon="Plus"
          @click="openEnvVarDialog"
        >
          添加环境变量
        </ElButton>
      </div>

      <ElTable
        v-if="envVars.length > 0"
        :data="envVars"
        style="width: 100%"
      >
        <ElTableColumn
          prop="name"
          label="变量名"
          width="150"
        />
        <ElTableColumn
          prop="value"
          label="变量值"
        >
          <template #default="{ row }">
            <span v-if="row.isSecret">********</span>
            <span v-else>{{ row.value }}</span>
          </template>
        </ElTableColumn>
        <ElTableColumn
          label="操作"
          width="120"
        >
          <template #default="{ $index }">
            <ElButton
              size="small"
              :icon="Edit"
              text
              @click="editEnvVar($index)"
            />
            <ElButton
              size="small"
              type="danger"
              :icon="Delete"
              text
              @click="removeEnvVar($index)"
            />
          </template>
        </ElTableColumn>
      </ElTable>
      <div v-else class="empty-state">
        <p class="text-gray-500 text-center py-8">暂无环境变量，点击上方按钮添加</p>
      </div>
    </div>

    <!-- AI Models -->
    <div class="bg-surface border-b border-border p-4 shadow-sm">
      <div class="flex items-center justify-between mb-3">
        <h2 class="text-lg font-semibold">
          模型管理
        </h2>
        <ElButton
          type="primary"
          :icon="Plus"
          @click="openAiModelDialog"
        >
          添加模型
        </ElButton>
      </div>

      <ElTable
        v-if="aiModels.length > 0"
        :data="aiModels"
        style="width: 100%"
      >

        <ElTableColumn
          prop="name"
          label="模型名称"
          width="180"
        />
        <ElTableColumn
          prop="endpoint"
          label="API端点"
        />
        <ElTableColumn
          prop="apiKey"
          label="API密钥"
        >
          <template #default>
            <span>********</span>
          </template>
        </ElTableColumn>
        <ElTableColumn
          label="操作"
          width="120"
        >
          <template #default="{ $index }">
            <ElButton
              size="small"
              :icon="Edit"
              text
              @click="editAiModel($index)"
            />
            <ElButton
              size="small"
              type="danger"
              :icon="Delete"
              text
              @click="removeAiModel($index)"
            />
          </template>
        </ElTableColumn>
      </ElTable>
      <div v-else class="empty-state">
        <p class="text-gray-500 text-center py-8">暂无AI模型，点击上方按钮添加</p>
      </div>
    </div>

    <!-- Code CLIs -->
    <div class="bg-surface rounded-b-lg p-4 shadow-sm">
      <div class="flex items-center justify-between mb-3">
        <h2 class="text-lg font-semibold">
          Code CLI 管理
        </h2>
        <ElButton
          type="primary"
          :icon="Plus"
          @click="openCodeCliDialog"
        >
          添加 Code CLI
        </ElButton>
      </div>

      <ElTable
        v-if="codeClis.length > 0"
        :data="codeClis"
        style="width: 100%"
      >
        <ElTableColumn
          prop="name"
          label="CLI名称"
          width="180"
        />
        <ElTableColumn
          prop="command"
          label="命令路径"
        />
        <ElTableColumn
          prop="args"
          label="参数"
        />
        <ElTableColumn
          label="操作"
          width="120"
        >
          <template #default="{ $index }">
            <ElButton
              size="small"
              :icon="Edit"
              text
              @click="editCodeCli($index)"
            />
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
        <p class="text-gray-500 text-center py-8">暂无Code CLI，点击上方按钮添加</p>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center justify-end space-x-4">
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

    <!-- Dialogs -->
    <ElDialog
      v-model="showEnvVarDialog"
      :title="editingEnvVarIndex !== null ? '编辑环境变量' : '添加环境变量'"
      width="500px"
    >
      <ElForm
        :model="newEnvVar"
        label-width="100px"
      >
        <ElFormItem label="变量名">
          <ElInput v-model="newEnvVar.name" />
        </ElFormItem>
        <ElFormItem label="变量值">
          <ElInput v-model="newEnvVar.value" />
        </ElFormItem>
        <ElFormItem label="敏感信息">
          <ElSwitch v-model="newEnvVar.isSecret" />
        </ElFormItem>
      </ElForm>
      <template #footer>
        <span class="dialog-footer">
          <ElButton @click="showEnvVarDialog = false">取消</ElButton>
          <ElButton
            type="primary"
            @click="addEnvVar"
          >确定</ElButton>
        </span>
      </template>
    </ElDialog>

    <ElDialog
      v-model="showAiModelDialog"
      :title="editingAiModelIndex !== null ? '编辑 AI 模型' : '添加 AI 模型'"
      width="500px"
    >
      <ElForm
        :model="newAiModel"
        label-width="100px"
      >
        <ElFormItem label="模型名称">
          <ElInput v-model="newAiModel.name" />
        </ElFormItem>
        <ElFormItem label="API端点">
          <ElInput v-model="newAiModel.endpoint" />
        </ElFormItem>
        <ElFormItem label="API密钥">
          <ElInput
            v-model="newAiModel.apiKey"
            type="password"
            show-password
            :placeholder="editingAiModelIndex !== null ? '留空则保持原密钥不变' : ''"
          />
        </ElFormItem>
      </ElForm>
      <template #footer>
        <span class="dialog-footer">
          <ElButton @click="showAiModelDialog = false">取消</ElButton>
          <ElButton
            type="primary"
            @click="addAiModel"
          >确定</ElButton>
        </span>
      </template>
    </ElDialog>

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
          <ElInput v-model="newCodeCli.name" />
        </ElFormItem>
        <ElFormItem label="命令路径">
          <ElInput v-model="newCodeCli.command" />
        </ElFormItem>
        <ElFormItem label="参数">
          <ElInput v-model="newCodeCli.args" />
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
:deep(.el-form-item) {
  margin-bottom: 12px;
}

:deep(.el-table) {
  background: transparent;
}

:deep(.el-table th) {
  background-color: var(--color-surface);
}

/* 空状态样式 */
.empty-state {
  border: 1px dashed var(--color-border);
  border-radius: 4px;
  background-color: rgba(0, 0, 0, 0.02);
}

/* 自定义滚动条样式 */
.settings-container {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
  scroll-behavior: smooth;
}

.settings-container::-webkit-scrollbar {
  width: 8px;
}

.settings-container::-webkit-scrollbar-track {
  background: transparent;
}

.settings-container::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 4px;
}

.settings-container::-webkit-scrollbar-thumb:hover {
  background-color: rgba(156, 163, 175, 0.8);
}
</style>
