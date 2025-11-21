use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub test_name: String,
    pub test_type: TestType,
    pub passed: bool,
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    Build,
    Lint,
    Coverage,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub report_id: String,
    pub sprint_id: String,
    pub results: Vec<VerificationResult>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub build_status: BuildStatus,
    pub coverage_percentage: f32,
    pub security_issues: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildStatus {
    Success,
    Warning,
    Failure,
}

impl VerificationReport {
    pub fn new(report_id: String, sprint_id: String) -> Self {
        Self {
            report_id,
            sprint_id,
            results: Vec::new(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            build_status: BuildStatus::Success,
            coverage_percentage: 0.0,
            security_issues: 0,
        }
    }

    pub fn add_result(&mut self, result: VerificationResult) {
        self.total_tests += 1;
        if result.passed {
            self.passed_tests += 1;
        } else {
            self.failed_tests += 1;
        }
        self.results.push(result);
    }

    pub fn get_pass_rate(&self) -> f32 {
        if self.total_tests == 0 {
            return 0.0;
        }
        (self.passed_tests as f32 / self.total_tests as f32) * 100.0
    }

    pub fn get_results_by_type(&self, test_type: TestType) -> Vec<&VerificationResult> {
        self.results.iter().filter(|r| r.test_type == test_type).collect()
    }

    pub fn get_failed_results(&self) -> Vec<&VerificationResult> {
        self.results.iter().filter(|r| !r.passed).collect()
    }

    pub fn is_all_passed(&self) -> bool {
        self.failed_tests == 0 && self.build_status == BuildStatus::Success
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== VERIFICATION REPORT ===\n\n");

        report.push_str(&format!("Sprint: {}\n", self.sprint_id));
        report.push_str(&format!("Build Status: {:?}\n\n", self.build_status));

        report.push_str(&format!("Total Tests: {}\n", self.total_tests));
        report.push_str(&format!("  Passed: {}\n", self.passed_tests));
        report.push_str(&format!("  Failed: {}\n", self.failed_tests));
        report.push_str(&format!("Pass Rate: {:.1}%\n\n", self.get_pass_rate()));

        report.push_str(&format!("Coverage: {:.1}%\n", self.coverage_percentage));
        report.push_str(&format!("Security Issues: {}\n\n", self.security_issues));

        let unit_tests = self.get_results_by_type(TestType::Unit);
        let integration_tests = self.get_results_by_type(TestType::Integration);
        let build_tests = self.get_results_by_type(TestType::Build);

        report.push_str(&format!("Unit Tests: {}\n", unit_tests.len()));
        report.push_str(&format!("Integration Tests: {}\n", integration_tests.len()));
        report.push_str(&format!("Build Tests: {}\n\n", build_tests.len()));

        let failed = self.get_failed_results();
        if !failed.is_empty() {
            report.push_str(&format!("Failed Tests: {}\n", failed.len()));
            for result in failed {
                report.push_str(&format!("  {} - {}\n", result.test_name, 
                    result.error_message.as_ref().unwrap_or(&"Unknown error".to_string())));
            }
        }

        report
    }
}

impl Default for VerificationReport {
    fn default() -> Self {
        Self::new("report-1".to_string(), "sprint-1".to_string())
    }
}

pub struct Verifier {
    reports: HashMap<String, VerificationReport>,
}

impl Verifier {
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
        }
    }

    pub fn create_report(&mut self, report_id: String, sprint_id: String) -> &mut VerificationReport {
        let report = VerificationReport::new(report_id.clone(), sprint_id);
        self.reports.insert(report_id.clone(), report);
        self.reports.get_mut(&report_id).unwrap()
    }

    pub fn get_report(&self, report_id: &str) -> Option<&VerificationReport> {
        self.reports.get(report_id)
    }

    pub fn add_unit_test(&mut self, report_id: &str, test_name: String, passed: bool, duration_ms: u64) {
        if let Some(report) = self.reports.get_mut(report_id) {
            let result = VerificationResult {
                test_name,
                test_type: TestType::Unit,
                passed,
                duration_ms,
                error_message: None,
                timestamp: "2025-11-21T00:00:00Z".to_string(),
            };
            report.add_result(result);
        }
    }

    pub fn add_integration_test(&mut self, report_id: &str, test_name: String, passed: bool, duration_ms: u64) {
        if let Some(report) = self.reports.get_mut(report_id) {
            let result = VerificationResult {
                test_name,
                test_type: TestType::Integration,
                passed,
                duration_ms,
                error_message: None,
                timestamp: "2025-11-21T00:00:00Z".to_string(),
            };
            report.add_result(result);
        }
    }

    pub fn set_build_status(&mut self, report_id: &str, status: BuildStatus) {
        if let Some(report) = self.reports.get_mut(report_id) {
            report.build_status = status;
        }
    }

    pub fn set_coverage(&mut self, report_id: &str, percentage: f32) {
        if let Some(report) = self.reports.get_mut(report_id) {
            report.coverage_percentage = percentage.min(100.0);
        }
    }
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_report_creation() {
        let report = VerificationReport::new("report-1".to_string(), "sprint-1".to_string());
        assert_eq!(report.total_tests, 0);
        assert!(report.is_all_passed());
    }

    #[test]
    fn test_add_result() {
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
    fn test_pass_rate() {
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
}
