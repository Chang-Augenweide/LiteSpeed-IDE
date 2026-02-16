// 文件系统管理器

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use lru::LruCache;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::info;
use tracing::debug;

type Content = String;

/// 文件系统管理器 - 负责文件读写、监听、缓存
#[derive(Debug)]
pub struct FSManager {
    /// 文件监听器（占位符）
    watcher: Option<notify::RecommendedWatcher>,
    /// LRU 缓存
    cache: RwLock<LruCache<PathBuf, Content>>,
    /// 文件索引
    indexer: RwLock<HashMap<PathBuf, FileMetadata>>,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// 文件大小
    pub size: u64,
    /// 最后修改时间
    pub modified: std::time::SystemTime,
    /// 是否是二进制文件
    pub is_binary: bool,
}

impl FSManager {
    /// 创建新的文件系统管理器
    pub async fn new() -> Result<Self> {
        info!("初始化文件系统管理器...");

        // 文件监听器先设为 None，等到 Phase 2 实现
        let watcher = None;

        Ok(Self {
            watcher,
            cache: RwLock::new(LruCache::new(std::num::NonZeroUsize::new(1024).unwrap())),
            indexer: RwLock::new(HashMap::new()),
        })
    }

    /// 读取文件内容（带缓存）
    pub async fn read_file(&self, path: &Path) -> Result<String> {
        // 检查缓存
        {
            let mut cache = self.cache.write().await;
            if let Some(content) = cache.get(path) {
                info!("从缓存读取: {:?}", path);
                return Ok(content.clone());
            }
        }

        // 异步读取文件
        let content = tokio::fs::read_to_string(path).await?;
        info!("从文件系统读取: {:?}", path);

        // 写入缓存
        {
            let mut cache = self.cache.write().await;
            cache.put(path.to_path_buf(), content.clone());
        }

        Ok(content)
    }

    /// 写入文件
    pub async fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        tokio::fs::write(path, content).await?;
        info!("写入文件: {:?}", path);

        // 更新缓存
        {
            let mut cache = self.cache.write().await;
            cache.put(path.to_path_buf(), content.to_string());
        }

        Ok(())
    }

    /// 检测是否是二进制文件
    pub fn is_binary_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap_or("");
            // 常见的二进制文件扩展名
            match ext {
                "exe" | "dll" | "so" | "dylib" | "bin" | "o" => true,
                "png" | "jpg" | "jpeg" | "gif" | "webp" => true,
                "pdf" | "zip" | "gz" | "7z" | "tar" => true,
                "mp3" | "mp4" | "avi" | "mov" => true,
                _ => false,
            }
        } else {
            false
        }
    }
}
