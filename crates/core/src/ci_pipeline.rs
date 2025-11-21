use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub stage_id: String,
    pub stage_name: String,
    pub description: String,
    pub commands: Vec<String>,
    pub timeout_seconds: u64,
    pub status: StageStatus,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageStatus {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIPipeline {
    pub pipeline_id: String,
    pub branch: String,
    pub commit_hash: String,
    pub stages: Vec<PipelineStage>,
    pub total_stages: usize,
    pub completed_stages: usize,
    pub failed_stages: usize,
    pub total_duration_ms: u64,
    pub started_at: String,
    pub completed_at: Option<String>,
}

impl CIPipeline {
    pub fn new(pipeline_id: String, branch: String, commit_hash: String) -> Self {
        Self {
            pipeline_id,
            branch,
            commit_hash,
            stages: Vec::new(),
            total_stages: 0,
            completed_stages: 0,
            failed_stages: 0,
            total_duration_ms: 0,
            started_at: "2025-11-21T00:00:00Z".to_string(),
            completed_at: None,
        }
    }

    pub fn add_stage(&mut self, stage: PipelineStage) {
        self.total_stages += 1;
        self.stages.push(stage);
    }

    pub fn update_stage_status(&mut self, stage_id: &str, status: StageStatus, duration_ms: u64) -> bool {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.stage_id == stage_id) {
            stage.status = status;
            stage.duration_ms = duration_ms;
            self.total_duration_ms += duration_ms;

            if status == StageStatus::Success {
                self.completed_stages += 1;
            } else if status == StageStatus::Failed {
                self.failed_stages += 1;
            }

            true
        } else {
            false
        }
    }

    pub fn is_successful(&self) -> bool {
        self.failed_stages == 0 && self.completed_stages == self.total_stages
    }

    pub fn get_success_rate(&self) -> f32 {
        if self.total_stages == 0 {
            return 0.0;
        }
        (self.completed_stages as f32 / self.total_stages as f32) * 100.0
    }

    pub fn get_stages_by_status(&self, status: StageStatus) -> Vec<&PipelineStage> {
        self.stages.iter().filter(|s| s.status == status).collect()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== CI PIPELINE REPORT ===\n\n");

        report.push_str(&format!("Pipeline: {}\n", self.pipeline_id));
        report.push_str(&format!("Branch: {}\n", self.branch));
        report.push_str(&format!("Commit: {}\n", self.commit_hash));
        report.push_str(&format!("Started: {}\n\n", self.started_at));

        report.push_str(&format!("Total Stages: {}\n", self.total_stages));
        report.push_str(&format!("  Completed: {}\n", self.completed_stages));
        report.push_str(&format!("  Failed: {}\n", self.failed_stages));
        report.push_str(&format!("Success Rate: {:.1}%\n", self.get_success_rate()));
        report.push_str(&format!("Duration: {:.1}s\n\n", self.total_duration_ms as f32 / 1000.0));

        report.push_str("Stages:\n");
        for stage in &self.stages {
            report.push_str(&format!("  {} ({:?}) - {:.1}s\n", 
                stage.stage_name, stage.status, stage.duration_ms as f32 / 1000.0));
        }

        report
    }
}

impl Default for CIPipeline {
    fn default() -> Self {
        Self::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
        assert_eq!(pipeline.pipeline_id, "pipeline-1");
        assert_eq!(pipeline.total_stages, 0);
    }

    #[test]
    fn test_add_stage() {
        let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
        let stage = PipelineStage {
            stage_id: "stage-1".to_string(),
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
    fn test_update_stage_status() {
        let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
        let stage = PipelineStage {
            stage_id: "stage-1".to_string(),
            stage_name: "Build".to_string(),
            description: "Build stage".to_string(),
            commands: vec!["cargo build".to_string()],
            timeout_seconds: 300,
            status: StageStatus::Pending,
            duration_ms: 0,
            error_message: None,
        };
        pipeline.add_stage(stage);
        pipeline.update_stage_status("stage-1", StageStatus::Success, 5000);
        assert_eq!(pipeline.completed_stages, 1);
    }

    #[test]
    fn test_is_successful() {
        let mut pipeline = CIPipeline::new("pipeline-1".to_string(), "main".to_string(), "abc123".to_string());
        let stage = PipelineStage {
            stage_id: "stage-1".to_string(),
            stage_name: "Build".to_string(),
            description: "Build stage".to_string(),
            commands: vec!["cargo build".to_string()],
            timeout_seconds: 300,
            status: StageStatus::Pending,
            duration_ms: 0,
            error_message: None,
        };
        pipeline.add_stage(stage);
        pipeline.update_stage_status("stage-1", StageStatus::Success, 5000);
        assert!(pipeline.is_successful());
    }
}
