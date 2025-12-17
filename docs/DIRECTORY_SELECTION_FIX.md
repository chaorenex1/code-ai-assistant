# 目录选择功能问题修复

## 问题描述
选择目录失败，无法打开系统目录选择对话框。

## 根本原因
原代码使用了自定义的 `openFileDialog` 命令，但该命令没有在后端实现。应该使用 Tauri 官方的 `@tauri-apps/plugin-dialog` 插件。

## 解决方案

### 1. 已安装的依赖
✅ 前端：`@tauri-apps/plugin-dialog` (v2.4.2) - 已安装
✅ 后端：`tauri-plugin-dialog` (v2.0.0) - 已配置
✅ 后端：插件已在 `main.rs` 中初始化

### 2. 代码修改

#### 前端 (Home/index.vue)
```typescript
// 导入正确的 API
import { open as openDialog } from '@tauri-apps/plugin-dialog';

// 使用 Tauri dialog 插件
async function selectDirectory() {
  try {
    isLoading.value = true;
    
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: '选择工作目录',
    });

    if (result && typeof result === 'string') {
      await fileStore.loadDirectory(result);
      await saveRecentDirectory(result);
      showDirectoryDialog.value = false;
      router.push('/dashboard');
    } else {
      showDirectoryDialog.value = false;
    }
  } catch (error) {
    ElMessage.error('选择目录失败: ' + (error as Error).message);
    console.error('选择目录失败', error);
    showDirectoryDialog.value = false;
  } finally {
    isLoading.value = false;
  }
}
```

### 3. 重启应用
需要重新启动开发服务器以使新安装的包生效：

```bash
# 停止当前服务器（Ctrl+C）
# 然后重新运行
pnpm run tauri:dev
```

### 4. 使用流程

1. 用户点击"开始编码"按钮
2. 显示提示对话框（可选）
3. 点击"选择目录"按钮
4. 系统文件选择对话框打开
5. 用户选择目录
6. 目录路径保存到数据库
7. 加载目录文件列表
8. 自动跳转到 Dashboard

### 5. API 说明

#### openDialog 参数
```typescript
{
  directory: true,     // 选择目录而不是文件
  multiple: false,     // 不允许多选
  title: string,       // 对话框标题
  defaultPath?: string // 默认路径（可选）
}
```

#### 返回值
- `string` - 选中的目录路径
- `null` - 用户取消选择

### 6. 错误处理

所有可能的错误都已处理：
- 用户取消选择 → 关闭对话框
- 权限错误 → 显示错误消息
- 网络错误 → 显示错误消息
- 数据库错误 → 记录到控制台（不影响主流程）

### 7. 测试检查清单

- [ ] 点击"开始编码"能打开对话框
- [ ] 系统文件选择对话框正确显示
- [ ] 选择目录后能加载文件
- [ ] 目录保存到最近列表
- [ ] 取消选择时正确关闭
- [ ] 错误消息正确显示

## 注意事项

1. **TypeScript 编译错误**：如果看到 "找不到模块" 错误，需要：
   - 停止开发服务器
   - 删除 `node_modules/.vite` 缓存
   - 重新运行 `pnpm run tauri:dev`

2. **权限问题**：在某些操作系统上，可能需要授予应用文件系统访问权限。

3. **路径格式**：Tauri 会自动处理不同操作系统的路径格式（Windows/Unix）。

## 相关文件

- `frontend/src/pages/Home/index.vue` - 主页组件
- `frontend/src/services/tauri/commands.ts` - Tauri 命令封装
- `src-tauri/src/main.rs` - 后端插件配置
- `src-tauri/Cargo.toml` - 后端依赖配置
