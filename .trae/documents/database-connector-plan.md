# 数据库连接器应用程序开发计划

## 项目概述
基于 Tauri 2.x + Vue 3 + TypeScript 开发一个功能完善的数据库连接器应用程序，首先实现对 PostgreSQL 数据库的适配功能。

## 技术栈
- **后端**: Rust + Tauri 2.x
- **前端**: Vue 3 + TypeScript + Vite
- **数据库**: PostgreSQL (使用 `sqlx` 或 `tokio-postgres` 驱动)
- **状态管理**: Pinia (前端)
- **UI组件**: 原生 CSS + 自定义组件

---

## 第一阶段：项目基础架构搭建

### 1.1 后端依赖配置
**文件**: `src-tauri/Cargo.toml`

添加 PostgreSQL 连接库和必要的依赖：
- `tokio-postgres` - PostgreSQL 异步驱动
- `tokio` - 异步运行时
- `serde` - 序列化/反序列化
- `anyhow` - 错误处理
- `deadpool-postgres` - 连接池管理

### 1.2 前端依赖配置
**文件**: `package.json`

添加前端依赖：
- `pinia` - 状态管理
- `vue-router` - 路由管理

### 1.3 项目结构规划

```
src-tauri/src/
├── main.rs                 # 应用入口
├── lib.rs                  # 库入口
├── commands/               # Tauri 命令模块
│   ├── mod.rs
│   ├── connection.rs       # 连接管理命令
│   ├── query.rs            # 查询执行命令
│   └── config.rs           # 配置管理命令
├── models/                 # 数据模型
│   ├── mod.rs
│   ├── connection.rs       # 连接配置模型
│   └── query.rs            # 查询结果模型
├── services/               # 业务服务层
│   ├── mod.rs
│   ├── postgres_service.rs # PostgreSQL 服务
│   └── connection_manager.rs # 连接管理器
├── db/                     # 数据库相关
│   ├── mod.rs
│   └── pool.rs             # 连接池管理
└── error.rs                # 错误定义

src/
├── main.ts
├── App.vue
├── router/                 # 路由配置
│   └── index.ts
├── stores/                 # Pinia 状态管理
│   ├── connection.ts       # 连接状态
│   └── query.ts            # 查询状态
├── components/             # 组件
│   ├── connection/
│   │   ├── ConnectionForm.vue    # 连接配置表单
│   │   ├── ConnectionList.vue    # 连接列表
│   │   └── ConnectionStatus.vue  # 连接状态显示
│   ├── query/
│   │   ├── QueryEditor.vue       # SQL编辑器
│   │   ├── QueryResult.vue       # 查询结果展示
│   │   └── QueryHistory.vue      # 查询历史
│   └── common/
│       ├── AppHeader.vue
│       └── AppSidebar.vue
├── views/                  # 页面视图
│   ├── HomeView.vue
│   ├── ConnectionView.vue
│   └── QueryView.vue
├── api/                    # Tauri API 封装
│   └── index.ts
├── types/                  # TypeScript 类型定义
│   └── index.ts
└── assets/                 # 静态资源
    └── styles/
```

---

## 第二阶段：后端核心功能实现

### 2.1 错误处理模块
**文件**: `src-tauri/src/error.rs`

定义统一的错误类型，包括：
- 数据库连接错误
- 查询执行错误
- 配置错误
- 验证错误

### 2.2 数据模型定义
**文件**: `src-tauri/src/models/connection.rs`

定义连接配置结构体：
```rust
struct ConnectionConfig {
    id: String,
    name: String,
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    ssl_mode: SslMode,
}
```

**文件**: `src-tauri/src/models/query.rs`

定义查询结果结构体：
```rust
struct QueryResult {
    columns: Vec<String>,
    rows: Vec<Vec<Value>>,
    row_count: usize,
    execution_time_ms: u64,
}
```

### 2.3 数据库连接池管理
**文件**: `src-tauri/src/db/pool.rs`

实现连接池：
- 使用 `deadpool-postgres` 管理连接池
- 支持多连接管理
- 连接健康检查

### 2.4 PostgreSQL 服务实现
**文件**: `src-tauri/src/services/postgres_service.rs`

实现核心数据库操作：
- 连接测试
- 执行查询
- 获取表列表
- 获取数据库结构信息

### 2.5 连接管理器
**文件**: `src-tauri/src/services/connection_manager.rs`

实现连接管理功能：
- 保存连接配置到本地文件
- 加载连接配置
- 删除连接配置
- 连接状态管理

### 2.6 Tauri 命令实现
**文件**: `src-tauri/src/commands/connection.rs`

实现前端可调用的命令：
- `test_connection` - 测试数据库连接
- `save_connection` - 保存连接配置
- `get_connections` - 获取所有连接配置
- `delete_connection` - 删除连接配置
- `connect` - 建立数据库连接
- `disconnect` - 断开数据库连接

**文件**: `src-tauri/src/commands/query.rs`

实现查询相关命令：
- `execute_query` - 执行 SQL 查询
- `get_tables` - 获取数据库表列表
- `get_table_schema` - 获取表结构

### 2.7 权限配置
**文件**: `src-tauri/capabilities/default.json`

配置 Tauri 权限，允许文件系统访问（用于保存连接配置）。

---

## 第三阶段：前端界面实现

### 3.1 路由配置
**文件**: `src/router/index.ts`

配置应用路由：
- `/` - 首页/仪表盘
- `/connections` - 连接管理页面
- `/query` - 查询执行页面

### 3.2 类型定义
**文件**: `src/types/index.ts`

定义 TypeScript 类型：
- `ConnectionConfig` - 连接配置类型
- `QueryResult` - 查询结果类型
- `ConnectionStatus` - 连接状态类型

### 3.3 API 封装
**文件**: `src/api/index.ts`

封装 Tauri invoke 调用：
- 连接相关 API
- 查询相关 API

### 3.4 状态管理
**文件**: `src/stores/connection.ts`

实现连接状态管理：
- 当前连接状态
- 连接配置列表
- 连接操作（连接、断开、保存配置）

**文件**: `src/stores/query.ts`

实现查询状态管理：
- 查询历史
- 当前查询结果
- 查询执行状态

### 3.5 布局组件
**文件**: `src/components/common/AppHeader.vue`

实现顶部导航栏：
- 应用标题
- 导航链接
- 主题切换

**文件**: `src/components/common/AppSidebar.vue`

实现侧边栏：
- 连接列表
- 数据库对象树（表、视图等）

### 3.6 连接管理组件
**文件**: `src/components/connection/ConnectionForm.vue`

实现连接配置表单：
- 主机地址输入
- 端口号输入（默认5432）
- 数据库名称输入
- 用户名输入
- 密码输入（加密显示）
- SSL模式选择
- 测试连接按钮
- 保存连接按钮

**文件**: `src/components/connection/ConnectionList.vue`

实现连接列表：
- 显示已保存的连接
- 连接/断开操作
- 编辑/删除操作

**文件**: `src/components/connection/ConnectionStatus.vue`

实现连接状态显示：
- 连接状态指示器
- 当前连接信息

### 3.7 查询执行组件
**文件**: `src/components/query/QueryEditor.vue`

实现 SQL 编辑器：
- 代码编辑区域
- 语法高亮（基础实现）
- 执行按钮
- 快捷键支持（Ctrl+Enter执行）

**文件**: `src/components/query/QueryResult.vue`

实现查询结果展示：
- 表格形式展示数据
- 列宽调整
- 分页支持
- 执行时间和行数显示

**文件**: `src/components/query/QueryHistory.vue`

实现查询历史：
- 历史查询列表
- 快速重新执行
- 清空历史

### 3.8 页面视图
**文件**: `src/views/HomeView.vue`

首页仪表盘：
- 快速连接入口
- 最近查询
- 使用统计

**文件**: `src/views/ConnectionView.vue`

连接管理页面：
- 连接表单
- 连接列表

**文件**: `src/views/QueryView.vue`

查询执行页面：
- SQL编辑器
- 结果展示
- 侧边栏数据库对象

### 3.9 主应用组件
**文件**: `src/App.vue`

重构主应用组件：
- 布局框架
- 全局样式
- 主题支持

---

## 第四阶段：功能完善与优化

### 4.1 错误处理
- 统一的错误提示机制
- 连接失败处理
- 查询错误处理
- 网络超时处理

### 4.2 数据验证
- 连接配置表单验证
- SQL注入防护
- 输入 sanitization

### 4.3 性能优化
- 查询结果虚拟滚动（大数据量）
- 连接池优化
- 查询超时控制

### 4.4 用户体验优化
- 加载状态指示
- 操作确认对话框
- 通知提示系统
- 键盘快捷键

---

## 第五阶段：测试与部署

### 5.1 功能测试
- 连接配置测试
- 查询执行测试
- 连接管理测试
- 错误场景测试

### 5.2 构建配置
**文件**: `src-tauri/tauri.conf.json`

配置应用元数据：
- 应用名称和版本
- 窗口配置
- 构建配置

### 5.3 文档
- README 更新
- 使用说明
- 开发文档

---

## 开发顺序建议

1. **后端基础** → 先完成后端核心功能，确保数据流正常
2. **前端框架** → 搭建前端基础架构和路由
3. **连接功能** → 实现连接配置和状态管理
4. **查询功能** → 实现查询执行和结果展示
5. **完善优化** → 添加错误处理、验证、优化

---

## 技术要点

### 安全性
- 密码加密存储（使用系统 keychain 或加密文件）
- SQL 注入防护（使用参数化查询）
- 输入验证和 sanitization

### 性能
- 连接池复用
- 查询结果分页
- 大数据量虚拟滚动

### 用户体验
- 实时连接状态
- 查询执行进度
- 友好的错误提示
- 响应式布局

---

## 后续扩展

- 支持更多数据库（MySQL、SQLite、SQL Server等）
- 数据可视化功能
- 导出查询结果（CSV、JSON、Excel）
- 数据库结构比较和同步
