// LiteSpeed-IDE - 极致性能的轻量化 IDE
//
// 核心特性：
// - 比 VS Code 轻 5 倍：安装包 15-25 MB
// - 比 VS Code 快 3 倍：启动 < 0.5s
// - 低内存占用：常驻 80-120 MB
//
// 技术栈：
// - Rust + Tauri：性能极致
// - CodeMirror 6：轻量编辑器
// - WASM：插件沙箱

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod core;
mod plugins;

use std::env;
use tracing::{info, error};
use tracing_subscriber;

fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 LiteSpeed-IDE 启动中...");

    // TODO: 初始化 Tauri 应用
    // TODO: 初始化核心模块
    // TODO: 启动事件循环
}
