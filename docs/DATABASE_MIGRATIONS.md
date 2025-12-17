# 数据库迁移指南

本项目使用 `sea-orm-migration` 进行数据库迁移管理。

## 迁移系统特点

- ✅ 自动版本管理
- ✅ 可回滚（up/down）
- ✅ 类型安全的 schema 定义
- ✅ 支持多种数据库（SQLite、PostgreSQL、MySQL）
- ✅ 自动在应用启动时运行

## 创建新迁移

### Windows (PowerShell)

```powershell
.\new_migration.ps1 create_users_table
```

### Linux/macOS

```bash
chmod +x new_migration.sh
./new_migration.sh create_users_table
```

### 手动创建

1. 在 `src/database/migrations/` 目录下创建新文件，命名格式：`mYYYYMMDD_HHMMSS_description.rs`

2. 文件内容模板：

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(YourTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(YourTable::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(YourTable::Name)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(YourTable::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum YourTable {
    Table,
    Id,
    Name,
}
```

3. 在 `src/database/migrations/mod.rs` 中注册迁移：

```rust
mod m20250101_000001_create_your_table;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_settings_table::Migration),
            Box::new(m20250101_000001_create_your_table::Migration), // 新增
        ]
    }
}
```

## 迁移操作示例

### 创建表

```rust
manager
    .create_table(
        Table::create()
            .table(Users::Table)
            .col(ColumnDef::new(Users::Id).integer().primary_key().auto_increment())
            .col(ColumnDef::new(Users::Email).string().unique_key().not_null())
            .to_owned(),
    )
    .await
```

### 添加列

```rust
manager
    .alter_table(
        Table::alter()
            .table(Users::Table)
            .add_column(ColumnDef::new(Users::NewColumn).string().null())
            .to_owned(),
    )
    .await
```

### 创建索引

```rust
manager
    .create_index(
        Index::create()
            .name("idx_users_email")
            .table(Users::Table)
            .col(Users::Email)
            .to_owned(),
    )
    .await
```

### 创建外键

```rust
manager
    .create_foreign_key(
        ForeignKey::create()
            .name("fk_posts_user_id")
            .from(Posts::Table, Posts::UserId)
            .to(Users::Table, Users::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned(),
    )
    .await
```

## 数据类型映射

| Rust 类型 | sea-orm-migration | SQLite 类型 |
|-----------|-------------------|-------------|
| i32, i64  | `.integer()`      | INTEGER     |
| String    | `.string()`       | TEXT        |
| bool      | `.boolean()`      | INTEGER     |
| f32, f64  | `.float()` / `.double()` | REAL |
| DateTime  | `.timestamp()`    | TEXT        |
| JSON      | `.text()`         | TEXT        |

## 列定义选项

```rust
ColumnDef::new(Column::Name)
    .string()              // 数据类型
    .not_null()            // NOT NULL 约束
    .unique_key()          // UNIQUE 约束
    .default("default")    // 默认值
    .auto_increment()      // 自动递增（主键）
```

## 自动运行

迁移会在应用启动时自动运行（参见 `src/database/connection.rs`）：

```rust
pub fn init(app: &mut App) -> AppResult<()> {
    // ...
    tauri::async_runtime::spawn(async move {
        let db = get_db_connection(&app_handle).await?;
        crate::database::migrations::run_migrations(&db).await?;
    });
    // ...
}
```

## 迁移历史

sea-orm-migration 会在数据库中创建一个 `seaql_migrations` 表来跟踪已应用的迁移。

## 最佳实践

1. **永远不要修改已应用的迁移** - 创建新迁移来修改 schema
2. **保持迁移小而专注** - 每个迁移只做一件事
3. **编写 down 方法** - 确保迁移可以回滚
4. **测试迁移** - 在开发环境测试 up 和 down
5. **添加注释** - 说明迁移的目的和影响

## 故障排除

### 迁移失败

检查日志输出：
```
Failed to run database migrations: [error details]
```

解决方案：
1. 检查迁移语法
2. 验证数据库连接
3. 查看 `seaql_migrations` 表的状态

### 重置数据库

开发环境下可以删除数据库文件重新开始：
```bash
rm -rf {data_dir}/app.db
```

应用会在下次启动时重新创建并运行所有迁移。

## 参考文档

- [sea-orm-migration 官方文档](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)
- [Schema Statement API](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/)
