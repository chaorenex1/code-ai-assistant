<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import {
  Tools,
  Key,
  DataAnalysis,
  Monitor
} from '@element-plus/icons-vue';

const route = useRoute();
const router = useRouter();

interface MenuItem {
  path: string;
  name: string;
  icon: any;
}

const menuItems = ref<MenuItem[]>([
  { path: '/settings/cli-paths', name: 'CLI 工具路径', icon: Tools },
  { path: '/settings/environment', name: '环境变量', icon: Key },
  { path: '/settings/ai-models', name: 'AI 模型', icon: DataAnalysis },
  { path: '/settings/code-cli', name: 'Code CLI', icon: Monitor },
]);

const activeMenu = computed(() => route.path);

function navigateTo(path: string) {
  router.push(path);
}
</script>

<template>
  <div class="settings-page">
    <div class="settings-sidebar">
      <div class="sidebar-header">
        <h2 class="text-xl font-bold">设置</h2>
      </div>
      <div class="sidebar-menu">
        <div
          v-for="item in menuItems"
          :key="item.path"
          :class="['menu-item', { active: activeMenu === item.path }]"
          @click="navigateTo(item.path)"
        >
          <component :is="item.icon" class="menu-icon" />
          <span>{{ item.name }}</span>
        </div>
      </div>
    </div>
    <div class="settings-content">
      <router-view />
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  display: flex;
  width: 100%;
  height: 100%;
  background-color: var(--color-background);
}

.settings-sidebar {
  width: 240px;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 24px 20px;
  border-bottom: 1px solid var(--color-border);
}

.sidebar-menu {
  flex: 1;
  padding: 12px 8px;
  overflow-y: auto;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  margin-bottom: 4px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--color-text-secondary);
}

.menu-item:hover {
  background-color: rgba(0, 0, 0, 0.04);
  color: var(--color-text);
}

.menu-item.active {
  background-color: var(--color-primary);
  color: white;
}

.menu-icon {
  width: 18px;
  height: 18px;
  margin-right: 12px;
}

.settings-content {
  flex: 1;
  overflow-y: auto;
  background-color: var(--color-background);
}

/* 自定义滚动条 */
.settings-content::-webkit-scrollbar {
  width: 8px;
}

.settings-content::-webkit-scrollbar-track {
  background: transparent;
}

.settings-content::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 4px;
}

.settings-content::-webkit-scrollbar-thumb:hover {
  background-color: rgba(156, 163, 175, 0.8);
}

.sidebar-menu::-webkit-scrollbar {
  width: 6px;
}

.sidebar-menu::-webkit-scrollbar-track {
  background: transparent;
}

.sidebar-menu::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.3);
  border-radius: 3px;
}
</style>
