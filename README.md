# LiteSpeed-IDE
## 极致性能的轻量化 IDE

> 为高级开发者打造，比 VS Code 轻 5 倍、快 3 倍

### 技术栈
- **Rust** (性能核心)
- **Tauri** (跨平台框架)
- **CodeMirror 6** (轻量编辑器)
- **TypeScript** (UI 层)
- **WASM** (插件沙箱)

### 核心指标
| 指标 | 目标 | VS Code |
|------|------|----------|
| 安装包 | 15-25 MB | ~100 MB |
| 内存 | 80-120 MB | ~500 MB |
| 启动 | < 0.5s | ~2s |

> 目前处于设计阶段，详见 [设计文档](docs/plans/2026-02-16-litespeed-ide-design.md)
