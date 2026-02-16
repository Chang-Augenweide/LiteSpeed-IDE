// Python 语言插件

use crate::core::plugin::{PluginMetadata, Permission};

pub const PYLSP_COMMAND: &str = "python-language-server";

/// Python 插件元数据
pub fn metadata() -> PluginMetadata {
    PluginMetadata {
        name: "python".to_string(),
        version: "0.1.0".to_string(),
        permissions: vec![
            Permission::ReadFiles,
            Permission::WriteFiles,
            Permission::SpawnProcess,
        ],
    }
}

// TODO: 实现 Python LSP 集成
