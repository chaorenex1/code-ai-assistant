<script setup lang="ts">
import { useRouter } from 'vue-router';
import AiModelsManagement from '../../components/settings/AiModelsManagement.vue';

const router = useRouter();

function goBack() {
  router.push('/dashboard');
}
</script>

<template>
  <div class="settings-page-wrapper">
    <div class="settings-page-header">
      <div class="settings-page-header-main">
        <h2 class="text-2xl font-bold">AI 模型管理</h2>
        <p class="text-sm text-gray-500 mt-2">配置和管理 AI 模型设置</p>
      </div>
      <button
        type="button"
        class="back-button"
        @click="goBack"
      >
        返回
      </button>
    </div>
    <div class="settings-page-content">
      <AiModelsManagement />
    </div>
  </div>
</template>

<style scoped>
.settings-page-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 24px 32px;
}

.settings-page-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.settings-page-header-main {
  flex: 1;
}

.settings-page-content {
  flex: 1;
  overflow-y: auto;
}

.back-button {
  padding: 6px 14px;
  font-size: 14px;
  border-radius: 999px;
  border: 1px solid var(--color-border);
  background-color: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.15s ease-in-out;
}

.back-button:hover {
  background-color: rgba(0, 0, 0, 0.04);
  color: var(--color-text);
}
</style>
