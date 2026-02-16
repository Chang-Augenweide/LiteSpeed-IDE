// 搜索功能 - 基于索引的快速搜索

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 文件路径
    pub file_path: PathBuf,
    /// 行号
    pub line_number: usize,
    /// 行内容
    pub line_content: String,
}

/// 搜索管理器 - 负责文本搜索
pub struct SearchManager {
    /// 搜索索引
    index: HashMap<String, Vec<PathBuf>>,
}

impl SearchManager {
    /// 创建新的搜索管理器
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// 在文件中搜索文本（普通文本）
    pub fn search_files<P: Into<PathBuf>>(
        &mut self,
        pattern: &str,
        path: P,
        is_regex: bool,
        case_sensitive: bool,
    ) -> Result<Vec<SearchResult>> {
        let path = path.into();
        let mut results = Vec::new();

        if is_regex {
            // 如果是正则，使用 grep crate（实现待完成）
            // TODO: 实现正则搜索
            Ok(results)
        } else {
            // 普通文本搜索（简单实现）
            self.search_text(pattern, &path, case_sensitive, &mut results)?;
            Ok(results)
        }
    }

    /// 普通文本搜索
    fn search_text(
        &self,
        pattern: &str,
        path: &Path,
        case_sensitive: bool,
        results: &mut Vec<SearchResult>,
    ) -> Result<()> {
        let search_pattern = if case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };

        if path.is_file() {
            // 搜索单个文件
            self.search_text_in_file(path, &search_pattern, case_sensitive, results)?;
        } else if path.is_dir() {
            // 搜索目录中的所有文件
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                self.search_text_in_file(entry.path(), &search_pattern, case_sensitive, results)?;
            }
        }

        Ok(())
    }

    /// 在单个文件中搜索
    fn search_text_in_file(
        &self,
        file_path: &Path,
        pattern: &str,
        case_sensitive: bool,
        results: &mut Vec<SearchResult>,
    ) -> Result<()> {
        // 跳过二进制文件
        if super::fs::FSManager::is_binary_file(file_path) {
            return Ok(());
        }

        // 读取文件
        let content = std::fs::read_to_string(file_path)?;

        // 遍历每一行
        for (line_num, line) in content.lines().enumerate() {
            let line_to_search = if case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };

            if line_to_search.contains(pattern) {
                results.push(SearchResult {
                    file_path: file_path.to_path_buf(),
                    line_number: line_num + 1,
                    line_content: line.to_string(),
                });
            }
        }

        Ok(())
    }

    /// 在当前工作区中搜索
    pub fn search_workspace(
        &mut self,
        pattern: &str,
    ) -> Result<Vec<SearchResult>> {
        let workspace = PathBuf::from(".");
        self.search_files(pattern, workspace, false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_search_files() {
        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "Hello World\nThis is a test\n").unwrap();

        let mut search_manager = SearchManager::new();
        let results = search_manager.search_files("test", &test_file, false, false).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].line_content.contains("test"));
    }

    #[test]
    fn test_search_case_insensitive() {
        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "Hello World\nThis is a TEST\nthis is test\n").unwrap();

        let mut search_manager = SearchManager::new();
        let results = search_manager.search_files("test", &test_file, false, false).unwrap();

        assert_eq!(results.len(), 2);  // "TEST" 和 "test"
    }

    #[test]
    fn test_search_case_sensitive() {
        let dir = tempdir().unwrap();
        let test_file = dir.path().join("test.txt");
        std::fs::write(&test_file, "Hello World\nThis is a TEST\nthis is test\n").unwrap();

        let mut search_manager = SearchManager::new();
        let results = search_manager.search_files("test", &test_file, false, true).unwrap();

        assert_eq!(results.len(), 1);  // 只有 "test"
    }
}
