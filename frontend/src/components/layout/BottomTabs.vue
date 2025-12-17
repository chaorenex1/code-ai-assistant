<script setup lang="ts">
import { ElButton, ElButtonGroup } from 'element-plus';
import { computed } from 'vue';

import ChatPanel from '@/components/chat/ChatPanel.vue';
import OutputPanel from '@/components/output/OutputPanel.vue';
import TerminalPanel from '@/components/terminal/TerminalPanel.vue';
import { useAppStore } from '@/stores/workspaceStore';

interface BottomTab {
  key: string;
  label: string;
  icon: any;
}

const props = defineProps<{
  tabs: BottomTab[];
  activeTab: string;
  visible: boolean;
  height: number;
  showFileExplorer: boolean;
  sidebarWidth: number;
}>();

const emit = defineEmits<{
  (e: 'update:activeTab', value: string): void;
  (e: 'update:visible', value: boolean): void;
}>();

const appStore = useAppStore();

const FOOTER_HEIGHT = 32;

const bottomPanelStyle = computed(() => ({
  height: props.height + 'px',
  left: props.showFileExplorer ? props.sidebarWidth + 4 + 'px' : '0',
  bottom: FOOTER_HEIGHT + 'px',
}));

const toggleBarStyle = computed(() => ({
  left: props.showFileExplorer ? props.sidebarWidth + 4 + 'px' : '0',
  bottom: props.height + FOOTER_HEIGHT + 'px',
}));

function onToggleVisible() {
  emit('update:visible', !props.visible);
}

function onSelectTab(key: string) {
  emit('update:activeTab', key);
}
</script>

<template>
  <div>
    <!-- Bottom Panel Toggle -->
    <div
      class="border-t border-border bg-surface px-4 py-1 toggle-bar"
      :style="toggleBarStyle"
    >
      <div class="flex items-center justify-between">
        <ElButtonGroup>
          <ElButton
            v-for="tab in props.tabs"
            :key="tab.key"
            :type="props.activeTab === tab.key ? 'primary' : 'default'"
            :icon="tab.icon"
            @click="onSelectTab(tab.key)"
          >
            {{ tab.label }}
          </ElButton>
        </ElButtonGroup>

        <ElButton
          :icon="props.visible ? 'ArrowDown' : 'ArrowUp'"
          text
          @click="onToggleVisible"
        >
          {{ props.visible ? '隐藏面板' : '显示面板' }}
        </ElButton>
      </div>
    </div>

    <!-- Bottom Panel -->
    <div
      v-if="props.visible"
      class="border-t border-border overflow-hidden bottom-panel"
      :style="bottomPanelStyle"
    >
      <ChatPanel v-if="props.activeTab === 'chat'" />
      <OutputPanel v-else-if="props.activeTab === 'output'" />
      <TerminalPanel v-else-if="props.activeTab === 'terminal'" />
    </div>

    <!-- Footer 状态信息-->
    <div class="border-t border-border bg-surface px-4 py-2 text-sm text-text-secondary footer-bar">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <span>工作区: {{ appStore.currentWorkspace }}</span>
          <span>|</span>
          <span>文件: {{ appStore.currentFile || '未选择文件' }}</span>
        </div>

        <div class="flex items-center space-x-4">
          <span>AI模型: {{ appStore.currentAiModel }}</span>
          <span>|</span>
          <span>状态: {{ appStore.isConnected ? '已连接' : '未连接' }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.bottom-panel {
  position: fixed;
  right: 0;
}

.toggle-bar {
  position: fixed;
  right: 0;
}

.footer-bar {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
}
</style>
