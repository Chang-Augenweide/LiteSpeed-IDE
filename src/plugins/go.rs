// Go 语言插件

use crate::core::plugin::{PluginMetadata, Permission};

pub const GOPLS_COMMAND: &str = "gopls";

/// Go 插件元数据
pub fn metadata() -> PluginMetadata {
    PluginMetadata {
        name: "go".to_string(),
        version: "0.1.0".to_string(),
        permissions: vec![
            Permission::ReadFiles,
            Permission::WriteFiles,
            Permission::SpawnProcess,
        ],
    }
}

// TODO: 实现 Go LSP 集成 (gopls)
