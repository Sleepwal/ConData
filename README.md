# ConData - PostgreSQL 数据库连接器

一款基于 Tauri + Vue 3 + TypeScript 开发的现代化数据库连接管理工具，专注于提供简洁高效的 PostgreSQL 数据库操作体验。

## 技术架构

### 后端技术栈
- **Tauri 2.x** - 跨平台桌面应用框架，提供原生性能和安全性
- **Rust** - 系统级编程语言，确保高性能和内存安全
- **tokio-postgres** - 异步 PostgreSQL 客户端
- **deadpool-postgres** - 高性能连接池管理

### 前端技术栈
- **Vue 3** - 渐进式 JavaScript 框架，采用 Composition API
- **TypeScript** - 类型安全的 JavaScript 超集
- **Pinia** - Vue 官方推荐的状态管理方案
- **Vue Router** - 单页应用路由管理

## 核心功能

### 1. 数据库连接管理
- 支持多连接配置保存和管理
- 连接参数配置：主机、端口、数据库、用户名、密码、SSL模式
- 连接状态实时显示和监控
- 一键测试连接功能

### 2. SQL 查询执行
- 内置 SQL 编辑器，支持语法高亮
- 查询结果表格展示，支持大数据量分页
- 查询历史记录自动保存
- 执行时间和结果行数统计

### 3. 数据库对象浏览
- 自动加载数据库表列表
- 点击表名快速生成查询语句
- 支持查看表结构信息

### 4. 数据安全
- 连接配置本地加密存储
- 密码安全处理
- 参数化查询防止 SQL 注入

## 项目结构

```
ConData/
├── src-tauri/                    # Tauri 后端代码
│   ├── src/
│   │   ├── commands/             # Tauri 命令处理器
│   │   │   ├── connection.rs     # 连接管理命令
│   │   │   └── query.rs          # 查询执行命令
│   │   ├── db/                   # 数据库相关模块
│   │   │   └── pool.rs           # 连接池管理
│   │   ├── models/               # 数据模型定义
│   │   │   ├── connection.rs     # 连接配置模型
│   │   │   └── query.rs          # 查询结果模型
│   │   ├── services/             # 业务服务层
│   │   │   ├── connection_manager.rs  # 连接配置持久化
│   │   │   └── postgres_service.rs    # PostgreSQL 服务
│   │   ├── error.rs              # 错误处理定义
│   │   ├── lib.rs                # 库入口
│   │   └── main.rs               # 应用入口
│   └── Cargo.toml                # Rust 依赖配置
├── src/                          # Vue 前端代码
│   ├── api/                      # API 接口封装
│   │   └── index.ts              # Tauri 命令调用封装
│   ├── components/               # Vue 组件
│   │   ├── connection/           # 连接管理组件
│   │   │   ├── ConnectionForm.vue    # 连接配置表单
│   │   │   └── ConnectionList.vue    # 连接列表
│   │   ├── query/                # 查询执行组件
│   │   │   ├── QueryEditor.vue       # SQL 编辑器
│   │   │   ├── QueryResult.vue       # 结果展示
│   │   │   └── QueryHistory.vue      # 查询历史
│   │   └── common/               # 通用组件
│   │       └── AppHeader.vue     # 应用头部导航
│   ├── router/                   # 路由配置
│   │   └── index.ts              # 路由定义
│   ├── stores/                   # Pinia 状态管理
│   │   ├── connection.ts         # 连接状态
│   │   └── query.ts              # 查询状态
│   ├── types/                    # TypeScript 类型定义
│   │   └── index.ts              # 类型声明
│   ├── views/                    # 页面视图
│   │   ├── HomeView.vue          # 首页仪表盘
│   │   ├── ConnectionView.vue    # 连接管理页
│   │   └── QueryView.vue         # 查询执行页
│   ├── App.vue                   # 根组件
│   └── main.ts                   # 前端入口
├── package.json                  # Node.js 依赖配置
└── vite.config.ts                # Vite 构建配置
```

## 开发环境要求

### 必需软件
- **Node.js** >= 18.0.0
- **pnpm** >= 8.0.0 (推荐) 或 npm/yarn
- **Rust** >= 1.70.0
- **PostgreSQL** >= 12.0 (用于测试)

### 推荐 IDE 配置
- **VS Code** 配合以下插件：
  - Vue - Official (Volar) - Vue 3 官方插件
  - Tauri - Tauri 官方插件
  - rust-analyzer - Rust 语言支持

## 快速开始

### 1. 克隆项目
```bash
git clone <repository-url>
cd ConData
```

### 2. 安装依赖
```bash
# 安装前端依赖
pnpm install

# Rust 依赖会在构建时自动安装
```

### 3. 启动开发服务器
```bash
# 同时启动前端开发服务器和 Tauri 应用
pnpm tauri dev
```

### 4. 构建生产版本
```bash
# 构建优化后的生产版本
pnpm tauri build
```

## 使用指南

### 创建数据库连接

1. 点击顶部导航栏的"连接管理"
2. 点击"新建连接"按钮
3. 填写连接信息：
   - **连接名称**: 给连接起个名字，如"本地开发库"
   - **主机地址**: PostgreSQL 服务器地址，本地使用 `localhost`
   - **端口**: 默认 `5432`
   - **数据库名称**: 要连接的数据库名
   - **用户名**: 数据库用户名
   - **密码**: 数据库密码
   - **SSL 模式**: 根据服务器配置选择
4. 点击"测试连接"验证配置
5. 点击"保存连接"保存配置

### 执行 SQL 查询

1. 在连接列表中点击"连接"按钮建立连接
2. 点击顶部导航栏的"查询执行"
3. 在 SQL 编辑器中输入查询语句
4. 按 `Ctrl+Enter` 或点击"执行"按钮运行查询
5. 在下方的结果区域查看查询结果

### 快捷操作

- **Ctrl+Enter**: 快速执行 SQL 查询
- **点击表名**: 自动生成 `SELECT * FROM table LIMIT 100` 语句
- **查询历史**: 点击历史记录可快速重新执行

## 配置说明

### 连接配置存储
连接配置保存在系统配置目录：
- **Windows**: `%APPDATA%/condata/connections.json`
- **macOS**: `~/Library/Application Support/condata/connections.json`
- **Linux**: `~/.config/condata/connections.json`

### SSL 模式说明
- **禁用**: 不使用 SSL 连接
- **允许**: 优先非 SSL，如果服务器要求则使用 SSL
- **优先**: 优先使用 SSL，如果服务器不支持则使用非 SSL
- **要求**: 必须使用 SSL 连接

## 常见问题

### 连接失败
1. 检查 PostgreSQL 服务是否正在运行
2. 确认主机地址和端口号正确
3. 验证用户名和密码
4. 检查防火墙设置
5. 尝试将 SSL 模式设置为"禁用"

### 查询超时
- 大数据量查询建议使用 `LIMIT` 限制返回行数
- 复杂查询可能需要优化 SQL 语句

### 应用启动失败
1. 确保 Node.js 和 Rust 版本符合要求
2. 删除 `node_modules` 和 `src-tauri/target` 后重新安装依赖
3. 检查端口是否被占用

## 开发计划

### 已实现功能
- [x] PostgreSQL 数据库连接管理
- [x] SQL 查询执行和结果展示
- [x] 连接配置持久化存储
- [x] 查询历史记录
- [x] 数据库表列表浏览

### 计划功能
- [ ] 支持更多数据库（MySQL、SQLite、SQL Server）
- [ ] 数据导出（CSV、JSON、Excel）
- [ ] 数据库结构可视化
- [ ] 查询结果图表展示
- [ ] SQL 语法自动补全
- [ ] 数据库性能监控

## 贡献指南

欢迎提交 Issue 和 Pull Request！

### 提交规范
- 使用清晰的提交信息描述改动
- 确保代码通过类型检查
- 遵循现有的代码风格

## 许可证

MIT License

## 联系方式

如有问题或建议，欢迎通过以下方式联系：
- 提交 GitHub Issue
- 发送邮件至：<your-email@example.com>

---

**ConData** - 让数据库连接更简单
