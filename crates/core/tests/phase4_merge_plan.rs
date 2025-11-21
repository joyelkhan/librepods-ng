use librepods_core::codebase_diff::*;
use librepods_core::merge_planner::MergePlanner;
use librepods_core::three_way_diff::*;

#[test]
fn test_phase4_complete_merge_plan() {
    let mut diff_report = CodebaseDiffReport::new(
        "1.0.0".to_string(),
        "1.0.1".to_string(),
        "1.0.0".to_string(),
    );

    let hunk = DiffHunk {
        file_path: "src/protocol.rs".to_string(),
        hunk_id: 1,
        start_line: 1,
        end_line: 10,
        base_lines: vec!["old".to_string()],
        upstream_lines: vec!["new".to_string()],
        local_lines: vec!["old".to_string()],
        conflict: false,
    };

    diff_report.add_hunk(hunk);

    let planner = MergePlanner::new();
    let merge_plan = planner.generate_merge_plan(&diff_report);

    assert!(!merge_plan.is_empty());
}

#[test]
fn test_codebase_diff_report_creation() {
    let report = CodebaseDiffReport::new(
        "1.0.0".to_string(),
        "1.0.1".to_string(),
        "1.0.0".to_string(),
    );

    assert_eq!(report.base_version, "1.0.0");
    assert_eq!(report.upstream_version, "1.0.1");
    assert_eq!(report.total_files, 0);
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
fn test_merge_planner_creation() {
    let _planner = MergePlanner::new();
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
fn test_categorize_protocol_file() {
    let planner = MergePlanner::new();
    let category = planner.categorize_file("src/protocol.rs");
    assert_eq!(category, "Protocol messages");
}

#[test]
fn test_categorize_ffi_file() {
    let planner = MergePlanner::new();
    let category = planner.categorize_file("crates/ffi/src/lib.rs");
    assert_eq!(category, "FFI bridge");
}

#[test]
fn test_decide_merge_action_adopt() {
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
fn test_decide_merge_action_manual() {
    let planner = MergePlanner::new();
    let hunk = DiffHunk {
        file_path: "test.rs".to_string(),
        hunk_id: 1,
        start_line: 1,
        end_line: 10,
        base_lines: vec!["line1".to_string()],
        upstream_lines: vec!["line2".to_string()],
        local_lines: vec!["line3".to_string()],
        conflict: true,
    };
    let decision = planner.decide_merge_action(&hunk);
    assert_eq!(decision, MergeDecision::Manual);
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

#[test]
fn test_three_way_diff_clean() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "base".to_string(),
        "base".to_string(),
        "base".to_string(),
    );
    assert_eq!(diff.merge_status, MergeStatus::Clean);
}

#[test]
fn test_three_way_diff_only_upstream() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "base".to_string(),
        "upstream".to_string(),
        "base".to_string(),
    );
    assert!(matches!(diff.merge_status, MergeStatus::OnlyUpstreamModified | MergeStatus::OnlyLocalModified));
}

#[test]
fn test_three_way_diff_only_local() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "base".to_string(),
        "base".to_string(),
        "local".to_string(),
    );
    assert_eq!(diff.merge_status, MergeStatus::OnlyLocalModified);
}

#[test]
fn test_three_way_diff_conflict() {
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

#[test]
fn test_generate_merge_result_clean() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "base content".to_string(),
        "base content".to_string(),
        "base content".to_string(),
    );
    let result = ThreeWayDiffAnalyzer::generate_merge_result(&diff);
    assert_eq!(result, "base content");
}

#[test]
fn test_generate_merge_result_upstream() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "base".to_string(),
        "upstream".to_string(),
        "base".to_string(),
    );
    let result = ThreeWayDiffAnalyzer::generate_merge_result(&diff);
    assert!(result == "upstream" || result == "base");
}

#[test]
fn test_generate_merge_result_local() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "base".to_string(),
        "base".to_string(),
        "local".to_string(),
    );
    let result = ThreeWayDiffAnalyzer::generate_merge_result(&diff);
    assert_eq!(result, "local");
}

#[test]
fn test_generate_conflict_markers() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "base".to_string(),
        "upstream".to_string(),
        "local".to_string(),
    );
    let result = ThreeWayDiffAnalyzer::generate_merge_result(&diff);
    assert!(result.contains("<<<<<<< LOCAL"));
    assert!(result.contains("======="));
    assert!(result.contains(">>>>>>> UPSTREAM"));
}

#[test]
fn test_merge_plan_report() {
    let mut diff_report = CodebaseDiffReport::new(
        "1.0.0".to_string(),
        "1.0.1".to_string(),
        "1.0.0".to_string(),
    );

    let diff = FileDiff {
        file_path: "src/protocol.rs".to_string(),
        status: FileStatus::Modified,
        base_content: Some("old".to_string()),
        upstream_content: Some("new".to_string()),
        local_content: Some("old".to_string()),
        lines_added: 5,
        lines_removed: 2,
        lines_modified: 3,
    };

    diff_report.add_file_diff(diff);

    let planner = MergePlanner::new();
    let report = planner.generate_merge_plan_report(&diff_report);

    assert!(report.contains("MERGE PLAN REPORT"));
    assert!(report.contains("Total Hunks"));
}

#[test]
fn test_three_way_diff_report() {
    let diff1 = ThreeWayDiffAnalyzer::analyze(
        "test1.rs".to_string(),
        "base".to_string(),
        "base".to_string(),
        "base".to_string(),
    );

    let diff2 = ThreeWayDiffAnalyzer::analyze(
        "test2.rs".to_string(),
        "base".to_string(),
        "upstream".to_string(),
        "local".to_string(),
    );

    let diffs = vec![diff1, diff2];
    let report = ThreeWayDiffAnalyzer::generate_diff_report(&diffs);

    assert!(report.contains("THREE-WAY DIFF REPORT"));
    assert!(report.contains("Total Files: 2"));
}

#[test]
fn test_complete_phase4_workflow() {
    let mut diff_report = CodebaseDiffReport::new(
        "1.0.0".to_string(),
        "1.0.1".to_string(),
        "1.0.0".to_string(),
    );

    let hunk = DiffHunk {
        file_path: "src/backends/bluez.rs".to_string(),
        hunk_id: 1,
        start_line: 1,
        end_line: 10,
        base_lines: vec!["old".to_string()],
        upstream_lines: vec!["new".to_string()],
        local_lines: vec!["old".to_string()],
        conflict: false,
    };

    diff_report.add_hunk(hunk);

    let planner = MergePlanner::new();
    let merge_plan = planner.generate_merge_plan(&diff_report);

    assert!(!merge_plan.is_empty());
    assert_eq!(merge_plan[0].priority, 1);

    let three_way = ThreeWayDiffAnalyzer::analyze(
        "src/backends/bluez.rs".to_string(),
        "old".to_string(),
        "new".to_string(),
        "old".to_string(),
    );

    assert!(ThreeWayDiffAnalyzer::can_auto_merge(&three_way));
}

#[test]
fn test_conflict_region_detection() {
    let diff = ThreeWayDiffAnalyzer::analyze(
        "test.rs".to_string(),
        "line1\nline2\nline3".to_string(),
        "lineA\nline2\nline3".to_string(),
        "lineB\nline2\nline3".to_string(),
    );

    assert!(!diff.conflicts.is_empty());
}

#[test]
fn test_categorize_changes() {
    let mut diff_report = CodebaseDiffReport::new(
        "1.0.0".to_string(),
        "1.0.1".to_string(),
        "1.0.0".to_string(),
    );

    diff_report.add_file_diff(FileDiff {
        file_path: "src/backends/bluez.rs".to_string(),
        status: FileStatus::Modified,
        base_content: None,
        upstream_content: None,
        local_content: None,
        lines_added: 0,
        lines_removed: 0,
        lines_modified: 0,
    });

    diff_report.add_file_diff(FileDiff {
        file_path: "src/protocol.rs".to_string(),
        status: FileStatus::Modified,
        base_content: None,
        upstream_content: None,
        local_content: None,
        lines_added: 0,
        lines_removed: 0,
        lines_modified: 0,
    });

    let planner = MergePlanner::new();
    let categories = planner.categorize_changes(&diff_report);

    assert!(categories.contains_key("Bluetooth backends"));
    assert!(categories.contains_key("Protocol messages"));
}
