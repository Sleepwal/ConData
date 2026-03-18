# 使用 GitHub Actions 自动打包 Tauri 应用为 EXE

## 目标
配置 GitHub Actions 工作流，实现自动化构建 Windows 可执行文件（.exe）和安装程序。

## 技术方案
使用 GitHub Actions 的 Windows 运行器，通过 Tauri 官方提供的 GitHub Action 实现自动化构建。

---

## 第一阶段：GitHub Actions 工作流配置

### 1.1 创建工作流文件
**文件**: `.github/workflows/build.yml`

创建 GitHub Actions 工作流配置文件，定义构建流程。

### 1.2 配置触发条件
设置工作流触发方式：
- 推送到 main 分支时触发
- 创建标签（tag）时触发（用于发布版本）
- 手动触发（workflow_dispatch）

### 1.3 配置构建环境
使用 GitHub 提供的 Windows 运行器：
- `windows-latest` - Windows Server 2022
- 预装 Rust、Node.js、Visual Studio Build Tools

---

## 第二阶段：工作流步骤详解

### 2.1 检出代码
使用 `actions/checkout@v4` 检出仓库代码。

### 2.2 安装依赖
- 安装 Node.js 依赖（pnpm install）
- 安装 Rust 工具链
- 安装 Tauri CLI

### 2.3 前端构建
执行 `pnpm run build` 生成生产环境前端代码。

### 2.4 Tauri 构建
使用 `tauri-apps/tauri-action@v0` 官方 Action 执行构建：
- 自动处理 Windows 依赖
- 生成 .exe 和 .msi 安装程序
- 上传到构建产物

### 2.5 上传构建产物
使用 `actions/upload-artifact@v4` 上传生成的文件：
- 独立的 .exe 文件
- .msi 安装程序

---

## 第三阶段：发布配置（可选）

### 3.1 自动创建 Release
配置工作流在创建标签时自动创建 GitHub Release。

### 3.2 自动上传 Release 资产
将构建好的安装程序自动上传到 Release 页面。

### 3.3 生成更新日志
自动提取提交信息生成更新日志。

---

## 工作流配置示例

```yaml
name: Build and Release

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  workflow_dispatch:
  release:
    types: [created]

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: 'pnpm'

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Install dependencies
        run: pnpm install

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: app-v__VERSION__
          releaseName: 'App v__VERSION__'
          releaseBody: 'See the assets to download this version and install.'
          releaseDraft: true
          prerelease: false
```

---

## 第四阶段：高级配置

### 4.1 多平台构建（可选）
配置同时构建 Windows、macOS、Linux 版本。

### 4.2 代码签名（可选）
配置使用代码签名证书对安装程序签名：
- 使用 GitHub Secrets 存储证书
- 配置签名步骤

### 4.3 缓存优化
配置依赖缓存加速构建：
- Rust 依赖缓存
- Node.js 依赖缓存

---

## 第五阶段：测试和验证

### 5.1 手动触发测试
通过 GitHub Actions 界面手动触发工作流测试。

### 5.2 验证构建产物
下载构建产物验证：
- 可执行文件能否正常运行
- 安装程序能否正常安装
- 所有功能是否正常

### 5.3 发布测试
创建测试标签验证自动发布流程。

---

## 文件清单

需要创建或修改的文件：

| 文件路径 | 说明 |
|---------|------|
| `.github/workflows/build.yml` | GitHub Actions 工作流配置 |
| `.github/workflows/release.yml` | 发布工作流配置（可选） |

---

## 执行步骤

1. **创建工作流目录** → 创建 `.github/workflows/` 目录
2. **编写工作流配置** → 创建 `build.yml` 文件
3. **提交到仓库** → 推送代码到 GitHub
4. **触发构建** → 手动触发或推送代码触发
5. **下载产物** → 从 Actions 页面下载构建的 EXE
6. **验证测试** → 测试生成的安装程序

---

## 输出文件位置

构建完成后，在 GitHub Actions 页面可以下载：
- `ConData.exe` - 独立可执行文件
- `ConData.msi` - Windows 安装程序
- `ConData-setup.exe` - NSIS 安装程序（如配置）

---

## 优势

- **自动化** - 每次推送自动构建
- **一致性** - 统一的构建环境
- **可追溯** - 每次构建都有记录
- **易于分发** - 直接提供下载链接
- **免费** - GitHub Actions 对公共仓库免费

---

## 注意事项

1. 确保仓库是公开的（GitHub Actions 免费额度更多）
2. 首次构建需要下载依赖，耗时较长（5-15分钟）
3. 私有仓库有 Actions 使用时长限制
4. Windows 运行器已预装 Visual Studio Build Tools
