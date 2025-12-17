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
  ElSwitch,
  ElMessage,
} from 'element-plus';
import { ref, onMounted } from 'vue';
import { getSettings, saveSettings as saveTauriSettings } from '@/services/tauri/commands';

// Environment variables
const envVars = ref([
  { name: 'API_KEY', value: 'sk-...****************', isSecret: true },
  { name: 'PATH', value: '/usr/local/bin:/usr/bin:/bin', isSecret: false },
]);

const newEnvVar = ref({ name: '', value: '', isSecret: false });
const showEnvVarDialog = ref(false);
const editingEnvVarIndex = ref<number | null>(null);

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

// Save settings
async function saveSettings() {
  try {
    const settingsToSave: Record<string, any> = {};

    // 保存环境变量
    envVars.value.forEach((envVar) => {
      settingsToSave[`env.${envVar.name}`] = JSON.stringify({
        value: envVar.value,
        isSecret: envVar.isSecret,
      });
    });

    await saveTauriSettings(settingsToSave);
    ElMessage.success('环境变量已保存');
  } catch (error) {
    console.error('Failed to save environment variables:', error);
    ElMessage.error('保存失败: ' + (error as Error).message);
  }
}

// Load settings
async function loadSettings() {
  try {
    const settings = await getSettings();

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
  } catch (error) {
    console.error('Failed to load environment variables:', error);
    ElMessage.warning('加载设置失败，使用默认值');
  }
}

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="environment-variables">
    <div class="settings-header">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-2xl font-bold">环境变量设置</h2>
          <p class="text-gray-500 mt-2">管理应用程序使用的环境变量</p>
        </div>
        <ElButton
          type="primary"
          :icon="Plus"
          @click="openEnvVarDialog"
        >
          添加环境变量
        </ElButton>
      </div>
    </div>

    <div class="settings-content bg-surface rounded-lg border border-border p-6 shadow-sm mt-6">
      <ElTable
        v-if="envVars.length > 0"
        :data="envVars"
        style="width: 100%"
      >
        <ElTableColumn
          prop="name"
          label="变量名"
          width="200"
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
          label="敏感信息"
          width="100"
          align="center"
        >
          <template #default="{ row }">
            <span v-if="row.isSecret" class="text-green-600">是</span>
            <span v-else class="text-gray-400">否</span>
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
              @click="editEnvVar($index)"
            >
              编辑
            </ElButton>
            <ElButton
              size="small"
              type="danger"
              :icon="Delete"
              text
              @click="removeEnvVar($index)"
            >
              删除
            </ElButton>
          </template>
        </ElTableColumn>
      </ElTable>
      <div v-else class="empty-state">
        <p class="text-gray-500 text-center py-12">暂无环境变量，点击右上方按钮添加</p>
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
      v-model="showEnvVarDialog"
      :title="editingEnvVarIndex !== null ? '编辑环境变量' : '添加环境变量'"
      width="500px"
    >
      <ElForm
        :model="newEnvVar"
        label-width="100px"
      >
        <ElFormItem label="变量名">
          <ElInput v-model="newEnvVar.name" placeholder="例: API_KEY" />
        </ElFormItem>
        <ElFormItem label="变量值">
          <ElInput v-model="newEnvVar.value" placeholder="变量的值" />
        </ElFormItem>
        <ElFormItem label="敏感信息">
          <ElSwitch v-model="newEnvVar.isSecret" />
          <span class="text-sm text-gray-500 ml-2">开启后将隐藏显示</span>
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
  </div>
</template>

<style scoped>
.environment-variables {
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
