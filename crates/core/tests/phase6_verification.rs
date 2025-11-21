use librepods_core::ci_pipeline::*;
use librepods_core::test_runner::*;
use librepods_core::coverage_analyzer::*;

#[test]
fn test_phase6_complete_automated_verification() {
    let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());

    let build_stage = PipelineStage {
        stage_id: "build".to_string(),
        stage_name: "Build".to_string(),
        description: "Compile project".to_string(),
        commands: vec!["cargo build --all-features".to_string()],
        timeout_seconds: 300,
        status: StageStatus::Success,
        duration_ms: 15000,
        error_message: None,
    };

    let test_stage = PipelineStage {
        stage_id: "test".to_string(),
        stage_name: "Test".to_string(),
        description: "Run tests".to_string(),
        commands: vec!["cargo test --all-features".to_string()],
        timeout_seconds: 600,
        status: StageStatus::Success,
        duration_ms: 25000,
        error_message: None,
    };

    pipeline.add_stage(build_stage);
    pipeline.add_stage(test_stage);

    pipeline.update_stage_status("build", StageStatus::Success, 15000);
    pipeline.update_stage_status("test", StageStatus::Success, 25000);

    assert!(pipeline.is_successful());
    assert_eq!(pipeline.get_success_rate(), 100.0);
}

#[test]
fn test_ci_pipeline_creation() {
    let pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
    assert_eq!(pipeline.pipeline_id, "pipeline-1");
    assert_eq!(pipeline.branch, "main");
    assert_eq!(pipeline.total_stages, 0);
}

#[test]
fn test_add_pipeline_stage() {
    let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
    let stage = PipelineStage {
        stage_id: "build".to_string(),
        stage_name: "Build".to_string(),
        description: "Build stage".to_string(),
        commands: vec!["cargo build".to_string()],
        timeout_seconds: 300,
        status: StageStatus::Pending,
        duration_ms: 0,
        error_message: None,
    };
    pipeline.add_stage(stage);
    assert_eq!(pipeline.total_stages, 1);
}

#[test]
fn test_update_pipeline_stage_status() {
    let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
    let stage = PipelineStage {
        stage_id: "build".to_string(),
        stage_name: "Build".to_string(),
        description: "Build stage".to_string(),
        commands: vec!["cargo build".to_string()],
        timeout_seconds: 300,
        status: StageStatus::Pending,
        duration_ms: 0,
        error_message: None,
    };
    pipeline.add_stage(stage);
    pipeline.update_stage_status("build", StageStatus::Success, 5000);
    assert_eq!(pipeline.completed_stages, 1);
}

#[test]
fn test_pipeline_success_rate() {
    let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
    for i in 0..4 {
        let stage = PipelineStage {
            stage_id: format!("stage-{}", i),
            stage_name: format!("Stage {}", i),
            description: "Test stage".to_string(),
            commands: vec!["test".to_string()],
            timeout_seconds: 300,
            status: StageStatus::Pending,
            duration_ms: 0,
            error_message: None,
        };
        pipeline.add_stage(stage);
    }

    for i in 0..3 {
        pipeline.update_stage_status(&format!("stage-{}", i), StageStatus::Success, 1000);
    }

    assert_eq!(pipeline.get_success_rate(), 75.0);
}

#[test]
fn test_test_suite_creation() {
    let suite = TestSuite::new("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
    assert_eq!(suite.suite_id, "suite-1");
    assert_eq!(suite.total_tests, 0);
}

#[test]
fn test_add_test_case() {
    let mut suite = TestSuite::new("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
    let test = TestCase {
        test_id: "test-1".to_string(),
        test_name: "Test 1".to_string(),
        status: TestStatus::Passed,
        duration_ms: 100,
        error_message: None,
        stack_trace: None,
    };
    suite.add_test(test);
    assert_eq!(suite.total_tests, 1);
    assert_eq!(suite.passed_tests, 1);
}

#[test]
fn test_test_suite_pass_rate() {
    let mut suite = TestSuite::new("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
    for i in 0..10 {
        let test = TestCase {
            test_id: format!("test-{}", i),
            test_name: format!("Test {}", i),
            status: if i < 8 { TestStatus::Passed } else { TestStatus::Failed },
            duration_ms: 100,
            error_message: None,
            stack_trace: None,
        };
        suite.add_test(test);
    }
    assert_eq!(suite.get_pass_rate(), 80.0);
}

#[test]
fn test_test_runner() {
    let mut runner = TestRunner::new();
    runner.create_suite("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
    let test = TestCase {
        test_id: "test-1".to_string(),
        test_name: "Test 1".to_string(),
        status: TestStatus::Passed,
        duration_ms: 100,
        error_message: None,
        stack_trace: None,
    };
    runner.add_test("suite-1", test);
    assert_eq!(runner.get_total_tests(), 1);
}

#[test]
fn test_test_runner_overall_pass_rate() {
    let mut runner = TestRunner::new();
    runner.create_suite("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
    
    for i in 0..10 {
        let test = TestCase {
            test_id: format!("test-{}", i),
            test_name: format!("Test {}", i),
            status: if i < 9 { TestStatus::Passed } else { TestStatus::Failed },
            duration_ms: 100,
            error_message: None,
            stack_trace: None,
        };
        runner.add_test("suite-1", test);
    }
    
    assert_eq!(runner.get_overall_pass_rate(), 90.0);
}

#[test]
fn test_coverage_metrics() {
    let metrics = CoverageMetrics {
        file_path: "test.rs".to_string(),
        lines_total: 100,
        lines_covered: 80,
        branches_total: 50,
        branches_covered: 40,
        functions_total: 10,
        functions_covered: 9,
    };
    assert_eq!(metrics.get_line_coverage(), 80.0);
    assert_eq!(metrics.get_branch_coverage(), 80.0);
    assert_eq!(metrics.get_function_coverage(), 90.0);
}

#[test]
fn test_coverage_report() {
    let mut report = CoverageReport::new("report-1".to_string());
    let metrics = CoverageMetrics {
        file_path: "test.rs".to_string(),
        lines_total: 100,
        lines_covered: 80,
        branches_total: 50,
        branches_covered: 40,
        functions_total: 10,
        functions_covered: 9,
    };
    report.add_metrics(metrics);
    assert_eq!(report.get_line_coverage(), 80.0);
}

#[test]
fn test_coverage_analyzer() {
    let mut analyzer = CoverageAnalyzer::new();
    analyzer.create_report("report-1".to_string());
    let metrics = CoverageMetrics {
        file_path: "test.rs".to_string(),
        lines_total: 100,
        lines_covered: 85,
        branches_total: 50,
        branches_covered: 40,
        functions_total: 10,
        functions_covered: 9,
    };
    analyzer.add_metrics("report-1", metrics);
    assert!(analyzer.is_coverage_sufficient("report-1", 80.0));
}

#[test]
fn test_pipeline_report_generation() {
    let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
    let stage = PipelineStage {
        stage_id: "build".to_string(),
        stage_name: "Build".to_string(),
        description: "Build stage".to_string(),
        commands: vec!["cargo build".to_string()],
        timeout_seconds: 300,
        status: StageStatus::Success,
        duration_ms: 5000,
        error_message: None,
    };
    pipeline.add_stage(stage);
    pipeline.update_stage_status("build", StageStatus::Success, 5000);
    
    let report = pipeline.generate_report();
    assert!(report.contains("CI PIPELINE REPORT"));
}

#[test]
fn test_test_suite_report_generation() {
    let mut suite = TestSuite::new("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
    let test = TestCase {
        test_id: "test-1".to_string(),
        test_name: "Test 1".to_string(),
        status: TestStatus::Passed,
        duration_ms: 100,
        error_message: None,
        stack_trace: None,
    };
    suite.add_test(test);
    
    let report = suite.generate_report();
    assert!(report.contains("TEST SUITE REPORT"));
}

#[test]
fn test_coverage_report_generation() {
    let mut report = CoverageReport::new("report-1".to_string());
    let metrics = CoverageMetrics {
        file_path: "test.rs".to_string(),
        lines_total: 100,
        lines_covered: 80,
        branches_total: 50,
        branches_covered: 40,
        functions_total: 10,
        functions_covered: 9,
    };
    report.add_metrics(metrics);
    
    let report_text = report.generate_report();
    assert!(report_text.contains("COVERAGE REPORT"));
}

#[test]
fn test_test_runner_summary() {
    let mut runner = TestRunner::new();
    runner.create_suite("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
    
    for i in 0..5 {
        let test = TestCase {
            test_id: format!("test-{}", i),
            test_name: format!("Test {}", i),
            status: TestStatus::Passed,
            duration_ms: 100,
            error_message: None,
            stack_trace: None,
        };
        runner.add_test("suite-1", test);
    }
    
    let summary = runner.generate_summary();
    assert!(summary.contains("TEST EXECUTION SUMMARY"));
}

#[test]
fn test_complete_phase6_workflow() {
    let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
    
    let build_stage = PipelineStage {
        stage_id: "build".to_string(),
        stage_name: "Build".to_string(),
        description: "Build".to_string(),
        commands: vec!["cargo build".to_string()],
        timeout_seconds: 300,
        status: StageStatus::Pending,
        duration_ms: 0,
        error_message: None,
    };
    pipeline.add_stage(build_stage);
    pipeline.update_stage_status("build", StageStatus::Success, 10000);
    
    let mut runner = TestRunner::new();
    runner.create_suite("suite-1".to_string(), "Unit Tests".to_string(), TestSuiteType::Unit);
    
    for i in 0..20 {
        let test = TestCase {
            test_id: format!("test-{}", i),
            test_name: format!("Test {}", i),
            status: if i < 19 { TestStatus::Passed } else { TestStatus::Failed },
            duration_ms: 50,
            error_message: None,
            stack_trace: None,
        };
        runner.add_test("suite-1", test);
    }
    
    let mut analyzer = CoverageAnalyzer::new();
    analyzer.create_report("report-1".to_string());
    let metrics = CoverageMetrics {
        file_path: "src/lib.rs".to_string(),
        lines_total: 1000,
        lines_covered: 850,
        branches_total: 500,
        branches_covered: 425,
        functions_total: 100,
        functions_covered: 95,
    };
    analyzer.add_metrics("report-1", metrics);
    
    assert!(pipeline.is_successful());
    assert!(runner.get_overall_pass_rate() > 90.0);
    assert!(analyzer.is_coverage_sufficient("report-1", 80.0));
}

#[test]
fn test_pipeline_failed_stage() {
    let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
    let stage = PipelineStage {
        stage_id: "build".to_string(),
        stage_name: "Build".to_string(),
        description: "Build stage".to_string(),
        commands: vec!["cargo build".to_string()],
        timeout_seconds: 300,
        status: StageStatus::Pending,
        duration_ms: 0,
        error_message: None,
    };
    pipeline.add_stage(stage);
    pipeline.update_stage_status("build", StageStatus::Failed, 5000);
    
    assert!(!pipeline.is_successful());
    assert_eq!(pipeline.failed_stages, 1);
}

#[test]
fn test_coverage_below_threshold() {
    let mut report = CoverageReport::new("report-1".to_string());
    let metrics = CoverageMetrics {
        file_path: "test.rs".to_string(),
        lines_total: 100,
        lines_covered: 50,
        branches_total: 50,
        branches_covered: 25,
        functions_total: 10,
        functions_covered: 5,
    };
    report.add_metrics(metrics);
    
    let below_threshold = report.get_files_below_threshold(80.0);
    assert_eq!(below_threshold.len(), 1);
}
