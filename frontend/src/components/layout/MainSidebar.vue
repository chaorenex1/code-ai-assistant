<script setup lang="ts">
import { ElAside } from 'element-plus';
import FileExplorer from '@/components/file-explorer/FileExplorer.vue';

const props = defineProps<{
  visible: boolean;
  width: number;
}>();
</script>

<template>
  <template v-if="props.visible">
    <ElAside
      class="border-r border-border bg-surface overflow-auto"
      :style="{ width: props.width + 'px' }"
    >
      <FileExplorer />
    </ElAside>

    <div class="sidebar-resizer" />
  </template>
</template>

<style scoped>
.sidebar-resizer {
  width: 4px;
  cursor: col-resize;
  background-color: transparent;
}

.sidebar-resizer:hover {
  background-color: rgba(0, 0, 0, 0.06);
}
</style>
