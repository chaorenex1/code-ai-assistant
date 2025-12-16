<script setup lang="ts">
import { ref } from 'vue'
import { Folder, Refresh, Plus, Delete, Edit } from '@element-plus/icons-vue'
import { ElForm, ElFormItem, ElInput, ElSelect, ElOption, ElButton, ElSwitch, ElTable, ElTableColumn, ElDialog } from 'element-plus'
import { useAppStore } from '../../stores/app'

const appStore = useAppStore()

// Workspace settings
const workspaceName = ref('')
const dataDirectory = ref('~/.code-ai-assistant')

// CLI tool paths
const cliPaths = ref({
  nodejs: '/usr/bin/node',
  python: '/usr/bin/python3',
  git: '/usr/bin/git',
})

// Environment variables
const envVars = ref([
  { name: 'API_KEY', value: 'sk-...****************', isSecret: true },
  { name: 'PATH', value: '/usr/local/bin:/usr/bin:/bin', isSecret: false },
])

const newEnvVar = ref({ name: '', value: '', isSecret: false })
const showEnvVarDialog = ref(false)

// AI models
const aiModels = ref([
  { name: 'Claude-3.5-Sonnet', endpoint: 'api.anthropic.com/v1', apiKey: '********' },
  { name: 'GPT-4', endpoint: 'api.openai.com/v1', apiKey: '********' },
])

const newAiModel = ref({ name: '', endpoint: '', apiKey: '' })
const showAiModelDialog = ref(false)

// Code CLIs
const codeClis = ref([
  { name: 'OpenAI-Codex', command: '/usr/bin/codex', args: '--model gpt-4' },
  { name: 'Local-Coder', command: '/usr/local/bin/coder', args: '--local' },
])

const newCodeCli = ref({ name: '', command: '', args: '' })
const showCodeCliDialog = ref(false)

// Save settings
async function saveSettings() {
  try {
    await appStore.saveSettings()
    // TODO: Save other settings
  } catch (error) {
    console.error('Failed to save settings:', error)
  }
}

// Reset to defaults
async function resetSettings() {
  if (confirm('确定要重置所有设置为默认值吗？')) {
    try {
      // Reset to default settings
      appStore.settings.theme = 'light'
      appStore.settings.fontSize = 14
      appStore.settings.autoSave = true
      await appStore.saveSettings()
      // TODO: Reset other settings
    } catch (error) {
      console.error('Failed to reset settings:', error)
    }
  }
}

// Workspace operations
async function createWorkspace() {
  if (workspaceName.value.trim()) {
    try {
      await appStore.createWorkspace(workspaceName.value.trim())
      workspaceName.value = ''
    } catch (error) {
      console.error('Failed to create workspace:', error)
    }
  }
}

async function deleteWorkspace(name: string) {
  if (confirm(`确定要删除工作区 "${name}" 吗？`)) {
    try {
      await appStore.deleteWorkspace(name)
    } catch (error) {
      console.error('Failed to delete workspace:', error)
    }
  }
}

// Environment variable operations
function addEnvVar() {
  if (newEnvVar.value.name.trim() && newEnvVar.value.value.trim()) {
    envVars.value.push({ ...newEnvVar.value })
    newEnvVar.value = { name: '', value: '', isSecret: false }
    showEnvVarDialog.value = false
  }
}

function removeEnvVar(index: number) {
  envVars.value.splice(index, 1)
}

// AI model operations
function addAiModel() {
  if (newAiModel.value.name.trim() && newAiModel.value.endpoint.trim()) {
    aiModels.value.push({ ...newAiModel.value, apiKey: '********' })
    newAiModel.value = { name: '', endpoint: '', apiKey: '' }
    showAiModelDialog.value = false
  }
}

function removeAiModel(index: number) {
  aiModels.value.splice(index, 1)
}

// Code CLI operations
function addCodeCli() {
  if (newCodeCli.value.name.trim() && newCodeCli.value.command.trim()) {
    codeClis.value.push({ ...newCodeCli.value })
    newCodeCli.value = { name: '', command: '', args: '' }
    showCodeCliDialog.value = false
  }
}

function removeCodeCli(index: number) {
  codeClis.value.splice(index, 1)
}

// Browse directory
function browseDirectory() {
  // TODO: Implement directory browser
  console.log('Browse directory')
}
</script>

<template>
  <div class="space-y-6">
    <!-- Workspace Settings -->
    <div class="bg-surface rounded-lg p-6">
      <h2 class="text-lg font-semibold mb-4">工作区设置</h2>

      <ElForm label-width="120px">
        <ElFormItem label="当前工作区">
          <div class="flex items-center space-x-2">
            <ElSelect v-model="appStore.currentWorkspace" style="width: 200px">
              <ElOption
                v-for="workspace in appStore.workspaces"
                :key="workspace.id"
                :label="workspace.name"
                :value="workspace.name"
              />
            </ElSelect>

            <ElInput
              v-model="workspaceName"
              placeholder="新工作区名称"
              style="width: 200px"
            />

            <ElButton type="primary" :icon="Plus" @click="createWorkspace">
              新建
            </ElButton>

            <ElButton
              v-if="appStore.currentWorkspace !== 'default'"
              type="danger"
              :icon="Delete"
              @click="deleteWorkspace(appStore.currentWorkspace)"
            >
              删除
            </ElButton>
          </div>
        </ElFormItem>

        <ElFormItem label="应用数据目录">
          <div class="flex items-center space-x-2">
            <ElInput v-model="dataDirectory" readonly style="width: 300px" />
            <ElButton :icon="Folder" @click="browseDirectory">
              浏览...
            </ElButton>
            <ElButton :icon="Refresh">
              重置为默认
            </ElButton>
          </div>
        </ElFormItem>
      </ElForm>
    </div>

    <!-- CLI Tool Paths -->
    <div class="bg-surface rounded-lg p-6">
      <h2 class="text-lg font-semibold mb-4">CLI 工具路径设置</h2>

      <ElForm label-width="120px">
        <ElFormItem label="Node.js">
          <div class="flex items-center space-x-2">
            <ElInput v-model="cliPaths.nodejs" style="width: 300px" />
            <ElButton :icon="Folder" @click="browseDirectory">
              浏览...
            </ElButton>
          </div>
        </ElFormItem>

        <ElFormItem label="Python">
          <div class="flex items-center space-x-2">
            <ElInput v-model="cliPaths.python" style="width: 300px" />
            <ElButton :icon="Folder" @click="browseDirectory">
              浏览...
            </ElButton>
          </div>
        </ElFormItem>

        <ElFormItem label="Git">
          <div class="flex items-center space-x-2">
            <ElInput v-model="cliPaths.git" style="width: 300px" />
            <ElButton :icon="Folder" @click="browseDirectory">
              浏览...
            </ElButton>
          </div>
        </ElFormItem>
      </ElForm>
    </div>

    <!-- Environment Variables -->
    <div class="bg-surface rounded-lg p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">环境变量设置</h2>
        <ElButton type="primary" :icon="Plus" @click="showEnvVarDialog = true">
          添加环境变量
        </ElButton>
      </div>

      <ElTable :data="envVars" style="width: 100%">
        <ElTableColumn prop="name" label="变量名" width="150" />
        <ElTableColumn prop="value" label="变量值">
          <template #default="{ row }">
            <span v-if="row.isSecret">********</span>
            <span v-else>{{ row.value }}</span>
          </template>
        </ElTableColumn>
        <ElTableColumn label="操作" width="120">
          <template #default="{ $index }">
            <ElButton size="small" :icon="Edit" text />
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
    </div>

    <!-- AI Models -->
    <div class="bg-surface rounded-lg p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">模型管理</h2>
        <ElButton type="primary" :icon="Plus" @click="showAiModelDialog = true">
          添加模型
        </ElButton>
      </div>

      <ElTable :data="aiModels" style="width: 100%">
        <ElTableColumn prop="name" label="模型名称" width="180" />
        <ElTableColumn prop="endpoint" label="API端点" />
        <ElTableColumn prop="apiKey" label="API密钥">
          <template #default="{ row }">
            <span>********</span>
          </template>
        </ElTableColumn>
        <ElTableColumn label="操作" width="120">
          <template #default="{ $index }">
            <ElButton size="small" :icon="Edit" text />
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
    </div>

    <!-- Code CLIs -->
    <div class="bg-surface rounded-lg p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">Code CLI 管理</h2>
        <ElButton type="primary" :icon="Plus" @click="showCodeCliDialog = true">
          添加 Code CLI
        </ElButton>
      </div>

      <ElTable :data="codeClis" style="width: 100%">
        <ElTableColumn prop="name" label="CLI名称" width="180" />
        <ElTableColumn prop="command" label="命令路径" />
        <ElTableColumn prop="args" label="参数" />
        <ElTableColumn label="操作" width="120">
          <template #default="{ $index }">
            <ElButton size="small" :icon="Edit" text />
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
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center justify-end space-x-4">
      <ElButton @click="resetSettings">
        恢复默认
      </ElButton>
      <ElButton type="primary" @click="saveSettings">
        保存设置
      </ElButton>
    </div>

    <!-- Dialogs -->
    <ElDialog v-model="showEnvVarDialog" title="添加环境变量" width="500px">
      <ElForm :model="newEnvVar" label-width="100px">
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
          <ElButton type="primary" @click="addEnvVar">确定</ElButton>
        </span>
      </template>
    </ElDialog>

    <ElDialog v-model="showAiModelDialog" title="添加 AI 模型" width="500px">
      <ElForm :model="newAiModel" label-width="100px">
        <ElFormItem label="模型名称">
          <ElInput v-model="newAiModel.name" />
        </ElFormItem>
        <ElFormItem label="API端点">
          <ElInput v-model="newAiModel.endpoint" />
        </ElFormItem>
        <ElFormItem label="API密钥">
          <ElInput v-model="newAiModel.apiKey" type="password" show-password />
        </ElFormItem>
      </ElForm>
      <template #footer>
        <span class="dialog-footer">
          <ElButton @click="showAiModelDialog = false">取消</ElButton>
          <ElButton type="primary" @click="addAiModel">确定</ElButton>
        </span>
      </template>
    </ElDialog>

    <ElDialog v-model="showCodeCliDialog" title="添加 Code CLI" width="500px">
      <ElForm :model="newCodeCli" label-width="100px">
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
          <ElButton type="primary" @click="addCodeCli">确定</ElButton>
        </span>
      </template>
    </ElDialog>
  </div>
</template>

<style scoped>
:deep(.el-form-item) {
  margin-bottom: 20px;
}

:deep(.el-table) {
  background: transparent;
}

:deep(.el-table th) {
  background-color: var(--color-surface);
}
</style>