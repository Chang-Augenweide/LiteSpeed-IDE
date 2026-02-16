// Rust 语言插件

use crate::core::plugin::{PluginMetadata, Permission};

pub const RUST_ANALYZER_COMMAND: &str = "rust-analyzer";

/// Rust 插件元数据
pub fn metadata() -> PluginMetadata {
    PluginMetadata {
        name: "rust".to_string(),
        version: "0.1.0".to_string(),
        permissions: vec![
            Permission::ReadFiles,
            Permission::WriteFiles,
            Permission::SpawnProcess,
        ],
    }
}

// TODO: 实现 Rust LSP 集成 (rust-analyzer)
