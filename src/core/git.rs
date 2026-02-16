// Git 管理器 - 基础 Git 操作

use std::path::{Path, PathBuf};
use git2::{Repository, StatusOptions};
use anyhow::Result;
use tracing::{info, error};
use serde::{Serialize, Deserialize};

/// Git 管理器 - 负责与 Git 仓库交互
pub struct GitManager {
    /// Git 仓库
    repo: Option<Repository>,
    /// 仓库路径
    repo_path: PathBuf,
    /// 状态缓存
    status_cache: Option<GitStatus>,
}

/// Git 状态信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    /// 当前分支
    pub branch: String,
    /// 被修改的文件
    pub modified: Vec<String>,
    /// 未跟踪的文件
    pub untracked: Vec<String>,
    /// 已暂存的文件
    pub staged: Vec<String>,
    /// 领先提交数
    pub ahead: usize,
    /// 落后提交数
    pub behind: usize,
    pub clean: bool,
}

impl GitManager {
    /// 创建新的 Git 管理器
    pub fn new<P: Into<PathBuf>>(repo_path: P) -> Result<Self> {
        let repo_path = repo_path.into();
        info!("初始化 Git 管理器，路径: {:?}", repo_path);

        // 尝试打开仓库
        let repo = Repository::open(&repo_path).ok();

        Ok(Self {
            repo,
            repo_path,
            status_cache: None,
        })
    }

    /// 获取 Git 状态
    pub fn get_status(&mut self) -> Result<GitStatus> {
        info!("获取 Git 状态...");

        if let Some(ref repo) = self.repo {
            let head = repo.head()?;
            let branch = head.shorthand().unwrap_or("HEAD").to_string();

            // 获取文件状态
            let mut status_opts = StatusOptions::new();
            status_opts.include_untracked(true);

            let mut modified = Vec::new();
            let mut untracked = Vec::new();
            let mut staged = Vec::new();

            repo.statuses(Some(&mut status_opts))?.iter()
                .for_each(|entry| {
                    let status = entry.status();
                    if status.is_wt_modified() {
                        modified.push(entry.path().unwrap_or("unknown").to_string());
                    } else if status.is_wt_new() {
                        untracked.push(entry.path().unwrap_or("unknown").to_string());
                    }
                    // staged 文件
                    if status.is_index_new() || status.is_index_modified() || status.is_index_renamed() {
                        staged.push(entry.path().unwrap_or("unknown").to_string());
                    }
                });

            let is_clean = modified.is_empty() && untracked.is_empty();

            let status = GitStatus {
                branch,
                modified,
                untracked,
                staged,
                ahead: 0,
                behind: 0,
                clean: is_clean,
            };

            self.status_cache = Some(status.clone());
            Ok(status)
        } else {
            error!("仓库未初始化");
            Err(anyhow::anyhow!("仓库未初始化"))
        }
    }

    /// 添加文件到暂存区
    pub fn add(&self, paths: &[&str]) -> Result<()> {
        info!("添加文件到暂存区: {:?}", paths);

        if let Some(ref repo) = self.repo {
            let mut index = repo.index()?;
            for path in paths {
                index.add_path(Path::new(path))?;
            }
            index.write()?;
            info!("文件已添加到暂存区");
            Ok(())
        } else {
            Err(anyhow::anyhow!("仓库未初始化"))
        }
    }

    /// 提交更改
    pub fn commit(&self, message: &str) -> Result<String> {
        info!("提交更改: {}", message);

        if let Some(ref repo) = self.repo {
            // TODO: 实现提交逻辑
            // 需要获取 head Commit
            // 创建 tree
            // 创建 Commit

            error!("提交功能尚未实现");
            Ok(String::new())
        } else {
            Err(anyhow::anyhow!("仓库未初始化"))
        }
    }

    /// 推送到远程
    pub fn push(&self) -> Result<()> {
        info!("推送到远程");

        if let Some(ref repo) = self.repo {
            // TODO: 实现推送逻辑
            error!("推送功能尚未实现");
            Ok(())
        } else {
            Err(anyhow::anyhow!("仓库未初始化"))
        }
    }
}
