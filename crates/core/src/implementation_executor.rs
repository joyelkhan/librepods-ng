use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationTask {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub category: TaskCategory,
    pub priority: u8,
    pub status: TaskStatus,
    pub effort_hours: f32,
    pub assigned_to: String,
    pub dependencies: Vec<String>,
    pub completion_percentage: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskCategory {
    BluetoothBackends,
    ProtocolMessages,
    FFIBridge,
    AndroidMerge,
    BuildPackaging,
    CICD,
    I18nA11y,
    Security,
    Documentation,
    ReleaseChores,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintPlan {
    pub sprint_id: String,
    pub sprint_name: String,
    pub start_date: String,
    pub end_date: String,
    pub tasks: Vec<ImplementationTask>,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub blocked_tasks: usize,
    pub total_effort_hours: f32,
    pub completed_effort_hours: f32,
}

impl SprintPlan {
    pub fn new(sprint_id: String, sprint_name: String) -> Self {
        Self {
            sprint_id,
            sprint_name,
            start_date: "2025-11-21T00:00:00Z".to_string(),
            end_date: "2025-11-28T23:59:59Z".to_string(),
            tasks: Vec::new(),
            total_tasks: 0,
            completed_tasks: 0,
            blocked_tasks: 0,
            total_effort_hours: 0.0,
            completed_effort_hours: 0.0,
        }
    }

    pub fn add_task(&mut self, task: ImplementationTask) {
        self.total_tasks += 1;
        self.total_effort_hours += task.effort_hours;
        self.tasks.push(task);
    }

    pub fn update_task_status(&mut self, task_id: &str, status: TaskStatus) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.task_id == task_id) {
            let old_status = task.status;
            task.status = status;

            if old_status == TaskStatus::Completed && status != TaskStatus::Completed {
                self.completed_tasks = self.completed_tasks.saturating_sub(1);
                self.completed_effort_hours -= task.effort_hours;
            } else if old_status != TaskStatus::Completed && status == TaskStatus::Completed {
                self.completed_tasks += 1;
                self.completed_effort_hours += task.effort_hours;
            }

            if old_status == TaskStatus::Blocked && status != TaskStatus::Blocked {
                self.blocked_tasks = self.blocked_tasks.saturating_sub(1);
            } else if old_status != TaskStatus::Blocked && status == TaskStatus::Blocked {
                self.blocked_tasks += 1;
            }

            true
        } else {
            false
        }
    }

    pub fn update_task_progress(&mut self, task_id: &str, percentage: u8) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.completion_percentage = percentage.min(100);
            true
        } else {
            false
        }
    }

    pub fn get_completion_percentage(&self) -> u8 {
        if self.total_tasks == 0 {
            return 0;
        }
        ((self.completed_tasks as f32 / self.total_tasks as f32) * 100.0) as u8
    }

    pub fn get_effort_completion_percentage(&self) -> f32 {
        if self.total_effort_hours == 0.0 {
            return 0.0;
        }
        (self.completed_effort_hours / self.total_effort_hours) * 100.0
    }

    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<&ImplementationTask> {
        self.tasks.iter().filter(|t| t.status == status).collect()
    }

    pub fn get_tasks_by_category(&self, category: TaskCategory) -> Vec<&ImplementationTask> {
        self.tasks.iter().filter(|t| t.category == category).collect()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== IMPLEMENTATION SPRINT REPORT ===\n\n");

        report.push_str(&format!("Sprint: {}\n", self.sprint_name));
        report.push_str(&format!("Period: {} to {}\n\n", self.start_date, self.end_date));

        report.push_str(&format!("Total Tasks: {}\n", self.total_tasks));
        report.push_str(&format!("  Completed: {}\n", self.completed_tasks));
        report.push_str(&format!("  In Progress: {}\n", self.get_tasks_by_status(TaskStatus::InProgress).len()));
        report.push_str(&format!("  Blocked: {}\n", self.blocked_tasks));
        report.push_str(&format!("  Pending: {}\n\n", self.get_tasks_by_status(TaskStatus::Pending).len()));

        report.push_str(&format!("Completion: {}%\n", self.get_completion_percentage()));
        report.push_str(&format!("Effort: {:.1}h / {:.1}h ({:.1}%)\n\n", 
            self.completed_effort_hours, self.total_effort_hours, self.get_effort_completion_percentage()));

        report.push_str("Tasks:\n");
        for task in &self.tasks {
            report.push_str(&format!("  [{}] {} ({:?}) - {}%\n", 
                task.priority, task.title, task.status, task.completion_percentage));
        }

        report
    }
}

impl Default for SprintPlan {
    fn default() -> Self {
        Self::new("sprint-1".to_string(), "Implementation Sprint 1".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprint_creation() {
        let sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
        assert_eq!(sprint.sprint_id, "sprint-1");
        assert_eq!(sprint.total_tasks, 0);
    }

    #[test]
    fn test_add_task() {
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
    fn test_completion_percentage() {
        let mut sprint = SprintPlan::new("sprint-1".to_string(), "Sprint 1".to_string());
        for i in 0..4 {
            let task = ImplementationTask {
                task_id: format!("task-{}", i),
                title: format!("Task {}", i),
                description: "Test".to_string(),
                category: TaskCategory::BluetoothBackends,
                priority: 1,
                status: if i < 2 { TaskStatus::Completed } else { TaskStatus::Pending },
                effort_hours: 1.0,
                assigned_to: "dev".to_string(),
                dependencies: Vec::new(),
                completion_percentage: if i < 2 { 100 } else { 0 },
            };
            sprint.add_task(task);
        }
        assert_eq!(sprint.get_completion_percentage(), 50);
    }
}
