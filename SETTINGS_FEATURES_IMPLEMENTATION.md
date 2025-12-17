# 设置功能实现文档

## 功能概述

实现了完整的设置管理功能，包括：
1. CLI 工具路径设置
2. 环境变量管理
3. AI 模型管理
4. Code CLI 管理

## 功能详情

### 1. CLI 工具路径设置

**功能说明：**
- 配置 Node.js、Python 和 Git 的命令路径
- 支持绝对路径或命令名称
- 自动保存到后端数据库

**使用方式：**
- 在对应输入框中输入路径
- 点击"保存设置"按钮保存

**示例路径：**
- Node.js: `/usr/bin/node` 或 `node`
- Python: `/usr/bin/python3` 或 `python`
- Git: `/usr/bin/git` 或 `git`

### 2. 环境变量管理

**功能列表：**
- ✅ 添加环境变量
- ✅ 编辑环境变量
- ✅ 删除环境变量
- ✅ 敏感信息隐藏（密码、密钥等）
- ✅ 表单验证
- ✅ 空状态提示

**操作步骤：**

#### 添加环境变量
1. 点击"添加环境变量"按钮
2. 填写变量名和变量值
3. 可选：勾选"敏感信息"隐藏显示
4. 点击"确定"保存

#### 编辑环境变量
1. 点击表格中的编辑图标
2. 修改变量信息
3. 点击"确定"保存更改

#### 删除环境变量
1. 点击表格中的删除图标
2. 确认删除操作

**验证规则：**
- 变量名不能为空
- 变量值不能为空

### 3. AI 模型管理

**功能列表：**
- ✅ 添加 AI 模型
- ✅ 编辑 AI 模型
- ✅ 删除 AI 模型
- ✅ API 密钥隐藏显示
- ✅ 编辑时可选择保留原密钥
- ✅ 表单验证
- ✅ 空状态提示

**字段说明：**
- **模型名称**：模型的标识名称（如：Claude-3.5-Sonnet、GPT-4）
- **API端点**：API 服务器地址（如：api.anthropic.com/v1）
- **API密钥**：访问 API 的密钥（显示为 `********`）

**操作步骤：**

#### 添加模型
1. 点击"添加模型"按钮
2. 填写模型名称、API端点和API密钥
3. 点击"确定"保存

#### 编辑模型
1. 点击表格中的编辑图标
2. 修改模型信息
3. API密钥可以留空以保持原密钥不变
4. 点击"确定"保存更改

#### 删除模型
1. 点击表格中的删除图标
2. 确认删除操作

**验证规则：**
- 模型名称不能为空
- API端点不能为空
- 新增模型时 API密钥不能为空
- 编辑模型时 API密钥可以留空

### 4. Code CLI 管理

**功能列表：**
- ✅ 添加 Code CLI
- ✅ 编辑 Code CLI
- ✅ 删除 Code CLI
- ✅ 表单验证
- ✅ 空状态提示

**字段说明：**
- **CLI名称**：CLI 工具的标识名称（如：OpenAI-Codex）
- **命令路径**：CLI 可执行文件路径（如：/usr/bin/codex）
- **参数**：命令行参数（如：--model gpt-4）

**操作步骤：**

#### 添加 Code CLI
1. 点击"添加 Code CLI"按钮
2. 填写 CLI 名称、命令路径和参数
3. 点击"确定"保存

#### 编辑 Code CLI
1. 点击表格中的编辑图标
2. 修改 CLI 信息
3. 点击"确定"保存更改

#### 删除 Code CLI
1. 点击表格中的删除图标
2. 确认删除操作

**验证规则：**
- CLI名称不能为空
- 命令路径不能为空
- 参数可选

## 数据持久化

所有设置数据都会保存到后端 SQLite 数据库中：

- **CLI 工具路径**：保存在 `settings` 表，key 为 `cli.nodejs`、`cli.python`、`cli.git`
- **环境变量**：保存在 `settings` 表，key 为 `env.{变量名}`
- **AI 模型**：保存在 `settings` 表，key 为 `ai.models.{索引}`
- **Code CLI**：保存在 `settings` 表，key 为 `cli.code.{索引}`

### 保存操作

点击"保存设置"按钮会：
1. 收集所有设置数据
2. 调用后端 API 保存到数据库
3. 显示成功消息

### 恢复默认

点击"恢复默认"按钮会：
1. 清除所有自定义设置
2. 恢复到默认配置
3. 需要确认操作

## 用户体验优化

### 1. 表单验证
- 实时验证必填字段
- 显示友好的错误提示
- 防止提交无效数据

### 2. 操作反馈
- 添加成功提示
- 编辑成功提示
- 删除成功提示
- 保存成功提示

### 3. 确认对话框
- 删除操作需要确认
- 恢复默认需要确认
- 防止误操作

### 4. 空状态提示
- 表格为空时显示友好提示
- 引导用户添加数据

### 5. 敏感信息保护
- 环境变量可标记为敏感信息
- 敏感信息显示为 `********`
- API 密钥自动隐藏
- 编辑时可选择保留原密钥

## 技术实现

### 前端技术栈
- Vue 3 Composition API
- TypeScript
- Element Plus UI 组件
- Pinia 状态管理

### 核心函数

#### 环境变量
- `addEnvVar()` - 添加/更新环境变量
- `editEnvVar(index)` - 编辑环境变量
- `removeEnvVar(index)` - 删除环境变量
- `openEnvVarDialog()` - 打开添加对话框

#### AI 模型
- `addAiModel()` - 添加/更新 AI 模型
- `editAiModel(index)` - 编辑 AI 模型
- `removeAiModel(index)` - 删除 AI 模型
- `openAiModelDialog()` - 打开添加对话框

#### Code CLI
- `addCodeCli()` - 添加/更新 Code CLI
- `editCodeCli(index)` - 编辑 Code CLI
- `removeCodeCli(index)` - 删除 Code CLI
- `openCodeCliDialog()` - 打开添加对话框

#### 设置管理
- `saveSettings()` - 保存所有设置
- `loadSettings()` - 加载设置
- `resetSettings()` - 恢复默认

### 数据结构

```typescript
// 环境变量
interface EnvVar {
  name: string;
  value: string;
  isSecret: boolean;
}

// AI 模型
interface AiModel {
  name: string;
  endpoint: string;
  apiKey: string;
}

// Code CLI
interface CodeCli {
  name: string;
  command: string;
  args: string;
}

// CLI 路径
interface CliPaths {
  nodejs: string;
  python: string;
  git: string;
}
```

## 后端 API

### 获取设置
```rust
#[tauri::command]
pub async fn get_settings(db: State<'_, DatabaseConnection>) -> Result<HashMap<String, String>>
```

### 保存设置
```rust
#[tauri::command]
pub async fn save_settings(
    db: State<'_, DatabaseConnection>,
    settings: HashMap<String, String>
) -> Result<()>
```

## 测试建议

### 1. 功能测试
- [ ] 测试添加各类设置
- [ ] 测试编辑功能
- [ ] 测试删除功能
- [ ] 测试保存和加载
- [ ] 测试恢复默认

### 2. 验证测试
- [ ] 测试必填字段验证
- [ ] 测试空值处理
- [ ] 测试特殊字符

### 3. 边界测试
- [ ] 测试大量数据
- [ ] 测试长文本
- [ ] 测试数据库连接失败

### 4. 用户体验测试
- [ ] 测试操作反馈
- [ ] 测试确认对话框
- [ ] 测试空状态显示

## 已知限制

1. 环境变量和 CLI 配置通过索引保存，删除后索引可能不连续
2. API 密钥编辑时无法查看原密钥
3. 没有配置导入/导出功能

## 未来改进

1. 添加配置导入/导出功能
2. 添加配置模板功能
3. 添加环境变量分组
4. 添加 CLI 工具版本检测
5. 添加 API 密钥有效性验证
6. 添加配置备份恢复功能

## 相关文件

- 前端组件：`frontend/src/components/settings/SettingsPanel.vue`
- 后端命令：`src-tauri/src/tauri/commands.rs`
- 数据库仓储：`src-tauri/src/database/repositories/settings_repository.rs`
- 类型定义：`frontend/src/utils/types/index.ts`
