// 命令行示例代码 - 演示核心功能

use litespeed_ide::core::{FSManager, PluginManager, LSPClient, GitManager};
use std::path::PathBuf;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 LiteSpeed-IDE CLI Demo\n");

    // 1. 文件系统管理器演示
    println!("📁 文件系统管理器演示:");
    let fs_manager = FSManager::new().await?;

    let test_file = PathBuf::from("/tmp/litespeed_test.txt");
    fs_manager.write_file(&test_file, "Hello from LiteSpeed-IDE!").await?;
    println!("   ✅ 写入文件: {:?}", test_file);

    let content = fs_manager.read_file(&test_file).await?;
    println!("   ✅ 读取内容: {}", content);

    // 检测二进制文件
    println!("   ✅ test.txt 是否为二进制文件: {}", FSManager::is_binary_file(&test_file));

    // 2. 插件管理器演示
    println!("\n🔌 插件管理器演示:");
    let mut plugin_manager = PluginManager::new().await?;
    println!("   ✅ 插件管理器初始化成功");

    // 列出内置插件的元数据
    println!("\n   📦 内置语言插件:");
    println!("      • Python: {}", crate::plugins::python::PYLSP_COMMAND);
    println!("      • Go: {}", crate::plugins::go::GOPLS_COMMAND);
    println!("      • Rust: {}", crate::plugins::rust::RUST_ANALYZER_COMMAND);

    // 3. LSP 客户端演示
    println!("\n🔍 LSP 客户端演示:");
    let lsp_client = LSPClient::new(PathBuf::from(".")).await?;
    println!("   ✅ LSP 客户端初始化成功");
    println!("   ℹ️  LSP 服务端启动命令: cargo run --bin lsp-server");

    // 4. Git 管理器演示
    println!("\n🔧 Git 管理器演示:");
    let git_manager = GitManager::new(PathBuf::from("."))?;
    if git_manager.get_status().is_ok() {
        println!("   ✅ Git 仓库检测成功");
    } else {
        println!("   ℹ️  当前目录不是 Git 仓库");
    }

    println!("\n✨ 演示完成！");
    Ok(())
}
