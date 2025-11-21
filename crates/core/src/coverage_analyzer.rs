use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    pub file_path: String,
    pub lines_total: usize,
    pub lines_covered: usize,
    pub branches_total: usize,
    pub branches_covered: usize,
    pub functions_total: usize,
    pub functions_covered: usize,
}

impl CoverageMetrics {
    pub fn get_line_coverage(&self) -> f32 {
        if self.lines_total == 0 {
            return 0.0;
        }
        (self.lines_covered as f32 / self.lines_total as f32) * 100.0
    }

    pub fn get_branch_coverage(&self) -> f32 {
        if self.branches_total == 0 {
            return 0.0;
        }
        (self.branches_covered as f32 / self.branches_total as f32) * 100.0
    }

    pub fn get_function_coverage(&self) -> f32 {
        if self.functions_total == 0 {
            return 0.0;
        }
        (self.functions_covered as f32 / self.functions_total as f32) * 100.0
    }

    pub fn get_overall_coverage(&self) -> f32 {
        let line_cov = self.get_line_coverage();
        let branch_cov = self.get_branch_coverage();
        let func_cov = self.get_function_coverage();
        (line_cov + branch_cov + func_cov) / 3.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    pub report_id: String,
    pub timestamp: String,
    pub metrics: Vec<CoverageMetrics>,
    pub total_lines: usize,
    pub total_lines_covered: usize,
    pub total_branches: usize,
    pub total_branches_covered: usize,
    pub total_functions: usize,
    pub total_functions_covered: usize,
}

impl CoverageReport {
    pub fn new(report_id: String) -> Self {
        Self {
            report_id,
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            metrics: Vec::new(),
            total_lines: 0,
            total_lines_covered: 0,
            total_branches: 0,
            total_branches_covered: 0,
            total_functions: 0,
            total_functions_covered: 0,
        }
    }

    pub fn add_metrics(&mut self, metrics: CoverageMetrics) {
        self.total_lines += metrics.lines_total;
        self.total_lines_covered += metrics.lines_covered;
        self.total_branches += metrics.branches_total;
        self.total_branches_covered += metrics.branches_covered;
        self.total_functions += metrics.functions_total;
        self.total_functions_covered += metrics.functions_covered;
        self.metrics.push(metrics);
    }

    pub fn get_line_coverage(&self) -> f32 {
        if self.total_lines == 0 {
            return 0.0;
        }
        (self.total_lines_covered as f32 / self.total_lines as f32) * 100.0
    }

    pub fn get_branch_coverage(&self) -> f32 {
        if self.total_branches == 0 {
            return 0.0;
        }
        (self.total_branches_covered as f32 / self.total_branches as f32) * 100.0
    }

    pub fn get_function_coverage(&self) -> f32 {
        if self.total_functions == 0 {
            return 0.0;
        }
        (self.total_functions_covered as f32 / self.total_functions as f32) * 100.0
    }

    pub fn get_overall_coverage(&self) -> f32 {
        let line_cov = self.get_line_coverage();
        let branch_cov = self.get_branch_coverage();
        let func_cov = self.get_function_coverage();
        (line_cov + branch_cov + func_cov) / 3.0
    }

    pub fn get_files_below_threshold(&self, threshold: f32) -> Vec<&CoverageMetrics> {
        self.metrics
            .iter()
            .filter(|m| m.get_overall_coverage() < threshold)
            .collect()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== COVERAGE REPORT ===\n\n");

        report.push_str(&format!("Report: {}\n", self.report_id));
        report.push_str(&format!("Timestamp: {}\n\n", self.timestamp));

        report.push_str(&format!("Line Coverage: {:.1}%\n", self.get_line_coverage()));
        report.push_str(&format!("  {} / {} lines\n\n", self.total_lines_covered, self.total_lines));

        report.push_str(&format!("Branch Coverage: {:.1}%\n", self.get_branch_coverage()));
        report.push_str(&format!("  {} / {} branches\n\n", self.total_branches_covered, self.total_branches));

        report.push_str(&format!("Function Coverage: {:.1}%\n", self.get_function_coverage()));
        report.push_str(&format!("  {} / {} functions\n\n", self.total_functions_covered, self.total_functions));

        report.push_str(&format!("Overall Coverage: {:.1}%\n\n", self.get_overall_coverage()));

        report.push_str("Files:\n");
        for metrics in &self.metrics {
            report.push_str(&format!("  {} - {:.1}%\n", metrics.file_path, metrics.get_overall_coverage()));
        }

        report
    }
}

impl Default for CoverageReport {
    fn default() -> Self {
        Self::new("report-1".to_string())
    }
}

pub struct CoverageAnalyzer {
    reports: HashMap<String, CoverageReport>,
}

impl CoverageAnalyzer {
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
        }
    }

    pub fn create_report(&mut self, report_id: String) -> &mut CoverageReport {
        let report = CoverageReport::new(report_id.clone());
        self.reports.insert(report_id.clone(), report);
        self.reports.get_mut(&report_id).unwrap()
    }

    pub fn get_report(&self, report_id: &str) -> Option<&CoverageReport> {
        self.reports.get(report_id)
    }

    pub fn add_metrics(&mut self, report_id: &str, metrics: CoverageMetrics) {
        if let Some(report) = self.reports.get_mut(report_id) {
            report.add_metrics(metrics);
        }
    }

    pub fn is_coverage_sufficient(&self, report_id: &str, threshold: f32) -> bool {
        if let Some(report) = self.get_report(report_id) {
            report.get_overall_coverage() >= threshold
        } else {
            false
        }
    }
}

impl Default for CoverageAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
