use librepods_core::implementation_executor::*;
use librepods_core::change_applier::*;
use librepods_core::verification::*;

#[test]
fn test_phase5_complete_implementation_sprint() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Implementation Sprint 1".to_string());

    let task = ImplementationTask {
        task_id: "task-1".to_string(),
        title: "Implement Bluetooth Backend".to_string(),
        description: "Add BlueZ backend support".to_string(),
        category: TaskCategory::BluetoothBackends,
        priority: 1,
        status: TaskStatus::Pending,
        effort_hours: 8.0,
        assigned_to: "dev-team".to_string(),
        dependencies: Vec::new(),
        completion_percentage: 0,
    };

    sprint.add_task(task);
    sprint.update_task_status("task-1", TaskStatus::Completed);

    assert_eq!(sprint.completed_tasks, 1);
    assert_eq!(sprint.get_completion_percentage(), 100);
}

#[test]
fn test_sprint_plan_creation() {
    let sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    assert_eq!(sprint.sprint_id, "sprint-1");
    assert_eq!(sprint.total_tasks, 0);
}

#[test]
fn test_add_task_to_sprint() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    let task = ImplementationTask {
        task_id: "task-1".to_string(),
        title: "Test Task".to_string(),
        description: "A test task".to_string(),
        category: TaskCategory::BluetoothBackends,
        priority: 1,
        status: TaskStatus::Pending,
        effort_hours: 4.0,
        assigned_to: "dev".to_string(),
        dependencies: Vec::new(),
        completion_percentage: 0,
    };
    sprint.add_task(task);
    assert_eq!(sprint.total_tasks, 1);
    assert_eq!(sprint.total_effort_hours, 4.0);
}

#[test]
fn test_update_task_status() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    let task = ImplementationTask {
        task_id: "task-1".to_string(),
        title: "Test Task".to_string(),
        description: "A test task".to_string(),
        category: TaskCategory::BluetoothBackends,
        priority: 1,
        status: TaskStatus::Pending,
        effort_hours: 4.0,
        assigned_to: "dev".to_string(),
        dependencies: Vec::new(),
        completion_percentage: 0,
    };
    sprint.add_task(task);
    sprint.update_task_status("task-1", TaskStatus::Completed);
    assert_eq!(sprint.completed_tasks, 1);
}

#[test]
fn test_update_task_progress() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    let task = ImplementationTask {
        task_id: "task-1".to_string(),
        title: "Test Task".to_string(),
        description: "A test task".to_string(),
        category: TaskCategory::BluetoothBackends,
        priority: 1,
        status: TaskStatus::InProgress,
        effort_hours: 4.0,
        assigned_to: "dev".to_string(),
        dependencies: Vec::new(),
        completion_percentage: 0,
    };
    sprint.add_task(task);
    sprint.update_task_progress("task-1", 50);
    
    let task = sprint.tasks.iter().find(|t| t.task_id == "task-1").unwrap();
    assert_eq!(task.completion_percentage, 50);
}

#[test]
fn test_get_tasks_by_status() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    for i in 0..3 {
        let task = ImplementationTask {
            task_id: format!("task-{}", i),
            title: format!("Task {}", i),
            description: "Test".to_string(),
            category: TaskCategory::BluetoothBackends,
            priority: 1,
            status: if i == 0 { TaskStatus::Completed } else { TaskStatus::Pending },
            effort_hours: 1.0,
            assigned_to: "dev".to_string(),
            dependencies: Vec::new(),
            completion_percentage: if i == 0 { 100 } else { 0 },
        };
        sprint.add_task(task);
    }
    
    let completed = sprint.get_tasks_by_status(TaskStatus::Completed);
    assert_eq!(completed.len(), 1);
}

#[test]
fn test_get_tasks_by_category() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    for i in 0..2 {
        let task = ImplementationTask {
            task_id: format!("task-{}", i),
            title: format!("Task {}", i),
            description: "Test".to_string(),
            category: if i == 0 { TaskCategory::BluetoothBackends } else { TaskCategory::ProtocolMessages },
            priority: 1,
            status: TaskStatus::Pending,
            effort_hours: 1.0,
            assigned_to: "dev".to_string(),
            dependencies: Vec::new(),
            completion_percentage: 0,
        };
        sprint.add_task(task);
    }
    
    let bluetooth = sprint.get_tasks_by_category(TaskCategory::BluetoothBackends);
    assert_eq!(bluetooth.len(), 1);
}

#[test]
fn test_completion_percentage() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    for i in 0..4 {
        let task = ImplementationTask {
            task_id: format!("task-{}", i),
            title: format!("Task {}", i),
            description: "Test".to_string(),
            category: TaskCategory::BluetoothBackends,
            priority: 1,
            status: TaskStatus::Pending,
            effort_hours: 1.0,
            assigned_to: "dev".to_string(),
            dependencies: Vec::new(),
            completion_percentage: 0,
        };
        sprint.add_task(task);
    }
    
    for i in 0..2 {
        sprint.update_task_status(&format!("task-{}", i), TaskStatus::Completed);
    }
    
    assert_eq!(sprint.get_completion_percentage(), 50);
}

#[test]
fn test_changelog_creation() {
    let log = ChangeLog::new();
    assert_eq!(log.total_changes, 0);
}

#[test]
fn test_add_change_to_log() {
    let mut log = ChangeLog::new();
    let change = AppliedChange {
        change_id: "test-1".to_string(),
        file_path: "test.rs".to_string(),
        change_type: ChangeType::Add,
        old_content: None,
        new_content: Some("code".to_string()),
        timestamp: "2025-11-21T00:00:00Z".to_string(),
        applied_by: "dev".to_string(),
        success: true,
        error_message: None,
    };
    log.add_change(change);
    assert_eq!(log.total_changes, 1);
    assert_eq!(log.successful_changes, 1);
}

#[test]
fn test_change_success_rate() {
    let mut log = ChangeLog::new();
    for i in 0..10 {
        let change = AppliedChange {
            change_id: format!("test-{}", i),
            file_path: format!("test{}.rs", i),
            change_type: ChangeType::Add,
            old_content: None,
            new_content: Some("code".to_string()),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            applied_by: "dev".to_string(),
            success: i < 8,
            error_message: if i >= 8 { Some("Error".to_string()) } else { None },
        };
        log.add_change(change);
    }
    assert_eq!(log.get_success_rate(), 80.0);
}

#[test]
fn test_change_applier_add() {
    let mut applier = ChangeApplier::new();
    let change = applier.apply_add("test.rs".to_string(), "code".to_string());
    assert_eq!(change.change_type, ChangeType::Add);
    assert!(change.success);
}

#[test]
fn test_change_applier_modify() {
    let mut applier = ChangeApplier::new();
    let change = applier.apply_modify("test.rs".to_string(), "old".to_string(), "new".to_string());
    assert_eq!(change.change_type, ChangeType::Modify);
    assert!(change.success);
}

#[test]
fn test_change_applier_delete() {
    let mut applier = ChangeApplier::new();
    let change = applier.apply_delete("test.rs".to_string(), "code".to_string());
    assert_eq!(change.change_type, ChangeType::Delete);
    assert!(change.success);
}

#[test]
fn test_verification_report_creation() {
    let report = VerificationReport::new("report-1".to_string(), "sprint-1".to_string());
    assert_eq!(report.total_tests, 0);
    assert!(report.is_all_passed());
}

#[test]
fn test_add_verification_result() {
    let mut report = VerificationReport::new("report-1".to_string(), "sprint-1".to_string());
    let result = VerificationResult {
        test_name: "test-1".to_string(),
        test_type: TestType::Unit,
        passed: true,
        duration_ms: 100,
        error_message: None,
        timestamp: "2025-11-21T00:00:00Z".to_string(),
    };
    report.add_result(result);
    assert_eq!(report.total_tests, 1);
    assert_eq!(report.passed_tests, 1);
}

#[test]
fn test_verification_pass_rate() {
    let mut report = VerificationReport::new("report-1".to_string(), "sprint-1".to_string());
    for i in 0..10 {
        let result = VerificationResult {
            test_name: format!("test-{}", i),
            test_type: TestType::Unit,
            passed: i < 7,
            duration_ms: 100,
            error_message: None,
            timestamp: "2025-11-21T00:00:00Z".to_string(),
        };
        report.add_result(result);
    }
    assert_eq!(report.get_pass_rate(), 70.0);
}

#[test]
fn test_verifier() {
    let mut verifier = Verifier::new();
    verifier.create_report("report-1".to_string(), "sprint-1".to_string());
    verifier.add_unit_test("report-1", "test-1".to_string(), true, 100);
    
    let report = verifier.get_report("report-1").unwrap();
    assert_eq!(report.total_tests, 1);
}

#[test]
fn test_verifier_coverage() {
    let mut verifier = Verifier::new();
    verifier.create_report("report-1".to_string(), "sprint-1".to_string());
    verifier.set_coverage("report-1", 85.5);
    
    let report = verifier.get_report("report-1").unwrap();
    assert_eq!(report.coverage_percentage, 85.5);
}

#[test]
fn test_verifier_build_status() {
    let mut verifier = Verifier::new();
    verifier.create_report("report-1".to_string(), "sprint-1".to_string());
    verifier.set_build_status("report-1", BuildStatus::Success);
    
    let report = verifier.get_report("report-1").unwrap();
    assert_eq!(report.build_status, BuildStatus::Success);
}

#[test]
fn test_sprint_report_generation() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
    let task = ImplementationTask {
        task_id: "task-1".to_string(),
        title: "Test Task".to_string(),
        description: "A test task".to_string(),
        category: TaskCategory::BluetoothBackends,
        priority: 1,
        status: TaskStatus::Completed,
        effort_hours: 4.0,
        assigned_to: "dev".to_string(),
        dependencies: Vec::new(),
        completion_percentage: 100,
    };
    sprint.add_task(task);
    sprint.update_task_status("task-1", TaskStatus::Completed);
    
    let report = sprint.generate_report();
    assert!(report.contains("IMPLEMENTATION SPRINT REPORT"));
    assert!(report.contains("Sprint 1"));
}

#[test]
fn test_change_log_report_generation() {
    let mut log = ChangeLog::new();
    let change = AppliedChange {
        change_id: "test-1".to_string(),
        file_path: "test.rs".to_string(),
        change_type: ChangeType::Add,
        old_content: None,
        new_content: Some("code".to_string()),
        timestamp: "2025-11-21T00:00:00Z".to_string(),
        applied_by: "dev".to_string(),
        success: true,
        error_message: None,
    };
    log.add_change(change);
    
    let report = log.generate_report();
    assert!(report.contains("CHANGE APPLICATION REPORT"));
}

#[test]
fn test_verification_report_generation() {
    let mut report = VerificationReport::new("report-1".to_string(), "sprint-1".to_string());
    let result = VerificationResult {
        test_name: "test-1".to_string(),
        test_type: TestType::Unit,
        passed: true,
        duration_ms: 100,
        error_message: None,
        timestamp: "2025-11-21T00:00:00Z".to_string(),
    };
    report.add_result(result);
    
    let report_text = report.generate_report();
    assert!(report_text.contains("VERIFICATION REPORT"));
}

#[test]
fn test_complete_phase5_workflow() {
    let mut sprint = SprintPlan::new("sprint-1".to_string(), "Implementation Sprint 1".to_string());
    
    let task = ImplementationTask {
        task_id: "task-1".to_string(),
        title: "Implement Feature".to_string(),
        description: "Implement new feature".to_string(),
        category: TaskCategory::ProtocolMessages,
        priority: 2,
        status: TaskStatus::InProgress,
        effort_hours: 8.0,
        assigned_to: "dev-team".to_string(),
        dependencies: Vec::new(),
        completion_percentage: 50,
    };
    sprint.add_task(task);
    
    let mut applier = ChangeApplier::new();
    let change = applier.apply_add("src/new_feature.rs".to_string(), "code".to_string());
    assert!(change.success);
    
    let mut verifier = Verifier::new();
    verifier.create_report("report-1".to_string(), "sprint-1".to_string());
    verifier.add_unit_test("report-1", "feature_test".to_string(), true, 150);
    verifier.set_coverage("report-1", 90.0);
    verifier.set_build_status("report-1", BuildStatus::Success);
    
    let report = verifier.get_report("report-1").unwrap();
    assert!(report.is_all_passed());
}
