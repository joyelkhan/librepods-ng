use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub suite_id: String,
    pub suite_name: String,
    pub test_type: TestSuiteType,
    pub tests: Vec<TestCase>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub total_duration_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestSuiteType {
    Unit,
    Integration,
    E2E,
    Performance,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub test_id: String,
    pub test_name: String,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub stack_trace: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Timeout,
}

impl TestSuite {
    pub fn new(suite_id: String, suite_name: String, test_type: TestSuiteType) -> Self {
        Self {
            suite_id,
            suite_name,
            test_type,
            tests: Vec::new(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            total_duration_ms: 0,
        }
    }

    pub fn add_test(&mut self, test: TestCase) {
        self.total_tests += 1;
        self.total_duration_ms += test.duration_ms;

        match test.status {
            TestStatus::Passed => self.passed_tests += 1,
            TestStatus::Failed => self.failed_tests += 1,
            TestStatus::Skipped => self.skipped_tests += 1,
            TestStatus::Timeout => self.failed_tests += 1,
        }

        self.tests.push(test);
    }

    pub fn get_pass_rate(&self) -> f32 {
        if self.total_tests == 0 {
            return 0.0;
        }
        (self.passed_tests as f32 / self.total_tests as f32) * 100.0
    }

    pub fn is_all_passed(&self) -> bool {
        self.failed_tests == 0 && self.skipped_tests == 0
    }

    pub fn get_tests_by_status(&self, status: TestStatus) -> Vec<&TestCase> {
        self.tests.iter().filter(|t| t.status == status).collect()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== TEST SUITE REPORT ===\n\n");

        report.push_str(&format!("Suite: {}\n", self.suite_name));
        report.push_str(&format!("Type: {:?}\n\n", self.test_type));

        report.push_str(&format!("Total Tests: {}\n", self.total_tests));
        report.push_str(&format!("  Passed: {}\n", self.passed_tests));
        report.push_str(&format!("  Failed: {}\n", self.failed_tests));
        report.push_str(&format!("  Skipped: {}\n", self.skipped_tests));
        report.push_str(&format!("Pass Rate: {:.1}%\n", self.get_pass_rate()));
        report.push_str(&format!("Duration: {:.1}s\n\n", self.total_duration_ms as f32 / 1000.0));

        let failed = self.get_tests_by_status(TestStatus::Failed);
        if !failed.is_empty() {
            report.push_str(&format!("Failed Tests: {}\n", failed.len()));
            for test in failed {
                report.push_str(&format!("  {} - {}\n", test.test_name, 
                    test.error_message.as_ref().unwrap_or(&"Unknown error".to_string())));
            }
        }

        report
    }
}

impl Default for TestSuite {
    fn default() -> Self {
        Self::new("suite-1".to_string(), "Test Suite 1".to_string(), TestSuiteType::Unit)
    }
}

pub struct TestRunner {
    suites: HashMap<String, TestSuite>,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            suites: HashMap::new(),
        }
    }

    pub fn create_suite(&mut self, suite_id: String, suite_name: String, test_type: TestSuiteType) -> &mut TestSuite {
        let suite = TestSuite::new(suite_id.clone(), suite_name, test_type);
        self.suites.insert(suite_id.clone(), suite);
        self.suites.get_mut(&suite_id).unwrap()
    }

    pub fn get_suite(&self, suite_id: &str) -> Option<&TestSuite> {
        self.suites.get(suite_id)
    }

    pub fn add_test(&mut self, suite_id: &str, test: TestCase) {
        if let Some(suite) = self.suites.get_mut(suite_id) {
            suite.add_test(test);
        }
    }

    pub fn get_total_tests(&self) -> usize {
        self.suites.values().map(|s| s.total_tests).sum()
    }

    pub fn get_total_passed(&self) -> usize {
        self.suites.values().map(|s| s.passed_tests).sum()
    }

    pub fn get_total_failed(&self) -> usize {
        self.suites.values().map(|s| s.failed_tests).sum()
    }

    pub fn get_overall_pass_rate(&self) -> f32 {
        let total = self.get_total_tests();
        if total == 0 {
            return 0.0;
        }
        (self.get_total_passed() as f32 / total as f32) * 100.0
    }

    pub fn is_all_passed(&self) -> bool {
        self.get_total_failed() == 0
    }

    pub fn generate_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("=== TEST EXECUTION SUMMARY ===\n\n");

        summary.push_str(&format!("Total Suites: {}\n", self.suites.len()));
        summary.push_str(&format!("Total Tests: {}\n", self.get_total_tests()));
        summary.push_str(&format!("  Passed: {}\n", self.get_total_passed()));
        summary.push_str(&format!("  Failed: {}\n", self.get_total_failed()));
        summary.push_str(&format!("Overall Pass Rate: {:.1}%\n\n", self.get_overall_pass_rate()));

        summary.push_str("Suites:\n");
        for suite in self.suites.values() {
            summary.push_str(&format!("  {} ({:?}) - {:.1}%\n", 
                suite.suite_name, suite.test_type, suite.get_pass_rate()));
        }

        summary
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suite_creation() {
        let suite = TestSuite::new("suite-1".to_string(), "Suite 1".to_string(), TestSuiteType::Unit);
        assert_eq!(suite.suite_id, "suite-1");
        assert_eq!(suite.total_tests, 0);
    }

    #[test]
    fn test_add_test() {
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
    fn test_pass_rate() {
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
    fn test_runner() {
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
}
