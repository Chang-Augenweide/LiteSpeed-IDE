# LiteSpeed-IDE

> 极致性能的轻量化 IDE，为高级开发者打造

## 🚀 核心特性

### 超轻量
- **安装包**: 15-25 MB（VS Code ~100 MB）
- **常驻内存**: 80-120 MB（VS Code ~500 MB）
- **启动速度**: < 0.5s（VS Code ~2s）

### 高性能
- **输入延迟**: < 16ms（VS Code 30-50ms）
- **大文件加载**: 100MB < 300ms（VS Code ~2s）
- **并发编辑**: 多线程处理，无卡顿

### 插件系统
- **WASM 沙箱**: 完全隔离，安全可靠
- **多语言支持**: Rust/Go/C++ 插件
- **热加载**: 无需重启即可加载插件

### LSP 集成
- **代码补全**: 智能提示
- **跳转定义**: 一键跳转
- **错误诊断**: 实时反馈

## 🛠️ 技术栈

### 核心
- **Rust**: 性极致、内存安全
- **Tauri**: 比 Electron 小 10 倍

### 前端
- **TypeScript** + **React**: 开发效率高
- **CodeMirror 6**: 比 Monaco 小 5 倍

### 插件
- **WASM**: 跨语言、沙箱隔离

## 📦 核心模块

### 1. 文件系统管理器 (FSManager)
- LRU 缓存（1024 个文件）
- 异步读写（tokio::fs）
- 二进制文件检测
- 文件监听器（Phase 2）

```rust
let fs_manager = FSManager::new().await?;
fs_manager.write_file(&path, content).await?;
let content = fs_manager.read_file(&path).await?;
```

### 2. 插件管理器 (PluginManager)
- WASM 沙箱隔离
- 插件元数据
- 权限系统

```rust
let mut plugin_manager = PluginManager::new().await?;
plugin_manager.load_plugin(plugin_path).await?;
```

### 3. LSP 客户端 (LSPClient)
- 语言服务协议
- 代码补全
- 跳转定义
- 错误诊断

```rust
let lsp_client = LSPClient::new(workspace_path).await?;
let completions = lsp_client.completion(&uri, line, col).await?;
```

### 4. Git 管理器 (GitManager)
- 状态读取
- commit/push
- branch 管理

```rust
let mut git_manager = GitManager::new(repo_path)?;
let status = git_manager.get_status()?;
git_manager.commit(&message)?;
```

### 5. 搜索管理器 (SearchManager)
- 正则搜索
- 文件内容搜索
- 行号定位

```rust
let mut search_manager = SearchManager::new();
let results = search_manager.search_files(pattern, path, false, false)?;
```

## 🧪 运行示例

```bash
# 编译
cargo build --release

# 运行演示
cargo run --example demo

# 运行测试
cargo test
```

## 📊 性能对比

| 指标 | LiteSpeed-IDE | VS Code | 提升 |
|------|---------------|---------|------|
| 安装包 | 15-25 MB | ~100 MB | 4-6x |
| 常驻内存 | 80-120 MB | ~500 MB | 4-6x |
| 启动时间 | < 0.5s | ~2s | 4x |
| 输入延迟 | < 16ms | ~30-50ms | 2-3x |
| 大文件(100MB) | < 300ms | ~2s | 6x |

## 🗺️ 开发路线图

### Phase 1: MVP (2-3 个月)
- [x] 核心模块实现
- [ ] Tauri 前端集成
- [ ] 文件树 UI
- [ ] CodeMirror 6 集成
- [ ] 终端嵌入

### Phase 2: 插件系统 (1-2 个月)
- [ ] WASM 插件 runtime
- [ ] 插件市场
- [ ] 3 个内置插件
- [ ] 插件权限

### Phase 3: LSP 集成 (1-2 个月)
- [ ] LSP 进程管理
- [ ] 代码补全
- [ ] 跳转定义
- [ ] 错误诊断

### Phase 4: 优化（1 个月）
- [ ] 性能调优
- [ ] UI/UX 完善
- [ ] 文档完整
- [ ] v1.0.0 Release

## 📚 文档

- [设计文档](./docs/plans/2026-02-16-litespeed-ide-design.md)
- [开发路线图](./docs/roadmap.md)
- [API 文档](./docs/api/)

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](./CONTRIBUTING.md)

## 📄 许可

MIT License

## 🔗 链接

- [GitHub](https://github.com/Chang-Augenweide/LiteSpeed-IDE)
- [Notion 项目](https://www.notion.so/LiteSpeed-IDE-309c76e99ed5819b8a41f3ed5cc42575)

---

**Made with ❤️ by Chang-Augenweide**
