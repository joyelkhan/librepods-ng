use crate::codebase_diff::{CodebaseDiffReport, DiffHunk, FileStatus, FileDiff, MergeDecision, MergeHunk};
use std::collections::HashMap;

pub struct MergePlanner {
    priority_map: HashMap<String, u8>,
    decision_rules: HashMap<FileStatus, MergeDecision>,
}

impl MergePlanner {
    pub fn new() -> Self {
        let mut planner = Self {
            priority_map: HashMap::new(),
            decision_rules: HashMap::new(),
        };
        planner.initialize_priorities();
        planner.initialize_rules();
        planner
    }

    fn initialize_priorities(&mut self) {
        self.priority_map.insert("Bluetooth backends".to_string(), 1);
        self.priority_map.insert("Protocol messages".to_string(), 2);
        self.priority_map.insert("FFI bridge".to_string(), 3);
        self.priority_map.insert("Android merge".to_string(), 4);
        self.priority_map.insert("Build/packaging".to_string(), 5);
        self.priority_map.insert("CI/CD".to_string(), 6);
        self.priority_map.insert("i18n/a11y".to_string(), 7);
        self.priority_map.insert("Security".to_string(), 8);
        self.priority_map.insert("Documentation".to_string(), 9);
        self.priority_map.insert("Release chores".to_string(), 10);
    }

    fn initialize_rules(&mut self) {
        self.decision_rules.insert(FileStatus::Added, MergeDecision::Adopt);
        self.decision_rules.insert(FileStatus::Modified, MergeDecision::Adapt);
        self.decision_rules.insert(FileStatus::Deleted, MergeDecision::Skip);
        self.decision_rules.insert(FileStatus::Renamed, MergeDecision::Adapt);
        self.decision_rules.insert(FileStatus::Conflict, MergeDecision::Manual);
    }

    pub fn generate_merge_plan(&self, diff_report: &CodebaseDiffReport) -> Vec<MergeHunk> {
        let mut merge_hunks = Vec::new();

        for hunk in &diff_report.hunks {
            let decision = self.decide_merge_action(hunk);
            let priority = self.calculate_priority(&hunk.file_path);
            let reason = self.generate_reason(&hunk.file_path, decision);

            merge_hunks.push(MergeHunk {
                hunk: hunk.clone(),
                decision,
                reason,
                priority,
            });
        }

        merge_hunks.sort_by_key(|m| m.priority);
        merge_hunks
    }

    pub fn decide_merge_action(&self, hunk: &DiffHunk) -> MergeDecision {
        if hunk.conflict {
            MergeDecision::Manual
        } else if hunk.upstream_lines.is_empty() {
            MergeDecision::Skip
        } else if hunk.base_lines == hunk.local_lines {
            MergeDecision::Adopt
        } else if hunk.base_lines != hunk.upstream_lines && hunk.base_lines != hunk.local_lines {
            MergeDecision::Adapt
        } else {
            MergeDecision::Replace
        }
    }

    pub fn calculate_priority(&self, file_path: &str) -> u8 {
        for (category, priority) in &self.priority_map {
            if file_path.contains(category) {
                return *priority;
            }
        }

        if file_path.contains("backends") {
            1
        } else if file_path.contains("protocol") {
            2
        } else if file_path.contains("ffi") {
            3
        } else if file_path.contains("android") {
            4
        } else if file_path.contains("build") || file_path.contains("Cargo") {
            5
        } else if file_path.contains("github") || file_path.contains("ci") {
            6
        } else if file_path.contains("i18n") || file_path.contains("a11y") {
            7
        } else if file_path.contains("security") || file_path.contains("crypto") {
            8
        } else if file_path.contains("docs") || file_path.contains("README") {
            9
        } else {
            10
        }
    }

    fn generate_reason(&self, file_path: &str, decision: MergeDecision) -> String {
        match decision {
            MergeDecision::Adopt => format!("Adopt upstream changes for {}", file_path),
            MergeDecision::Adapt => format!("Adapt upstream changes for {}", file_path),
            MergeDecision::Skip => format!("Skip changes for {}", file_path),
            MergeDecision::Replace => format!("Replace with upstream version for {}", file_path),
            MergeDecision::Manual => format!("Manual review required for {}", file_path),
        }
    }

    pub fn categorize_changes(&self, diff_report: &CodebaseDiffReport) -> HashMap<String, Vec<FileDiff>> {
        let mut categories = HashMap::new();

        for diff in &diff_report.file_diffs {
            let category = self.categorize_file(&diff.file_path);
            categories.entry(category).or_insert_with(Vec::new).push(diff.clone());
        }

        categories
    }

    pub fn categorize_file(&self, file_path: &str) -> String {
        if file_path.contains("backends") {
            "Bluetooth backends".to_string()
        } else if file_path.contains("protocol") {
            "Protocol messages".to_string()
        } else if file_path.contains("ffi") {
            "FFI bridge".to_string()
        } else if file_path.contains("android") {
            "Android merge".to_string()
        } else if file_path.contains("build") || file_path.contains("Cargo") {
            "Build/packaging".to_string()
        } else if file_path.contains("github") || file_path.contains("ci") {
            "CI/CD".to_string()
        } else if file_path.contains("i18n") || file_path.contains("a11y") {
            "i18n/a11y".to_string()
        } else if file_path.contains("security") || file_path.contains("crypto") {
            "Security".to_string()
        } else if file_path.contains("docs") || file_path.contains("README") {
            "Documentation".to_string()
        } else {
            "Release chores".to_string()
        }
    }

    pub fn estimate_effort(&self, merge_hunks: &[MergeHunk]) -> (usize, usize, usize) {
        let mut adopt_count = 0;
        let mut adapt_count = 0;
        let mut manual_count = 0;

        for hunk in merge_hunks {
            match hunk.decision {
                MergeDecision::Adopt => adopt_count += 1,
                MergeDecision::Adapt => adapt_count += 1,
                MergeDecision::Manual => manual_count += 1,
                _ => {}
            }
        }

        (adopt_count, adapt_count, manual_count)
    }

    pub fn generate_merge_plan_report(&self, diff_report: &CodebaseDiffReport) -> String {
        let mut report = String::new();
        report.push_str("=== MERGE PLAN REPORT ===\n\n");

        let merge_hunks = self.generate_merge_plan(diff_report);
        let (adopt, adapt, manual) = self.estimate_effort(&merge_hunks);

        report.push_str(&format!("Total Hunks: {}\n", merge_hunks.len()));
        report.push_str(&format!("  Adopt: {}\n", adopt));
        report.push_str(&format!("  Adapt: {}\n", adapt));
        report.push_str(&format!("  Manual: {}\n\n", manual));

        let categories = self.categorize_changes(diff_report);
        report.push_str("Categories:\n");
        for (category, files) in &categories {
            report.push_str(&format!("  {} ({})\n", category, files.len()));
        }

        report.push_str("\nMerge Strategy:\n");
        for hunk in &merge_hunks {
            report.push_str(&format!("  [{}] {} ({:?})\n", hunk.priority, hunk.hunk.file_path, hunk.decision));
        }

        report
    }
}

impl Default for MergePlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planner_creation() {
        let planner = MergePlanner::new();
        assert!(!planner.priority_map.is_empty());
    }

    #[test]
    fn test_calculate_priority() {
        let planner = MergePlanner::new();
        let priority = planner.calculate_priority("src/backends/bluez.rs");
        assert_eq!(priority, 1);
    }

    #[test]
    fn test_categorize_file() {
        let planner = MergePlanner::new();
        let category = planner.categorize_file("src/backends/bluez.rs");
        assert_eq!(category, "Bluetooth backends");
    }

    #[test]
    fn test_decide_merge_action() {
        let planner = MergePlanner::new();
        let hunk = DiffHunk {
            file_path: "test.rs".to_string(),
            hunk_id: 1,
            start_line: 1,
            end_line: 10,
            base_lines: vec!["line1".to_string()],
            upstream_lines: vec!["line2".to_string()],
            local_lines: vec!["line1".to_string()],
            conflict: false,
        };
        let decision = planner.decide_merge_action(&hunk);
        assert_eq!(decision, MergeDecision::Adopt);
    }

    #[test]
    fn test_estimate_effort() {
        let planner = MergePlanner::new();
        let hunks = vec![
            MergeHunk {
                hunk: DiffHunk {
                    file_path: "test1.rs".to_string(),
                    hunk_id: 1,
                    start_line: 1,
                    end_line: 10,
                    base_lines: vec![],
                    upstream_lines: vec![],
                    local_lines: vec![],
                    conflict: false,
                },
                decision: MergeDecision::Adopt,
                reason: "test".to_string(),
                priority: 1,
            },
            MergeHunk {
                hunk: DiffHunk {
                    file_path: "test2.rs".to_string(),
                    hunk_id: 2,
                    start_line: 1,
                    end_line: 10,
                    base_lines: vec![],
                    upstream_lines: vec![],
                    local_lines: vec![],
                    conflict: false,
                },
                decision: MergeDecision::Adapt,
                reason: "test".to_string(),
                priority: 2,
            },
        ];
        let (adopt, adapt, manual) = planner.estimate_effort(&hunks);
        assert_eq!(adopt, 1);
        assert_eq!(adapt, 1);
        assert_eq!(manual, 0);
    }
}
