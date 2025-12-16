<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Terminal } from '@xterm/xterm'
import { FitAddon } from '@xterm/addon-fit'
import { WebLinksAddon } from '@xterm/addon-web-links'
import '@xterm/xterm/css/xterm.css'
import { Plus, Close, Refresh } from '@element-plus/icons-vue'
import { ElTabs, ElTabPane, ElButton, ElSelect, ElOption } from 'element-plus'
import { useAppStore } from '../../stores/app'

const appStore = useAppStore()
const terminalContainer = ref<HTMLElement>()
const terminals = ref<{ id: string; name: string; terminal: Terminal }[]>([])
const activeTerminalIndex = ref(0)
const fitAddon = ref<FitAddon>()

// Initialize terminal
onMounted(() => {
  createNewTerminal()
})

// Cleanup on unmount
onUnmounted(() => {
  terminals.value.forEach(term => {
    term.terminal.dispose()
  })
})

// Create new terminal
function createNewTerminal() {
  const id = `terminal-${Date.now()}`
  const name = `终端 ${terminals.value.length + 1}`

  // Create terminal instance
  const terminal = new Terminal({
    fontSize: 14,
    fontFamily: 'Consolas, "Courier New", monospace',
    theme: {
      background: getComputedStyle(document.documentElement)
        .getPropertyValue('--color-background')
        .trim(),
      foreground: getComputedStyle(document.documentElement)
        .getPropertyValue('--color-text')
        .trim(),
      cursor: getComputedStyle(document.documentElement)
        .getPropertyValue('--color-primary')
        .trim(),
    },
    cursorBlink: true,
    scrollback: 1000,
  })

  // Add addons
  const fitAddonInstance = new FitAddon()
  const webLinksAddon = new WebLinksAddon()

  terminal.loadAddon(fitAddonInstance)
  terminal.loadAddon(webLinksAddon)

  // Store terminal
  terminals.value.push({ id, name, terminal })
  activeTerminalIndex.value = terminals.value.length - 1

  // Initialize terminal after DOM update
  nextTick(() => {
    const container = terminalContainer.value
    if (container) {
      terminal.open(container)
      fitAddonInstance.fit()

      // Write welcome message
      terminal.writeln('\x1b[1;32m欢迎使用 Code AI Assistant 终端\x1b[0m')
      terminal.writeln('输入 "help" 查看可用命令')
      terminal.writeln('')

      // Set up command handling
      setupCommandHandling(terminal)
    }
  })
}

// Set up command handling
function setupCommandHandling(terminal: Terminal) {
  let command = ''

  terminal.onData(data => {
    if (data === '\r') { // Enter key
      terminal.writeln('')
      handleCommand(command, terminal)
      command = ''
      terminal.write('$ ')
    } else if (data === '\u007F') { // Backspace
      if (command.length > 0) {
        command = command.slice(0, -1)
        terminal.write('\b \b')
      }
    } else if (data === '\u0003') { // Ctrl+C
      terminal.writeln('^C')
      command = ''
      terminal.write('$ ')
    } else if (data >= ' ' && data <= '~') { // Printable characters
      command += data
      terminal.write(data)
    }
  })

  // Initial prompt
  terminal.write('$ ')
}

// Handle command execution
async function handleCommand(command: string, terminal: Terminal) {
  const trimmedCommand = command.trim()

  if (!trimmedCommand) {
    return
  }

  // Built-in commands
  if (trimmedCommand === 'help') {
    terminal.writeln('可用命令:')
    terminal.writeln('  help     - 显示此帮助信息')
    terminal.writeln('  clear    - 清空终端')
    terminal.writeln('  ls       - 列出文件')
    terminal.writeln('  pwd      - 显示当前目录')
    terminal.writeln('  echo     - 显示文本')
    terminal.writeln('  exit     - 退出终端')
    return
  }

  if (trimmedCommand === 'clear') {
    terminal.clear()
    terminal.write('$ ')
    return
  }

  if (trimmedCommand === 'exit') {
    const index = terminals.value.findIndex(t => t.terminal === terminal)
    if (index >= 0) {
      closeTerminal(index)
    }
    return
  }

  // Execute command via Tauri
  try {
    const [cmd, ...args] = trimmedCommand.split(' ')
    const result = await invoke('execute_command', {
      command: cmd,
      args,
      cwd: '.',
    })

    if (result) {
      terminal.writeln(result)
    }
  } catch (error) {
    terminal.writeln(`\x1b[1;31m错误: ${error}\x1b[0m`)
  }

  terminal.write('$ ')
}

// Close terminal
function closeTerminal(index: number) {
  if (terminals.value[index]) {
    terminals.value[index].terminal.dispose()
    terminals.value.splice(index, 1)

    if (terminals.value.length === 0) {
      createNewTerminal()
    } else if (activeTerminalIndex.value >= terminals.value.length) {
      activeTerminalIndex.value = terminals.value.length - 1
    }
  }
}

// Switch terminal
function switchTerminal(index: number) {
  activeTerminalIndex.value = index
}

// Refresh terminal
function refreshTerminal() {
  const terminal = terminals.value[activeTerminalIndex.value]?.terminal
  if (terminal) {
    terminal.clear()
    terminal.writeln('\x1b[1;32m终端已刷新\x1b[0m')
    terminal.writeln('')
    terminal.write('$ ')
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Terminal Tabs -->
    <div class="border-b border-border bg-surface">
      <div class="flex items-center justify-between px-4 py-2">
        <ElTabs
          v-model="activeTerminalIndex"
          type="card"
          closable
          @tab-click="switchTerminal"
          @tab-remove="closeTerminal"
        >
          <ElTabPane
            v-for="(term, index) in terminals"
            :key="term.id"
            :label="term.name"
            :name="index"
          />
        </ElTabs>

        <div class="flex items-center space-x-2">
          <ElButton
            :icon="Plus"
            size="small"
            text
            @click="createNewTerminal"
          >
            新建
          </ElButton>

          <ElButton
            :icon="Refresh"
            size="small"
            text
            @click="refreshTerminal"
          >
            刷新
          </ElButton>

          <ElSelect
            v-model="appStore.settings.terminalShell"
            size="small"
            style="width: 100px"
            @change="appStore.setTerminalShell"
          >
            <ElOption label="bash" value="bash" />
            <ElOption label="zsh" value="zsh" />
            <ElOption label="powershell" value="powershell" />
            <ElOption label="cmd" value="cmd" />
          </ElSelect>
        </div>
      </div>
    </div>

    <!-- Terminal Area -->
    <div ref="terminalContainer" class="flex-1 overflow-hidden"></div>

    <!-- Status Bar -->
    <div class="border-t border-border bg-surface px-4 py-1 text-xs text-text-secondary">
      <div class="flex items-center justify-between">
        <div>
          <span>终端: {{ terminals[activeTerminalIndex]?.name || '' }}</span>
          <span class="mx-2">|</span>
          <span>Shell: {{ appStore.settings.terminalShell }}</span>
        </div>

        <div>
          <span>就绪</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.xterm) {
  padding: 8px;
  height: 100%;
}

:deep(.xterm-screen) {
  height: 100%;
}

:deep(.el-tabs__header) {
  margin: 0;
}

:deep(.el-tabs__nav-wrap) {
  padding: 0;
}
</style>