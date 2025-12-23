# read_file 故障排查指南

## 问题：前端无法读取文件

### ✅ 已验证正常的部分
1. **后端测试通过** - `read_file` 函数本身工作正常
2. **命令已注册** - 在 `main.rs` 中正确注册
3. **权限配置** - capabilities/desktop.json 已更新

### 🔍 排查步骤

#### 1. 检查浏览器控制台错误
打开开发者工具（F12），查看是否有以下错误：
- `invoke error: ...`
- 权限被拒绝
- 路径不存在

#### 2. 验证前端调用方式

**正确的调用方式：**
```typescript
import { invoke } from '@tauri-apps/api/core';

// 使用绝对路径
const content = await invoke<string>('read_file', { 
  path: 'C:\\Users\\YourName\\test.txt' 
});
```

**常见错误：**
❌ 使用相对路径：`./test.txt`
❌ 路径格式错误：`C:\Users\...` （需要转义或使用 `/`）
❌ 文件不存在或无权限访问

#### 3. 测试路径格式

**Windows 路径格式：**
```typescript
// 方式 1：双反斜杠
const path1 = 'C:\\Users\\username\\Documents\\test.txt';

// 方式 2：正斜杠
const path2 = 'C:/Users/username/Documents/test.txt';

// 方式 3：原始字符串（推荐）
const path3 = String.raw`C:\Users\username\Documents\test.txt`;
```

#### 4. 创建测试用例

在前端创建一个测试按钮：

```vue
<template>
  <button @click="testReadFile">测试读取文件</button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

async function testReadFile() {
  try {
    // 创建一个测试文件
    const testPath = 'C:/Users/zarag/test.txt';
    
    // 写入测试内容
    await invoke('write_file', { 
      path: testPath, 
      content: 'Hello from Tauri!' 
    });
    
    // 读取测试文件
    const content = await invoke<string>('read_file', { 
      path: testPath 
    });
    
    console.log('✅ 读取成功:', content);
    alert(`成功读取: ${content}`);
  } catch (error) {
    console.error('❌ 读取失败:', error);
    alert(`失败: ${error}`);
  }
}
</script>
```

#### 5. 检查文件大小限制

当前限制：**8MB**

如果文件超过此大小，会返回错误：
```
文件过大（X bytes），最大支持 8388608 bytes
```

#### 6. 使用文件选择器

推荐使用 Tauri 的文件选择器确保路径正确：

```typescript
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

async function openAndReadFile() {
  // 打开文件选择器
  const filePath = await open({
    multiple: false,
    directory: false,
    filters: [{
      name: 'Text',
      extensions: ['txt', 'md', 'json']
    }]
  });
  
  if (filePath) {
    try {
      const content = await invoke<string>('read_file', { 
        path: filePath 
      });
      console.log('文件内容:', content);
    } catch (error) {
      console.error('读取失败:', error);
    }
  }
}
```

### 🐛 常见问题

#### 问题1：路径包含中文或特殊字符
**解决：** 确保路径正确编码，Tauri 支持 UTF-8

#### 问题2：跨平台路径问题
**解决：** 使用 `@tauri-apps/api/path` 模块
```typescript
import { join, appDataDir } from '@tauri-apps/api/path';

const dataDir = await appDataDir();
const filePath = await join(dataDir, 'myfile.txt');
```

#### 问题3：开发环境能用，生产环境不能用
**检查：** 
- capabilities/desktop.json 是否包含在构建中
- 生产环境的文件路径是否正确

### 📝 调试命令

```bash
# 运行 Tauri 开发服务器并查看后端日志
cd src-tauri
cargo tauri dev

# 运行测试
cargo test read_file

# 检查编译错误
cargo check
```

### 🔧 重新构建

如果修改了 capabilities 配置，需要重新构建：

```bash
# 清理构建缓存
cd src-tauri
cargo clean

# 重新构建
cd ..
pnpm run tauri:dev
```

### ✨ 验证修复

1. 打开应用
2. 按 F12 打开控制台
3. 尝试读取一个已知存在的文件
4. 检查控制台输出

如果看到文件内容，说明问题已解决！🎉
