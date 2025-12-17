<script setup lang="ts">
import { Search, Delete, VideoPause, VideoPlay } from '@element-plus/icons-vue';
import { ElInput, ElButton, ElSelect, ElOption, ElTooltip } from 'element-plus';
import { ref, computed, onMounted, onUnmounted } from 'vue';

import { getLogs, clearLogs as clearLogsCommand } from '@/services/tauri/commands';

const logs = ref<string[]>([]);

const searchQuery = ref('');
const logLevel = ref('all');
const isPaused = ref(false);

let refreshTimer: number | null = null;

// 从后端加载日志
async function loadLogs() {
  try {
    const data = await getLogs();
    logs.value = data;
  } catch (error) {
    console.error('加载日志失败:', error);
  }
}

function startAutoRefresh() {
  if (refreshTimer !== null) return;
  refreshTimer = window.setInterval(() => {
    if (!isPaused.value) {
      loadLogs();
    }
  }, 3000);
}

function stopAutoRefresh() {
  if (refreshTimer !== null) {
    clearInterval(refreshTimer);
    refreshTimer = null;
  }
}

onMounted(() => {
  loadLogs();
  startAutoRefresh();
});

onUnmounted(() => {
  stopAutoRefresh();
});

// Filter logs based on search and level
const filteredLogs = computed(() => {
  return logs.value.filter((log) => {
    // Filter by level
    if (logLevel.value !== 'all') {
      const level = log.split(' ')[2]?.toLowerCase();
      if (level !== logLevel.value.toLowerCase()) {
        return false;
      }
    }

    // Filter by search query
    if (searchQuery.value) {
      return log.toLowerCase().includes(searchQuery.value.toLowerCase());
    }

    return true;
  });
});

// Clear logs
async function clearLogs() {
  if (confirm('确定要清空所有日志吗？')) {
    try {
      await clearLogsCommand();
      logs.value = [];
    } catch (error) {
      console.error('清空日志失败:', error);
    }
  }
}

// Toggle pause
function togglePause() {
  isPaused.value = !isPaused.value;
}
</script>

<template>
  <div class="h-full flex flex-col output-panel">
    <!-- Toolbar -->
    <div class="border-b border-border bg-surface p-2">
      <div class="flex items-center space-x-2 mb-2">
        <ElInput
          v-model="searchQuery"
          :prefix-icon="Search"
          placeholder="搜索日志..."
          size="small"
          clearable
          class="flex-1"
        />

        <ElSelect
          v-model="logLevel"
          size="small"
          style="width: 100px"
        >
          <ElOption
            label="全部"
            value="all"
          />
          <ElOption
            label="INFO"
            value="info"
          />
          <ElOption
            label="DEBUG"
            value="debug"
          />
          <ElOption
            label="WARN"
            value="warn"
          />
          <ElOption
            label="ERROR"
            value="error"
          />
        </ElSelect>
      </div>

      <div class="flex items-center justify-between">
        <div class="text-sm text-text-secondary">
          共 {{ filteredLogs.length }} 条日志
        </div>

        <div class="flex items-center space-x-2">
          <ElTooltip
            :content="isPaused ? '继续输出' : '暂停输出'"
            placement="bottom"
          >
            <ElButton
		      :icon="isPaused ? VideoPlay : VideoPause"
              size="small"
              text
              @click="togglePause"
            />
          </ElTooltip>

          <ElTooltip
            content="清空日志"
            placement="bottom"
          >
            <ElButton
              :icon="Delete"
              size="small"
              text
              @click="clearLogs"
            />
          </ElTooltip>
        </div>
      </div>
    </div>

    <!-- Logs Area -->
    <div class="flex-1 overflow-auto p-2 font-mono text-sm logs-container">
      <div
        v-if="filteredLogs.length === 0"
        class="text-center p-4 text-text-secondary"
      >
        {{ searchQuery || logLevel !== 'all' ? '没有匹配的日志' : '暂无日志' }}
      </div>

      <div
        v-else
        class="space-y-1"
      >
        <div
          v-for="(log, index) in filteredLogs"
          :key="index"
          class="p-2 rounded hover:bg-surface"
          :class="{
            'text-blue-500': log.includes('INFO'),
            'text-green-500': log.includes('DEBUG'),
            'text-yellow-500': log.includes('WARN'),
            'text-red-500': log.includes('ERROR'),
          }"
        >
          {{ log }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.el-select) {
  width: 100px;
}

.output-panel {
  min-width: 0;
}

.logs-container {
  white-space: pre-wrap;
  word-break: break-all;
}
</style>
