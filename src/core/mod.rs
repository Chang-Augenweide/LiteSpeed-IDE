// 核心模块 - 文件系统、插件、LSP、Git 等

pub mod fs;
pub mod plugin;
pub mod lsp;
pub mod git;

pub use fs::FSManager;
pub use plugin::PluginManager;
pub use lsp::LSPClient;
pub use git::GitManager;
