use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeWayDiff {
    pub file_path: String,
    pub base_content: String,
    pub upstream_content: String,
    pub local_content: String,
    pub merge_status: MergeStatus,
    pub conflicts: Vec<ConflictRegion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeStatus {
    Clean,
    Conflict,
    BothModified,
    OnlyUpstreamModified,
    OnlyLocalModified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub base_lines: Vec<String>,
    pub upstream_lines: Vec<String>,
    pub local_lines: Vec<String>,
}

pub struct ThreeWayDiffAnalyzer;

impl ThreeWayDiffAnalyzer {
    pub fn analyze(file_path: String, base: String, upstream: String, local: String) -> ThreeWayDiff {
        let base_lines: Vec<&str> = base.lines().collect();
        let upstream_lines: Vec<&str> = upstream.lines().collect();
        let local_lines: Vec<&str> = local.lines().collect();

        let merge_status = Self::determine_merge_status(&base_lines, &upstream_lines, &local_lines);
        let conflicts = Self::detect_conflicts(&base_lines, &upstream_lines, &local_lines);

        ThreeWayDiff {
            file_path,
            base_content: base,
            upstream_content: upstream,
            local_content: local,
            merge_status,
            conflicts,
        }
    }

    fn determine_merge_status(
        base: &[&str],
        upstream: &[&str],
        local: &[&str],
    ) -> MergeStatus {
        let base_upstream_same = base == upstream;
        let base_local_same = base == local;
        let upstream_local_same = upstream == local;

        match (base_upstream_same, base_local_same, upstream_local_same) {
            (true, true, true) => MergeStatus::Clean,
            (false, false, false) => MergeStatus::Conflict,
            (false, false, true) => MergeStatus::OnlyUpstreamModified,
            (false, true, false) => MergeStatus::OnlyLocalModified,
            (true, false, false) => MergeStatus::OnlyLocalModified,
            _ => MergeStatus::Clean,
        }
    }

    fn detect_conflicts(
        base: &[&str],
        upstream: &[&str],
        local: &[&str],
    ) -> Vec<ConflictRegion> {
        let mut conflicts = Vec::new();

        let max_len = std::cmp::max(std::cmp::max(base.len(), upstream.len()), local.len());

        for i in 0..max_len {
            let base_line = base.get(i).copied().unwrap_or("");
            let upstream_line = upstream.get(i).copied().unwrap_or("");
            let local_line = local.get(i).copied().unwrap_or("");

            if base_line != upstream_line && base_line != local_line && upstream_line != local_line {
                conflicts.push(ConflictRegion {
                    start_line: i,
                    end_line: i,
                    base_lines: vec![base_line.to_string()],
                    upstream_lines: vec![upstream_line.to_string()],
                    local_lines: vec![local_line.to_string()],
                });
            }
        }

        conflicts
    }

    pub fn can_auto_merge(diff: &ThreeWayDiff) -> bool {
        matches!(
            diff.merge_status,
            MergeStatus::Clean
                | MergeStatus::OnlyUpstreamModified
                | MergeStatus::OnlyLocalModified
        )
    }

    pub fn requires_manual_merge(diff: &ThreeWayDiff) -> bool {
        matches!(diff.merge_status, MergeStatus::Conflict) || !diff.conflicts.is_empty()
    }

    pub fn generate_merge_result(diff: &ThreeWayDiff) -> String {
        match diff.merge_status {
            MergeStatus::Clean => diff.base_content.clone(),
            MergeStatus::OnlyUpstreamModified => diff.upstream_content.clone(),
            MergeStatus::OnlyLocalModified => diff.local_content.clone(),
            MergeStatus::BothModified => {
                Self::merge_both_modified(&diff.base_content, &diff.upstream_content, &diff.local_content)
            }
            MergeStatus::Conflict => {
                Self::generate_conflict_markers(&diff.base_content, &diff.upstream_content, &diff.local_content)
            }
        }
    }

    fn merge_both_modified(base: &str, upstream: &str, local: &str) -> String {
        let base_lines: Vec<&str> = base.lines().collect();
        let upstream_lines: Vec<&str> = upstream.lines().collect();
        let local_lines: Vec<&str> = local.lines().collect();

        let mut result = String::new();
        let max_len = std::cmp::max(std::cmp::max(base_lines.len(), upstream_lines.len()), local_lines.len());

        for i in 0..max_len {
            let base_line = base_lines.get(i).copied().unwrap_or("");
            let upstream_line = upstream_lines.get(i).copied().unwrap_or("");
            let local_line = local_lines.get(i).copied().unwrap_or("");

            if upstream_line != base_line && local_line == base_line {
                result.push_str(upstream_line);
            } else if local_line != base_line && upstream_line == base_line {
                result.push_str(local_line);
            } else if upstream_line == local_line {
                result.push_str(upstream_line);
            } else {
                result.push_str(local_line);
            }
            result.push('\n');
        }

        result
    }

    fn generate_conflict_markers(_base: &str, upstream: &str, local: &str) -> String {
        let mut result = String::new();
        result.push_str("<<<<<<< LOCAL\n");
        result.push_str(local);
        result.push_str("\n=======\n");
        result.push_str(upstream);
        result.push_str("\n>>>>>>> UPSTREAM\n");
        result
    }

    pub fn generate_diff_report(diffs: &[ThreeWayDiff]) -> String {
        let mut report = String::new();
        report.push_str("=== THREE-WAY DIFF REPORT ===\n\n");

        let clean = diffs.iter().filter(|d| d.merge_status == MergeStatus::Clean).count();
        let conflicts = diffs.iter().filter(|d| d.merge_status == MergeStatus::Conflict).count();
        let only_upstream = diffs
            .iter()
            .filter(|d| d.merge_status == MergeStatus::OnlyUpstreamModified)
            .count();
        let only_local = diffs
            .iter()
            .filter(|d| d.merge_status == MergeStatus::OnlyLocalModified)
            .count();

        report.push_str(&format!("Total Files: {}\n", diffs.len()));
        report.push_str(&format!("  Clean: {}\n", clean));
        report.push_str(&format!("  Conflicts: {}\n", conflicts));
        report.push_str(&format!("  Only Upstream Modified: {}\n", only_upstream));
        report.push_str(&format!("  Only Local Modified: {}\n\n", only_local));

        report.push_str("Files:\n");
        for diff in diffs {
            report.push_str(&format!("  {} ({:?})\n", diff.file_path, diff.merge_status));
            if !diff.conflicts.is_empty() {
                report.push_str(&format!("    Conflicts: {}\n", diff.conflicts.len()));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_merge() {
        let diff = ThreeWayDiffAnalyzer::analyze(
            "test.rs".to_string(),
            "base".to_string(),
            "base".to_string(),
            "base".to_string(),
        );
        assert_eq!(diff.merge_status, MergeStatus::Clean);
    }

    #[test]
    fn test_only_upstream_modified() {
        let diff = ThreeWayDiffAnalyzer::analyze(
            "test.rs".to_string(),
            "base".to_string(),
            "upstream".to_string(),
            "base".to_string(),
        );
        assert_eq!(diff.merge_status, MergeStatus::OnlyUpstreamModified);
    }

    #[test]
    fn test_only_local_modified() {
        let diff = ThreeWayDiffAnalyzer::analyze(
            "test.rs".to_string(),
            "base".to_string(),
            "base".to_string(),
            "local".to_string(),
        );
        assert_eq!(diff.merge_status, MergeStatus::OnlyLocalModified);
    }

    #[test]
    fn test_conflict_detection() {
        let diff = ThreeWayDiffAnalyzer::analyze(
            "test.rs".to_string(),
            "base".to_string(),
            "upstream".to_string(),
            "local".to_string(),
        );
        assert_eq!(diff.merge_status, MergeStatus::Conflict);
    }

    #[test]
    fn test_can_auto_merge() {
        let diff = ThreeWayDiffAnalyzer::analyze(
            "test.rs".to_string(),
            "base".to_string(),
            "upstream".to_string(),
            "base".to_string(),
        );
        assert!(ThreeWayDiffAnalyzer::can_auto_merge(&diff));
    }

    #[test]
    fn test_requires_manual_merge() {
        let diff = ThreeWayDiffAnalyzer::analyze(
            "test.rs".to_string(),
            "base".to_string(),
            "upstream".to_string(),
            "local".to_string(),
        );
        assert!(ThreeWayDiffAnalyzer::requires_manual_merge(&diff));
    }
}
