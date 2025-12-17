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

// AI models
const aiModels = ref([
  { name: 'Claude-3.5-Sonnet', endpoint: 'api.anthropic.com/v1', apiKey: '********' },
  { name: 'GPT-4', endpoint: 'api.openai.com/v1', apiKey: '********' },
]);

const newAiModel = ref({ name: '', endpoint: '', apiKey: '' });
const showAiModelDialog = ref(false);
const editingAiModelIndex = ref<number | null>(null);

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

// Save settings
async function saveSettings() {
  try {
    const settingsToSave: Record<string, any> = {};

    // 保存 AI 模型配置
    aiModels.value.forEach((model, index) => {
      settingsToSave[`ai.models.${index}`] = JSON.stringify({
        name: model.name,
        endpoint: model.endpoint,
        apiKey: model.apiKey,
      });
    });

    await saveTauriSettings(settingsToSave);
    ElMessage.success('AI 模型配置已保存');
  } catch (error) {
    console.error('Failed to save AI models:', error);
    ElMessage.error('保存失败: ' + (error as Error).message);
  }
}

// Load settings
async function loadSettings() {
  try {
    const settings = await getSettings();

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
  } catch (error) {
    console.error('Failed to load AI models:', error);
    ElMessage.warning('加载设置失败，使用默认值');
  }
}

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="ai-models-management">
    <div class="settings-header">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-2xl font-bold">AI 模型管理</h2>
          <p class="text-gray-500 mt-2">配置和管理 AI 模型连接信息</p>
        </div>
        <ElButton
          type="primary"
          :icon="Plus"
          @click="openAiModelDialog"
        >
          添加模型
        </ElButton>
      </div>
    </div>

    <div class="settings-content bg-surface rounded-lg border border-border p-6 shadow-sm mt-6">
      <ElTable
        v-if="aiModels.length > 0"
        :data="aiModels"
        style="width: 100%"
      >
        <ElTableColumn
          prop="name"
          label="模型名称"
          width="200"
        />
        <ElTableColumn
          prop="endpoint"
          label="API端点"
        />
        <ElTableColumn
          prop="apiKey"
          label="API密钥"
          width="150"
        >
          <template #default>
            <span>********</span>
          </template>
        </ElTableColumn>
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
              @click="editAiModel($index)"
            >
              编辑
            </ElButton>
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
        <p class="text-gray-500 text-center py-12">暂无AI模型，点击右上方按钮添加</p>
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
      v-model="showAiModelDialog"
      :title="editingAiModelIndex !== null ? '编辑 AI 模型' : '添加 AI 模型'"
      width="500px"
    >
      <ElForm
        :model="newAiModel"
        label-width="100px"
      >
        <ElFormItem label="模型名称">
          <ElInput v-model="newAiModel.name" placeholder="例: Claude-3.5-Sonnet" />
        </ElFormItem>
        <ElFormItem label="API端点">
          <ElInput v-model="newAiModel.endpoint" placeholder="例: api.anthropic.com/v1" />
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
  </div>
</template>

<style scoped>
.ai-models-management {
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
