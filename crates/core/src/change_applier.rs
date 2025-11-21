use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedChange {
    pub change_id: String,
    pub file_path: String,
    pub change_type: ChangeType,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub timestamp: String,
    pub applied_by: String,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Add,
    Modify,
    Delete,
    Rename,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeLog {
    pub changes: Vec<AppliedChange>,
    pub total_changes: usize,
    pub successful_changes: usize,
    pub failed_changes: usize,
    pub start_time: String,
    pub end_time: String,
}

impl ChangeLog {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            total_changes: 0,
            successful_changes: 0,
            failed_changes: 0,
            start_time: "2025-11-21T00:00:00Z".to_string(),
            end_time: String::new(),
        }
    }

    pub fn add_change(&mut self, change: AppliedChange) {
        self.total_changes += 1;
        if change.success {
            self.successful_changes += 1;
        } else {
            self.failed_changes += 1;
        }
        self.changes.push(change);
    }

    pub fn get_success_rate(&self) -> f32 {
        if self.total_changes == 0 {
            return 0.0;
        }
        (self.successful_changes as f32 / self.total_changes as f32) * 100.0
    }

    pub fn get_changes_by_type(&self, change_type: ChangeType) -> Vec<&AppliedChange> {
        self.changes.iter().filter(|c| c.change_type == change_type).collect()
    }

    pub fn get_failed_changes(&self) -> Vec<&AppliedChange> {
        self.changes.iter().filter(|c| !c.success).collect()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== CHANGE APPLICATION REPORT ===\n\n");

        report.push_str(&format!("Period: {} to {}\n", self.start_time, self.end_time));
        report.push_str(&format!("Total Changes: {}\n", self.total_changes));
        report.push_str(&format!("  Successful: {}\n", self.successful_changes));
        report.push_str(&format!("  Failed: {}\n", self.failed_changes));
        report.push_str(&format!("Success Rate: {:.1}%\n\n", self.get_success_rate()));

        let adds = self.get_changes_by_type(ChangeType::Add).len();
        let modifies = self.get_changes_by_type(ChangeType::Modify).len();
        let deletes = self.get_changes_by_type(ChangeType::Delete).len();
        let renames = self.get_changes_by_type(ChangeType::Rename).len();

        report.push_str(&format!("Changes by Type:\n"));
        report.push_str(&format!("  Add: {}\n", adds));
        report.push_str(&format!("  Modify: {}\n", modifies));
        report.push_str(&format!("  Delete: {}\n", deletes));
        report.push_str(&format!("  Rename: {}\n\n", renames));

        let failed = self.get_failed_changes();
        if !failed.is_empty() {
            report.push_str(&format!("Failed Changes: {}\n", failed.len()));
            for change in failed {
                report.push_str(&format!("  {} - {}\n", change.file_path, 
                    change.error_message.as_ref().unwrap_or(&"Unknown error".to_string())));
            }
        }

        report
    }
}

impl Default for ChangeLog {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ChangeApplier {
    applied_changes: HashMap<String, AppliedChange>,
}

impl ChangeApplier {
    pub fn new() -> Self {
        Self {
            applied_changes: HashMap::new(),
        }
    }

    pub fn apply_add(&mut self, file_path: String, content: String) -> AppliedChange {
        let change = AppliedChange {
            change_id: format!("add-{}", file_path.replace('/', "-")),
            file_path: file_path.clone(),
            change_type: ChangeType::Add,
            old_content: None,
            new_content: Some(content),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            applied_by: "system".to_string(),
            success: true,
            error_message: None,
        };
        self.applied_changes.insert(change.change_id.clone(), change.clone());
        change
    }

    pub fn apply_modify(&mut self, file_path: String, old_content: String, new_content: String) -> AppliedChange {
        let change = AppliedChange {
            change_id: format!("modify-{}", file_path.replace('/', "-")),
            file_path: file_path.clone(),
            change_type: ChangeType::Modify,
            old_content: Some(old_content),
            new_content: Some(new_content),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            applied_by: "system".to_string(),
            success: true,
            error_message: None,
        };
        self.applied_changes.insert(change.change_id.clone(), change.clone());
        change
    }

    pub fn apply_delete(&mut self, file_path: String, old_content: String) -> AppliedChange {
        let change = AppliedChange {
            change_id: format!("delete-{}", file_path.replace('/', "-")),
            file_path: file_path.clone(),
            change_type: ChangeType::Delete,
            old_content: Some(old_content),
            new_content: None,
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            applied_by: "system".to_string(),
            success: true,
            error_message: None,
        };
        self.applied_changes.insert(change.change_id.clone(), change.clone());
        change
    }

    pub fn apply_rename(&mut self, old_path: String, new_path: String) -> AppliedChange {
        let change = AppliedChange {
            change_id: format!("rename-{}-{}", old_path.replace('/', "-"), new_path.replace('/', "-")),
            file_path: new_path,
            change_type: ChangeType::Rename,
            old_content: Some(old_path),
            new_content: None,
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            applied_by: "system".to_string(),
            success: true,
            error_message: None,
        };
        self.applied_changes.insert(change.change_id.clone(), change.clone());
        change
    }

    pub fn get_applied_change(&self, change_id: &str) -> Option<&AppliedChange> {
        self.applied_changes.get(change_id)
    }

    pub fn get_all_changes(&self) -> Vec<&AppliedChange> {
        self.applied_changes.values().collect()
    }
}

impl Default for ChangeApplier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_changelog_creation() {
        let log = ChangeLog::new();
        assert_eq!(log.total_changes, 0);
    }

    #[test]
    fn test_add_change() {
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
    fn test_success_rate() {
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
    fn test_change_applier() {
        let mut applier = ChangeApplier::new();
        let change = applier.apply_add("test.rs".to_string(), "code".to_string());
        assert_eq!(change.change_type, ChangeType::Add);
        assert!(change.success);
    }
}
