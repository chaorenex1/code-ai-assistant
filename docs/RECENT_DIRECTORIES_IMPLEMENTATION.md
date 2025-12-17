# 最近打开目录功能实现文档

## 功能概述

实现了完整的"最近打开目录"功能，包括：
1. 点击"开始编码"打开系统目录选择对话框
2. 将选中的路径发送到后端保存
3. 获取目录下的文件和目录信息并展示在主界面
4. 后端数据库记录最近打开的目录（最多5条）
5. 在首页展示最近打开的目录列表

## 实现细节

### 1. 后端实现

#### 数据库存储库
创建了 `recent_directories_repository.rs`，实现以下功能：
- `add()` - 添加最近打开的目录，自动去重并限制最多5条
- `get_recent()` - 获取最近打开的目录列表（按时间倒序）
- `clear()` - 清除所有最近打开的目录记录

存储在 `settings` 表中，使用 `category = 'recent_directory'` 标识。

#### Tauri 命令
在 `commands.rs` 中添加了3个命令：
- `add_recent_directory(path: String)` - 保存目录到数据库
- `get_recent_directories()` - 获取最近目录列表
- `clear_recent_directories()` - 清除记录

返回数据格式：
```json
[
  {
    "path": "/path/to/directory",
    "openedAt": "2025-12-18T10:30:00Z"
  }
]
```

### 2. 前端实现

#### commands.ts
添加了类型定义和命令封装：
```typescript
export interface RecentDirectory {
  path: string;
  openedAt: string;
}

export async function addRecentDirectory(path: string): Promise<void>
export async function getRecentDirectories(): Promise<RecentDirectory[]>
export async function clearRecentDirectories(): Promise<void>
```

#### Home/index.vue
更新了主页功能：

1. **选择目录流程**：
   - 点击"开始编码"按钮
   - 打开系统目录选择对话框（使用 Tauri 的 `openFileDialog`）
   - 选择后调用 `fileStore.loadDirectory()` 加载目录
   - 调用 `addRecentDirectory()` 保存到后端数据库
   - 自动跳转到 dashboard 页面

2. **最近目录展示**：
   - 页面加载时调用 `getRecentDirectories()` 获取列表
   - 展示目录路径和最后打开时间
   - 点击目录项可直接打开该目录
   - 使用卡片式UI展示，支持响应式布局

3. **代码结构**：
```typescript
// 从后端加载最近目录
async function loadRecentDirectories() {
  const directories = await getRecentDirectories();
  recentDirectories.value = directories;
}

// 保存最近打开的目录
async function saveRecentDirectory(path: string) {
  await addRecentDirectory(path);
  await loadRecentDirectories();
}

// 打开最近的目录
async function openRecentDirectory(dir: RecentDirectory) {
  await fileStore.loadDirectory(dir.path);
  await addRecentDirectory(dir.path); // 更新最近打开时间
  router.push('/dashboard');
}
```

#### filesStore.ts
优化了 `loadDirectory()` 方法：
- 当加载新的根目录时，自动清空之前打开的文件
- 重置活动文件索引
- 确保目录切换时的状态一致性

### 3. UI展示

#### 首页最近目录卡片
```vue
<ElCard>
  <template #header>
    <h2 class="text-2xl font-bold text-center">
      最近打开的目录
    </h2>
  </template>
  <div class="space-y-2">
    <div
      v-for="(dir, index) in recentDirectories"
      :key="index"
      class="flex items-center justify-between p-3 hover:bg-gray-50 dark:hover:bg-gray-800 rounded cursor-pointer"
      @click="openRecentDirectory(dir)"
    >
      <div class="flex items-center flex-1">
        <el-icon class="mr-3 text-primary-500">
          <Folder />
        </el-icon>
        <div class="flex-1 min-w-0">
          <div class="font-medium truncate">
            {{ dir.path }}
          </div>
          <div class="text-sm text-gray-500">
            {{ new Date(dir.openedAt).toLocaleString('zh-CN') }}
          </div>
        </div>
      </div>
      <el-icon class="text-gray-400 ml-2">
        <Promotion />
      </el-icon>
    </div>
  </div>
</ElCard>
```

特点：
- 响应式悬停效果
- 文本截断防止溢出
- 显示中文格式化的时间
- 图标指示器
- 点击整个区域都可以触发

#### Dashboard 主界面
文件将在 MainLayout 中的 FileExplorer 组件展示：
- 使用树形结构展示目录和文件
- 支持展开/折叠目录
- 点击文件在编辑器中打开
- 支持创建、删除、重命名操作

## 数据流程

```
用户点击"开始编码"
    ↓
打开系统目录选择对话框
    ↓
用户选择目录
    ↓
前端: fileStore.loadDirectory(path)
    ↓
后端: list_files(path) → 返回文件列表
    ↓
前端: 显示文件列表
    ↓
前端: addRecentDirectory(path)
    ↓
后端: 保存到数据库（去重、限制5条）
    ↓
前端: 跳转到 dashboard
```

## 错误处理

所有异步操作都包含错误处理：
- 使用 try-catch 捕获异常
- 使用 ElMessage 显示用户友好的错误消息
- 记录详细错误到控制台以便调试

示例：
```typescript
try {
  await fileStore.loadDirectory(dir.path);
  await addRecentDirectory(dir.path);
  router.push('/dashboard');
} catch (error) {
  ElMessage.error('打开目录失败: ' + (error as Error).message);
  console.error('打开目录失败', error);
}
```

## 性能优化

1. **数据库层面**：
   - 使用索引加速查询（category 字段）
   - 自动限制记录数量（最多5条）
   - 使用 updated_at 字段排序

2. **前端层面**：
   - 组件挂载时才加载数据（onMounted）
   - 使用响应式数据自动更新UI
   - 防抖处理（如有频繁操作）

3. **UI层面**：
   - 使用 v-if 条件渲染（无数据时不显示卡片）
   - 文本截断防止长路径影响布局
   - 合理的加载状态提示

## 使用场景

1. **首次使用**：
   - 用户点击"开始编码"
   - 选择项目目录
   - 开始工作

2. **重复访问**：
   - 用户打开应用
   - 从最近目录列表直接选择
   - 快速继续工作

3. **切换项目**：
   - 在不同项目间快速切换
   - 无需每次都浏览文件系统
   - 提高工作效率

## 测试要点

1. **功能测试**：
   - [ ] 选择目录后能正确保存
   - [ ] 最近目录列表正确显示
   - [ ] 点击最近目录能正确打开
   - [ ] 限制最多5条记录
   - [ ] 重复打开同一目录会更新时间而不是重复记录

2. **边界情况**：
   - [ ] 目录不存在时的处理
   - [ ] 没有权限访问目录时的处理
   - [ ] 目录路径特殊字符的处理
   - [ ] 数据库连接失败的处理

3. **UI测试**：
   - [ ] 长路径正确截断显示
   - [ ] 悬停效果正常
   - [ ] 响应式布局在不同屏幕尺寸下正常
   - [ ] 加载状态正确显示

## 未来改进

1. **功能增强**：
   - 添加收藏目录功能
   - 支持目录标签/分类
   - 添加目录搜索功能
   - 支持工作区概念（一个工作区包含多个目录）

2. **UI优化**：
   - 添加目录图标预览
   - 显示目录文件统计信息
   - 添加快捷操作按钮（从列表中移除等）
   - 支持拖拽排序

3. **性能优化**：
   - 实现虚拟滚动（如果目录很多）
   - 添加缓存机制
   - 优化大目录加载速度

4. **同步功能**：
   - 云端同步最近目录
   - 多设备数据共享
