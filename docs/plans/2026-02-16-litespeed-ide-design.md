# LiteSpeed-IDE 设计文档

**项目代号**: LiteSpeed-IDE
**目标**: 为高级开发者打造极致性能的跨平台轻量化 IDE
**设计日期**: 2026-02-16
**GitHub**: https://github.com/Chang-Augenweide/LiteSpeed-IDE

---

## 1. 项目概述

### 1.1 核心定位
- **目标用户**: 高级开发者（追求极致性能）
- **核心价值**: 比 VS Code 轻 5 倍、快 3 倍，同时具备插件系统
- **关键技术**:
  - Rust + Tauri（性能 + 跨平台）
  - CodeMirror 6（轻量编辑器）
  - WASM 插件系统（安全 + 跨语言）

### 1.2 核心指标
| 指标 | 目标 | VS Code 参考 |
|------|------|--------------|
| 安装包 | 15-25 MB | ~100 MB |
| 常驻内存 | 80-120 MB | ~500 MB |
| 启动时间 | < 0.5s | ~2s |
| 输入延迟 | < 16ms | ~30-50ms |
| 大文件打开(100MB) | < 300ms | ~2s |

---

## 2. 技术架构

### 2.1 技术栈
```
┌─────────────────────────────────────┐
│  Frontend (TypeScript + React)      │
│  - CodeMirror 6 编辑器              │
│  - Tauri 前端 API                   │
│  - 现代简约 UI (Glassmorphism)      │
└──────────┬──────────────────────────┘
           │ Tauri Bridge
┌──────────▼──────────────────────────┐
│  Tauri (Webview + IPC)              │
│  - 跨进程通信                        │
│  - 系统原生 Webview                 │
└──────────┬──────────────────────────┘
           │
┌──────────▼──────────────────────────┐
│  Core (Rust)                         │
│  ├── 文件系统 (FSManager)           │
│  ├── 插件 Runtime (WASM)             │
│  ├── LSP 客户端                      │
│  ├── Git 集成 (`git2`)              │
│  └── 终端管理 (`pty`)                │
└─────────────────────────────────────┘
```

### 2.2 技术选型理由

|技术| 版本/类型| 选型理由 |
|---|---|---|
| Rust | 1.70+ | 零成本抽象、内存安全、性能极致 |
| Tauri | 2.x | 比 Electron 小 10 倍，系统原生 Webview |
| TypeScript | 5.x | 开发效率高，生态成熟 |
| React | 18.x | 组件化开发，易于维护 |
| CodeMirror 6 | 6.x | 比 Monaco 小 5 倍，模块化 |
| WASM | 标准 | 插件沙箱、跨语言、安全 |
| `git2` | libgit2 绑定 | 原生 Git 支持 |
| `notify` | 文件监听 | 增量更新，低开销 |

---

## 3. UI/UX 设计

### 3.1 设计风格
**现代简约 + 轻质感（Clean & Modern + Glassmorphism）**

- **视觉语言**: 清晰、克制、功能优先
- **质感倾向**: 柔和阴影、半透明背景、微妙渐变
- **主色**: `#6366f1`（靛蓝），占比 ≤ 20%
- **中性色**: 深灰背景 + 浅灰文本，占比 ≥ 70%

### 3.2 色彩系统（深色模式）
```css
--primary: #6366f1;           /* 靛蓝 */
--primary-hover: #4f46e5;
--primary-light: #e0e7ff;

--success: #10b981;           /* 绿色 */
--warning: #f59e0b;           /* 橙色 */
--error: #ef4444;             /* 红色 */

--bg-primary: #0f172a;         /* 主背景 */
--bg-card: rgba(30, 41, 59, 0.8);   /* 半透明 + 毛玻璃 */
--bg-hover: rgba(51, 65, 85, 0.9);
--bg-active: rgba(67, 56, 202, 0.3);

--text-primary: #f8fafc;      /* 主文本 */
--text-secondary: #94a3b8;   /* 次要文本 */
--text-hint: #64748b;         /* 提示文本 */

--border: rgba(148, 163, 184, 0.12); /* 极浅边框 */
```

### 3.3 核心组件规范

**卡片**
```css
background: var(--bg-card);
backdrop-filter: blur(12px);
border-radius: 12px;
padding: 16px;
box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
```

**按钮**
```css
btn-primary {
  background: var(--primary);
  border-radius: 10px;
  padding: 10px 20px;
  transition: all 0.15s ease;
}
```

**编辑器**
```css
background: #0b0f19;
font-family: 'JetBrains Mono', monospace;
font-size: 14px;
line-height: 1.6;
caret-color: var(--primary);
```

### 3.4 布局结构
```
+---------------------------------------+
|  菜单栏 (40px)                         |
+-------+---------------+---------------+
|       |               |               |
| 文件树|   编辑器区    |  信息面板     |
| (240px)| (flex-1)      | (280px)       |
|       |               +---------------+
|       |               |  终端         |
|       |               | (200px)       |
+-------+---------------+---------------+
|  状态栏 (28px)                         |
+---------------------------------------+
```

---

## 4. 插件系统

### 4.1 插件架构（WASM 沙箱）

**设计目标**:
- ✅ 完全隔离（插件崩溃不影响 IDE）
- ✅ 跨语言支持（Rust, Go, C++, AssemblyScript）
- ✅ 受限 API（权限控制）
- ✅ 热加载（修改插件无需重启 IDE）

### 4.2 插件结构
```
/plugin-name
├── plugin.wasm      # 编译后的 WASM 二进制
├── manifest.json    # 元数据（名称、版本、权限）
├── assets/         # 图标、配置
└── README.md       # 说明文档
```

### 4.3 插件 API（简化版）

```rust
pub trait Plugin {
    fn on_open(&self, file_path: Path) -> Result<()>;
    fn on_save(&self, content: &str) -> Result<()>;
    fn commands(&self) -> Vec<Command>;
    fn complete(&self, input: &str) -> Vec<Completion>;
}

impl PluginContext {
    pub fn read_file(&self, path: Path) -> Result<String>;
    pub fn write_file(&self, path: Path, content: String) -> Result<()>;
    pub fn spawn_process(&self, cmd: &str) -> Result<Process>;
    pub fn show_notification(&self, msg: &str);
}
```

### 4.4 权限控制
插件必须在 `manifest.json` 中声明权限：
```json
{
  "permissions": ["read_files", "write_files", "spawn_process"]
}
```

---

## 5. 编辑器核心

### 5.1 CodeMirror 6 集成

**优化策略**:
- ✅ 虚拟滚动（10MB+ 文件流畅）
- ✅ 懒加载语法高亮（Tree-sitter WASM）
- ✅ 增量渲染（只在必要时更新 DOM）

**性能关键**:
```rust
// 前端直接渲染，后端异步同步
Frontend Render (0ms) → User Sees
   ↓
Backend Sync (debounce 300ms) → Update LRU Cache
```

### 5.2 大文件处理

**虚拟滚动实现**:
```rust
pub struct VirtualScroll {
    visible_range: Range<usize>,  // 当前可见行
    buffer_front: Vec<String>,   // 向上缓冲
    buffer_back: Vec<String>,    // 向下缓冲
}
```

**预期性能**:
- 100MB 文件打开: < 300ms
- 输入延迟: < 16ms (60fps)
- 100K 行搜索: < 100ms

### 5.3 搜索与索引

**使用 `ripgrep` 作为后端引擎**:
- 极速全文搜索（Rust 实现）
- 前端查询 → 返回匹配行 + context
- 1M 行文件搜索: < 200ms

---

## 6. 文件系统

### 6.1 FSManager 架构

```rust
pub struct FSManager {
    watcher: RecommendedWatcher,  // 文件监听
    cache: LruCache<PathBuf, Content>, // LRU 缓存
    indexer: SearchIndex,         // 全文索引
}
```

### 6.2 文件操作流程

```
用户打开文件
   ↓
检查缓存 → Hit? 直接返回
   ↓ Miss
异步读取 → 更新缓存 → 触发 LSP
```

**特性**:
- ✅ 实时监听（`notify` crate）
- ✅ 智能缓存（热点文件常驻内存）
- ✅ 非阻塞读取（`tokio` 异步 I/O）
- ✅ 符号链接跟踪
- ✅ 二进制文件检测（> 50MB 跳过）

---

## 7. LSP 集成

### 7.1 支持的语言（MVP）

| 语言 | LSP Server | 集成方式 |
|---|---|---|
| Python | `pyright` / `jedi-language-server` | ✅ |
| Go | `gopls` | ✅ |
| Rust | `rust-analyzer` | ✅ |
| TypeScript | `tsserver` | ✅ |
| 其他 | 通用 LSP 协议 | ⚠️ 部分支持 |

### 7.2 LSP 架构

```rust
pub struct LSPClient {
    process: Child,              // LSP 进程
    server_capabilities: Capability,
    workspace_folders: Vec<Path>,
}

impl LSPClient {
    pub async fn completion(&self, uri: &str, line: usize, col: usize)
        -> Vec<CompletionItem>;
    pub async fn definition(&self, uri: &str, line: usize, col: usize)
        -> Location;
    pub async fn diagnostics(&self, uri: &str)
        -> Vec<Diagnostic>;
}
```

### 7.3 性能优化
- LSP 进程池（复用）
- 请求去重（同时只发送一个相同请求）
- 懒加载（首次打开项目才启动）
- 超时机制（5s 无响应则降级）

### 7.4 支持
- ✅ 代码补全（基于 LSP）
- ✅ 跳转定义
- ✅ 错误诊断
- ✅ 文档提示
- ❌ 重构（MVP 阶段不支持）

---

## 8. Git 集成

### 8.1 GitManager 架构

```rust
pub struct GitManager {
    repo: git2::Repository,
    status_cache: GitStatus,
}

pub struct GitStatus {
    modified: Vec<PathBuf>,
    untracked: Vec<PathBuf>,
    staged: Vec<PathBuf>,
    branch: String,
    ahead: usize,
    behind: usize,
}
```

### 8.2 Git 操作（MVP）

| 操作 | 用途 | 支持状态 |
|---|---|---|
| `status` | 实时状态 | ✅ |
| `add` | 添加到暂存区 | ✅ |
| `commit` | 提交 | ✅ |
| `push/pull` | 同步 | ✅ |
| `checkout` | 切换分支 | ✅ |
| `branch` | 创建/删除分支 | ✅ |
| `diff` | 简化 diff | ✅ |

### 8.3 不支持（保持轻量）
- ❌ 复杂合并冲突解决
- ❌ 交互式 rebase
- ❌ Git blame

---

## 9. 终端集成

### 9.1 技术选型

| 平台 | 终端方案 |
|---|---|
| Windows | `windows-terminal` Webview |
| macOS | `iTerm2` / `Terminal.app` |
| Linux | `xterm.js`（最小配置） |

### 9.2 终端特性

- ✅ 多标签页（Ctrl+` 新增）
- ✅ 热键绑定（Ctrl+T, Ctrl+W）
- ✅ 大小可调整（拖动底部边框）
- ✅ 原生 Shell（zsh, bash, fish, PowerShell）
- ✅ 环境变量继承

### 9.3 不支持
- ❌ 终端分屏
- ❌ 颜色主题自定义
- ❌ 复杂终端插件

---

## 10. 项目结构

```
LiteSpeed-IDE/
├── src/
│   ├── main.rs                    # Rust 入口
│   ├── core/                      # Core 模块
│   │   ├── fs.rs                  # 文件系统
│   │   ├── plugin.rs              # 插件 runtime
│   │   ├── lsp.rs                 # LSP 客户端
│   │   └── git.rs                 # Git 集成
│   └── plugins/                   # 内置插件
│       ├── python/
│       ├── go/
│       └── rust/
├── frontend/                      # TypeScript + React
│   ├── src/
│   │   ├── components/            # UI 组件
│   │   ├── editor/                # CodeMirror 6
│   │   └── hooks/                 # React Hooks
│   └── package.json
├── src-tauri/                     # Tauri 配置
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                          # 文档
│   └── README.md
└── README.md
```

---

## 11. 开发里程碑

### Phase 1: MVP（2-3 个月）
- ✅ 基础窗口与文件树
- ✅ CodeMirror 6 编辑器
- ✅ 文件打开/保存/重命名
- ✅ 终端嵌入
- ✅ 基础 Git 操作

### Phase 2: 插件系统（1-2 个月）
- ✅ WASM 插件 runtime
- ✅ 插件市场（本地）
- ✅ 3 个内置插件（Python, Go, Rust LSP）
- ✅ 插件权限管理

### Phase 3: LSP 集成（1-2 个月）
- ✅ LSP 客户端（通用）
- ✅ 代码补全
- ✅ 跳转定义
- ✅ 错误诊断

### Phase 4: 优化与完善（1 个月）
- ✅ 性能调优（启动 < 0.5s, 内存 < 120MB）
- ✅ UI/UX 完善（深色模式）
- ✅ 文档完整
- ✅ GitHub Release v1.0.0

**总工期**: 6-8 个月

---

## 12. 后续扩展方向

### v1.x
- 更多语言 LSP 集成
- 插件市场（云端）
- 多窗口支持

### v2.0（长期）
- 完全兼容 VS Code 插件 API（子集）
- 协作编辑（CRDT）
- AI 辅助编程集成

---

## 附录

### A. 技术选型对比

| 框架 | 优点 | 缺点 | 选择 |
|---|---|---|---|
| Electron | 生态成熟、开发快 | 体积大、内存高 | ❌ |
| Tauri | 体积小、性能高 | 生态较小 | ✅ |
| Monaco | 功能强大 | 体积大 | ❌ |
| CodeMirror 6 | 轻量、模块化 | 功能较少 | ✅ |

### B. 关键依赖（Rust）

| Crate | 用途 |
|---|---|
| `tauri` | 桌面框架 |
| `tokio` | 异步运行时 |
| `git2` | Git 集成 |
| `notify` | 文件监听 |
| `lru` | LRU 缓存 |
| `tree-sitter` | 语法分析 |
| `wasmer` | WASM runtime |
| `ripgrep` | 搜索引擎 |

### C. 关键依赖（TypeScript）

| Package | 用途 |
|---|---|
| `@codemirror/state` | CodeMirror 状态 |
| `@codemirror/view` | CodeMirror 视图 |
| `@codemirror/language` | 语言支持 |
| `react` | UI 框架 |
| `@tauri-apps/api` | Tauri API |

---

**文档版本**: v1.0
**最后更新**: 2026-02-16
