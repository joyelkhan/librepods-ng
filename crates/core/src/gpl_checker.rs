use crate::legal_scan::{GPLViolation, LegalRisk, LicenseHeader, LicenseType};
use std::collections::HashMap;

pub struct GPLChecker {
    required_header: String,
    known_violations: Vec<GPLViolation>,
    license_compatibility: HashMap<String, bool>,
}

impl GPLChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            required_header: "GPL-3.0-or-later WITH Classpath-exception-2.0".to_string(),
            known_violations: Vec::new(),
            license_compatibility: HashMap::new(),
        };
        checker.initialize_compatibility();
        checker
    }

    fn initialize_compatibility(&mut self) {
        self.license_compatibility.insert("GPL-3.0".to_string(), true);
        self.license_compatibility.insert("GPL-2.0".to_string(), false);
        self.license_compatibility.insert("MIT".to_string(), true);
        self.license_compatibility.insert("Apache-2.0".to_string(), true);
        self.license_compatibility.insert("BSD".to_string(), true);
        self.license_compatibility.insert("Proprietary".to_string(), false);
    }

    pub fn check_license_header(&self, file_path: &str, content: &str) -> LicenseHeader {
        let has_header = content.contains("GPL-3.0") || content.contains("GPL-3");
        let license_type = if has_header {
            LicenseType::GPL3
        } else {
            LicenseType::Unknown
        };

        let year = self.extract_year(content);
        let author = self.extract_author(content);

        LicenseHeader {
            file_path: file_path.to_string(),
            has_header,
            license_type,
            year,
            author,
        }
    }

    fn extract_year(&self, content: &str) -> Option<String> {
        for line in content.lines().take(20) {
            if line.contains("202") {
                if let Some(year) = line.split_whitespace().find(|w| w.starts_with("202")) {
                    return Some(year.to_string());
                }
            }
        }
        None
    }

    fn extract_author(&self, content: &str) -> Option<String> {
        for line in content.lines().take(20) {
            if line.contains("Author") || line.contains("author") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 1 {
                    return Some(parts[1].trim().to_string());
                }
            }
        }
        None
    }

    pub fn check_license_compatibility(&self, license: &str) -> bool {
        self.license_compatibility
            .get(license)
            .copied()
            .unwrap_or(false)
    }

    pub fn verify_all_files_have_headers(&self, files: Vec<LicenseHeader>) -> (usize, usize) {
        let total = files.len();
        let with_headers = files.iter().filter(|f| f.has_header).count();
        (with_headers, total)
    }

    pub fn check_for_violations(&self, files: Vec<LicenseHeader>) -> Vec<GPLViolation> {
        let mut violations = Vec::new();

        for file in files {
            if !file.has_header {
                violations.push(GPLViolation {
                    file: file.file_path.clone(),
                    violation_type: "Missing GPL header".to_string(),
                    description: "File does not contain GPL-3.0 license header".to_string(),
                    severity: LegalRisk::Medium,
                });
            }

            if file.license_type != LicenseType::GPL3 && file.license_type != LicenseType::Unknown {
                violations.push(GPLViolation {
                    file: file.file_path.clone(),
                    violation_type: "Incompatible license".to_string(),
                    description: format!("File has {:?} license, expected GPL-3.0", file.license_type),
                    severity: LegalRisk::High,
                });
            }
        }

        violations
    }

    pub fn check_dependency_licenses(&self, dependencies: Vec<(String, String)>) -> Vec<GPLViolation> {
        let mut violations = Vec::new();

        for (dep_name, license) in dependencies {
            if !self.check_license_compatibility(&license) {
                violations.push(GPLViolation {
                    file: format!("Cargo.toml ({})", dep_name),
                    violation_type: "Incompatible dependency license".to_string(),
                    description: format!("Dependency {} has incompatible license: {}", dep_name, license),
                    severity: LegalRisk::High,
                });
            }
        }

        violations
    }

    pub fn generate_gpl_report(&self, files: Vec<LicenseHeader>) -> String {
        let mut report = String::new();
        report.push_str("=== GPL COMPLIANCE REPORT ===\n\n");

        let (with_headers, total) = self.verify_all_files_have_headers(files.clone());
        let percentage = (with_headers as f32 / total as f32) * 100.0;

        report.push_str(&format!("Files with GPL headers: {}/{} ({:.1}%)\n", with_headers, total, percentage));

        let violations = self.check_for_violations(files);
        report.push_str(&format!("Violations found: {}\n\n", violations.len()));

        for violation in violations {
            report.push_str(&format!("  {} ({})\n", violation.file, violation.violation_type));
            report.push_str(&format!("    {}\n", violation.description));
        }

        report
    }
}

impl Default for GPLChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_creation() {
        let checker = GPLChecker::new();
        assert_eq!(checker.required_header, "GPL-3.0-or-later WITH Classpath-exception-2.0");
    }

    #[test]
    fn test_check_license_header() {
        let checker = GPLChecker::new();
        let content = "// GPL-3.0 License\n// Copyright 2025";
        let header = checker.check_license_header("test.rs", content);
        assert!(header.has_header);
        assert_eq!(header.license_type, LicenseType::GPL3);
    }

    #[test]
    fn test_license_compatibility() {
        let checker = GPLChecker::new();
        assert!(checker.check_license_compatibility("GPL-3.0"));
        assert!(checker.check_license_compatibility("MIT"));
        assert!(!checker.check_license_compatibility("Proprietary"));
    }

    #[test]
    fn test_verify_headers() {
        let checker = GPLChecker::new();
        let files = vec![
            LicenseHeader {
                file_path: "file1.rs".to_string(),
                has_header: true,
                license_type: LicenseType::GPL3,
                year: Some("2025".to_string()),
                author: None,
            },
            LicenseHeader {
                file_path: "file2.rs".to_string(),
                has_header: false,
                license_type: LicenseType::Unknown,
                year: None,
                author: None,
            },
        ];
        let (with, total) = checker.verify_all_files_have_headers(files);
        assert_eq!(with, 1);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_check_violations() {
        let checker = GPLChecker::new();
        let files = vec![LicenseHeader {
            file_path: "test.rs".to_string(),
            has_header: false,
            license_type: LicenseType::Unknown,
            year: None,
            author: None,
        }];
        let violations = checker.check_for_violations(files);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_dependency_licenses() {
        let checker = GPLChecker::new();
        let deps = vec![
            ("serde".to_string(), "MIT".to_string()),
            ("proprietary-lib".to_string(), "Proprietary".to_string()),
        ];
        let violations = checker.check_dependency_licenses(deps);
        assert_eq!(violations.len(), 1);
    }
}
