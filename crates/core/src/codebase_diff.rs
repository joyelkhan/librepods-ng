use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub file_path: String,
    pub status: FileStatus,
    pub base_content: Option<String>,
    pub upstream_content: Option<String>,
    pub local_content: Option<String>,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub lines_modified: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Unchanged,
    Conflict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub file_path: String,
    pub hunk_id: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub base_lines: Vec<String>,
    pub upstream_lines: Vec<String>,
    pub local_lines: Vec<String>,
    pub conflict: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeDecision {
    Adopt,
    Adapt,
    Skip,
    Replace,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeHunk {
    pub hunk: DiffHunk,
    pub decision: MergeDecision,
    pub reason: String,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseDiffReport {
    pub scan_date: String,
    pub base_version: String,
    pub upstream_version: String,
    pub local_version: String,
    pub file_diffs: Vec<FileDiff>,
    pub hunks: Vec<DiffHunk>,
    pub total_files: usize,
    pub files_added: usize,
    pub files_modified: usize,
    pub files_deleted: usize,
    pub files_conflicted: usize,
    pub total_hunks: usize,
    pub conflicted_hunks: usize,
}

impl CodebaseDiffReport {
    pub fn new(base_version: String, upstream_version: String, local_version: String) -> Self {
        Self {
            scan_date: "2025-11-21T23:55:00Z".to_string(),
            base_version,
            upstream_version,
            local_version,
            file_diffs: Vec::new(),
            hunks: Vec::new(),
            total_files: 0,
            files_added: 0,
            files_modified: 0,
            files_deleted: 0,
            files_conflicted: 0,
            total_hunks: 0,
            conflicted_hunks: 0,
        }
    }

    pub fn add_file_diff(&mut self, diff: FileDiff) {
        match diff.status {
            FileStatus::Added => self.files_added += 1,
            FileStatus::Modified => self.files_modified += 1,
            FileStatus::Deleted => self.files_deleted += 1,
            FileStatus::Conflict => self.files_conflicted += 1,
            _ => {}
        }
        self.total_files += 1;
        self.file_diffs.push(diff);
    }

    pub fn add_hunk(&mut self, hunk: DiffHunk) {
        if hunk.conflict {
            self.conflicted_hunks += 1;
        }
        self.total_hunks += 1;
        self.hunks.push(hunk);
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== CODEBASE DIFF REPORT ===\n\n");

        report.push_str(&format!("Base Version: {}\n", self.base_version));
        report.push_str(&format!("Upstream Version: {}\n", self.upstream_version));
        report.push_str(&format!("Local Version: {}\n", self.local_version));
        report.push_str(&format!("Scan Date: {}\n\n", self.scan_date));

        report.push_str(&format!("Total Files: {}\n", self.total_files));
        report.push_str(&format!("  Added: {}\n", self.files_added));
        report.push_str(&format!("  Modified: {}\n", self.files_modified));
        report.push_str(&format!("  Deleted: {}\n", self.files_deleted));
        report.push_str(&format!("  Conflicted: {}\n\n", self.files_conflicted));

        report.push_str(&format!("Total Hunks: {}\n", self.total_hunks));
        report.push_str(&format!("  Conflicted: {}\n\n", self.conflicted_hunks));

        report.push_str("File Changes:\n");
        for diff in &self.file_diffs {
            report.push_str(&format!("  {} ({:?})\n", diff.file_path, diff.status));
            report.push_str(&format!("    +{} -{} ~{}\n", diff.lines_added, diff.lines_removed, diff.lines_modified));
        }

        report
    }
}

impl Default for CodebaseDiffReport {
    fn default() -> Self {
        Self::new("1.0.0".to_string(), "1.0.0".to_string(), "1.0.0".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_creation() {
        let report = CodebaseDiffReport::new(
            "1.0.0".to_string(),
            "1.0.1".to_string(),
            "1.0.0".to_string(),
        );
        assert_eq!(report.base_version, "1.0.0");
        assert_eq!(report.upstream_version, "1.0.1");
    }

    #[test]
    fn test_add_file_diff() {
        let mut report = CodebaseDiffReport::new(
            "1.0.0".to_string(),
            "1.0.1".to_string(),
            "1.0.0".to_string(),
        );

        let diff = FileDiff {
            file_path: "src/test.rs".to_string(),
            status: FileStatus::Added,
            base_content: None,
            upstream_content: Some("code".to_string()),
            local_content: None,
            lines_added: 10,
            lines_removed: 0,
            lines_modified: 0,
        };

        report.add_file_diff(diff);
        assert_eq!(report.total_files, 1);
        assert_eq!(report.files_added, 1);
    }

    #[test]
    fn test_add_hunk() {
        let mut report = CodebaseDiffReport::new(
            "1.0.0".to_string(),
            "1.0.1".to_string(),
            "1.0.0".to_string(),
        );

        let hunk = DiffHunk {
            file_path: "src/test.rs".to_string(),
            hunk_id: 1,
            start_line: 1,
            end_line: 10,
            base_lines: vec!["line1".to_string()],
            upstream_lines: vec!["line2".to_string()],
            local_lines: vec!["line1".to_string()],
            conflict: false,
        };

        report.add_hunk(hunk);
        assert_eq!(report.total_hunks, 1);
    }

    #[test]
    fn test_conflict_tracking() {
        let mut report = CodebaseDiffReport::new(
            "1.0.0".to_string(),
            "1.0.1".to_string(),
            "1.0.0".to_string(),
        );

        let hunk = DiffHunk {
            file_path: "src/test.rs".to_string(),
            hunk_id: 1,
            start_line: 1,
            end_line: 10,
            base_lines: vec!["line1".to_string()],
            upstream_lines: vec!["line2".to_string()],
            local_lines: vec!["line3".to_string()],
            conflict: true,
        };

        report.add_hunk(hunk);
        assert_eq!(report.conflicted_hunks, 1);
    }
}
