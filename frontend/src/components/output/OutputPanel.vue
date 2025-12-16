<script setup lang="ts">
import { ref, computed } from 'vue'
import { Search, Delete, VideoPause, VideoPlay } from '@element-plus/icons-vue'
import { ElInput, ElButton, ElSelect, ElOption, ElTooltip } from 'element-plus'

const logs = ref<string[]>([
  '2024-01-15 10:30:25 INFO - 应用启动成功',
  '2024-01-15 10:30:26 DEBUG - 加载配置文件: config.toml',
  '2024-01-15 10:30:27 INFO - 数据库连接成功',
  '2024-01-15 10:30:28 ERROR - 文件读取失败: /path/to/file',
  '2024-01-15 10:30:29 INFO - AI模型加载完成: claude-3-5-sonnet',
  '2024-01-15 10:30:30 INFO - 终端初始化完成',
  '2024-01-15 10:30:31 DEBUG - 监听文件变化',
  '2024-01-15 10:30:32 INFO - 工作区加载完成: default',
])

const searchQuery = ref('')
const logLevel = ref('all')
const isPaused = ref(false)

// Filter logs based on search and level
const filteredLogs = computed(() => {
  return logs.value.filter(log => {
    // Filter by level
    if (logLevel.value !== 'all') {
      const level = log.split(' ')[2]?.toLowerCase()
      if (level !== logLevel.value.toLowerCase()) {
        return false
      }
    }

    // Filter by search query
    if (searchQuery.value) {
      return log.toLowerCase().includes(searchQuery.value.toLowerCase())
    }

    return true
  })
})

// Clear logs
function clearLogs() {
  if (confirm('确定要清空所有日志吗？')) {
    logs.value = []
  }
}

// Toggle pause
function togglePause() {
  isPaused.value = !isPaused.value
}
</script>

<template>
  <div class="h-full flex flex-col">
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

        <ElSelect v-model="logLevel" size="small" style="width: 100px">
          <ElOption label="全部" value="all" />
          <ElOption label="INFO" value="info" />
          <ElOption label="DEBUG" value="debug" />
          <ElOption label="WARN" value="warn" />
          <ElOption label="ERROR" value="error" />
        </ElSelect>
      </div>

      <div class="flex items-center justify-between">
        <div class="text-sm text-text-secondary">
          共 {{ filteredLogs.length }} 条日志
        </div>

        <div class="flex items-center space-x-2">
          <ElTooltip :content="isPaused ? '继续输出' : '暂停输出'" placement="bottom">
            <ElButton
              :icon="isPaused ? Play : Pause"
              size="small"
              text
              @click="togglePause"
            />
          </ElTooltip>

          <ElTooltip content="清空日志" placement="bottom">
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
    <div class="flex-1 overflow-auto p-2 font-mono text-sm">
      <div v-if="filteredLogs.length === 0" class="text-center p-4 text-text-secondary">
        {{ searchQuery || logLevel !== 'all' ? '没有匹配的日志' : '暂无日志' }}
      </div>

      <div v-else class="space-y-1">
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
</style>