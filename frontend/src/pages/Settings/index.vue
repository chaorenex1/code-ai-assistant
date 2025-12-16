<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import {
  ElButton,
  ElCard,
  ElForm,
  ElFormItem,
  ElInput,
  ElSwitch,
  ElSelect,
  ElOption,
  ElMessage,
  ElPageHeader,
} from 'element-plus'
import { useAppStore } from '../../stores/app'

const router = useRouter()
const appStore = useAppStore()

const settings = ref({
  theme: 'light',
  language: 'zh-CN',
  fontSize: 14,
  autoSave: true,
  aiModel: 'gpt-3.5-turbo',
  apiKey: '',
})

const themes = [
  { label: '浅色', value: 'light' },
  { label: '深色', value: 'dark' },
  { label: '自动', value: 'auto' },
]

const languages = [
  { label: '简体中文', value: 'zh-CN' },
  { label: 'English', value: 'en' },
]

const aiModels = [
  { label: 'GPT-3.5 Turbo', value: 'gpt-3.5-turbo' },
  { label: 'GPT-4', value: 'gpt-4' },
  { label: 'GPT-4 Turbo', value: 'gpt-4-turbo' },
]

function handleSave() {
  // TODO: Implement save settings logic
  ElMessage.success('设置已保存')
}

function handleReset() {
  // Reset to default settings
  settings.value = {
    theme: 'light',
    language: 'zh-CN',
    fontSize: 14,
    autoSave: true,
    aiModel: 'gpt-3.5-turbo',
    apiKey: '',
  }
  ElMessage.info('设置已重置为默认值')
}

function goBack() {
  router.back()
}
</script>

<template>
  <div class="settings-page min-h-screen bg-gray-50 dark:bg-gray-900">
    <div class="container mx-auto px-4 py-8">
      <ElPageHeader @back="goBack" class="mb-6">
        <template #content>
          <h1 class="text-2xl font-bold">设置</h1>
        </template>
      </ElPageHeader>

      <div class="max-w-3xl mx-auto space-y-6">
        <!-- 外观设置 -->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">外观</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="主题">
              <ElSelect v-model="settings.theme" class="w-full">
                <ElOption
                  v-for="theme in themes"
                  :key="theme.value"
                  :label="theme.label"
                  :value="theme.value"
                />
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="语言">
              <ElSelect v-model="settings.language" class="w-full">
                <ElOption
                  v-for="lang in languages"
                  :key="lang.value"
                  :label="lang.label"
                  :value="lang.value"
                />
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="字体大小">
              <ElInput v-model.number="settings.fontSize" type="number" :min="10" :max="24">
                <template #append>px</template>
              </ElInput>
            </ElFormItem>
          </ElForm>
        </ElCard>

        <!-- 编辑器设置 -->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">编辑器</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="自动保存">
              <ElSwitch v-model="settings.autoSave" />
            </ElFormItem>
          </ElForm>
        </ElCard>

        <!-- AI 设置 -->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">AI 配置</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="AI 模型">
              <ElSelect v-model="settings.aiModel" class="w-full">
                <ElOption
                  v-for="model in aiModels"
                  :key="model.value"
                  :label="model.label"
                  :value="model.value"
                />
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="API Key">
              <ElInput
                v-model="settings.apiKey"
                type="password"
                show-password
                placeholder="输入您的 API Key"
              />
            </ElFormItem>
          </ElForm>
        </ElCard>

        <!-- 操作按钮 -->
        <div class="flex justify-end gap-4">
          <ElButton @click="handleReset">重置为默认</ElButton>
          <ElButton type="primary" @click="handleSave">保存设置</ElButton>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  min-height: 100vh;
}
</style>
