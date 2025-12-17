# 设置集成文档

## 概述

已实现从后端读取配置和保存配置到后端的功能。

## 功能实现

### 后端 (Rust/Tauri)

在 `src-tauri/src/tauri/commands.rs` 中，已实现以下命令：

1. **get_settings** - 从数据库获取所有设置
   ```rust
   pub async fn get_settings(app: tauri::AppHandle) -> Result<serde_json::Value, String>
   ```

2. **save_settings** - 保存设置到数据库
   ```rust
   pub async fn save_settings(
       app: tauri::AppHandle,
       settings: serde_json::Value,
   ) -> Result<(), String>
   ```

3. **reset_settings** - 重置设置为默认值
   ```rust
   pub async fn reset_settings(app: tauri::AppHandle) -> Result<(), String>
   ```

4. **get_setting** - 获取单个设置项
   ```rust
   pub async fn get_setting(app: tauri::AppHandle, key: String) -> Result<Option<serde_json::Value>, String>
   ```

5. **save_setting** - 保存单个设置项
   ```rust
   pub async fn save_setting(
       app: tauri::AppHandle,
       key: String,
       value: serde_json::Value,
       category: Option<String>,
   ) -> Result<(), String>
   ```

6. **get_settings_by_category** - 按分类获取设置
   ```rust
   pub async fn get_settings_by_category(
       app: tauri::AppHandle,
       category: String,
   ) -> Result<serde_json::Value, String>
   ```

### 前端 (Vue/TypeScript)

在 `frontend/src/components/settings/SettingsPanel.vue` 中，实现了以下功能：

1. **loadSettings()** - 从后端加载所有设置
   - 加载用户设置（主题、字体大小、自动保存）
   - 加载工作区设置
   - 加载 CLI 工具路径
   - 加载环境变量
   - 加载 AI 模型配置
   - 加载 Code CLI 配置

2. **saveSettings()** - 保存所有设置到后端
   - 保存用户设置
   - 保存工作区设置
   - 保存 CLI 工具路径
   - 保存环境变量（包括敏感信息标记）
   - 保存 AI 模型配置（包括 API 密钥）
   - 保存 Code CLI 配置

3. **resetSettings()** - 重置所有设置为默认值

## 设置键名规范

设置使用点号分隔的命名空间：

### 用户设置 (user.*)
- `user.theme` - 主题（'light' | 'dark'）
- `user.fontSize` - 字体大小（number）
- `user.autoSave` - 自动保存（boolean）

### 工作区设置 (workspace.*)
- `workspace.current` - 当前工作区名称
- `workspace.dataDirectory` - 数据目录路径

### CLI 工具设置 (cli.*)
- `cli.nodejs` - Node.js 路径
- `cli.python` - Python 路径
- `cli.git` - Git 路径
- `cli.code.{index}` - Code CLI 配置（JSON 格式）

### 环境变量 (env.*)
- `env.{变量名}` - 环境变量配置（JSON 格式）
  ```json
  {
    "value": "变量值",
    "isSecret": true/false
  }
  ```

### AI 模型设置 (ai.*)
- `ai.models.{index}` - AI 模型配置（JSON 格式）
  ```json
  {
    "name": "模型名称",
    "endpoint": "API端点",
    "apiKey": "API密钥"
  }
  ```

## 使用方式

### 在组件中使用

```typescript
import { getSettings, saveSettings } from '@/services/tauri/commands';

// 加载设置
const settings = await getSettings();
console.log(settings['user.theme']); // 'light' or 'dark'

// 保存设置
await saveSettings({
  'user.theme': 'dark',
  'user.fontSize': 16,
  'user.autoSave': true
});
```

### 在设置面板中

1. 组件挂载时自动调用 `loadSettings()` 加载所有设置
2. 用户修改设置后，点击"保存设置"按钮调用 `saveSettings()`
3. 点击"恢复默认"按钮调用 `resetSettings()` 重置所有设置

## 数据持久化

所有设置都保存在 SQLite 数据库中，具体位置：
- 开发环境: `~/.code-ai-assistant/app.db`
- 生产环境: 由 Tauri 配置的应用数据目录

## 安全性考虑

1. **敏感信息**: 环境变量可标记为 `isSecret`，在界面上显示为 `********`
2. **API 密钥**: AI 模型的 API 密钥在显示时始终显示为 `********`
3. **数据加密**: 建议在后端对敏感信息进行加密存储（待实现）

## 下一步改进

1. [ ] 实现敏感信息的加密存储
2. [ ] 添加设置验证逻辑
3. [ ] 实现设置导入/导出功能
4. [ ] 添加设置版本管理
5. [ ] 实现设置同步功能
6. [ ] 添加设置备份和恢复功能
7. [ ] 实现目录浏览对话框功能（`browseDirectory()`）
