use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalRisk {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseType {
    GPL3,
    MIT,
    Apache2,
    BSD,
    Proprietary,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseHeader {
    pub file_path: String,
    pub has_header: bool,
    pub license_type: LicenseType,
    pub year: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMCANotice {
    pub date: String,
    pub repository: String,
    pub reason: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPLViolation {
    pub file: String,
    pub violation_type: String,
    pub description: String,
    pub severity: LegalRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrademarkUsage {
    pub term: String,
    pub context: String,
    pub compliant: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareAnalysis {
    pub is_proprietary: bool,
    pub is_nda_protected: bool,
    pub is_apple_firmware: bool,
    pub risk_level: LegalRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalScanResult {
    pub scan_date: String,
    pub dmca_notices: Vec<DMCANotice>,
    pub gpl_violations: Vec<GPLViolation>,
    pub license_headers: Vec<LicenseHeader>,
    pub trademark_usage: Vec<TrademarkUsage>,
    pub firmware_analysis: FirmwareAnalysis,
    pub overall_risk: LegalRisk,
    pub is_safe_to_distribute: bool,
    pub findings: Vec<String>,
    pub recommendations: Vec<String>,
}

impl LegalScanResult {
    pub fn new() -> Self {
        Self {
            scan_date: "2025-11-21T23:55:00Z".to_string(),
            dmca_notices: Vec::new(),
            gpl_violations: Vec::new(),
            license_headers: Vec::new(),
            trademark_usage: Vec::new(),
            firmware_analysis: FirmwareAnalysis {
                is_proprietary: false,
                is_nda_protected: false,
                is_apple_firmware: false,
                risk_level: LegalRisk::None,
            },
            overall_risk: LegalRisk::None,
            is_safe_to_distribute: true,
            findings: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn add_dmca_notice(&mut self, notice: DMCANotice) {
        self.dmca_notices.push(notice);
        self.overall_risk = LegalRisk::Critical;
        self.is_safe_to_distribute = false;
    }

    pub fn add_gpl_violation(&mut self, violation: GPLViolation) {
        let severity = violation.severity.clone();
        if severity == LegalRisk::Critical {
            self.is_safe_to_distribute = false;
        }
        self.gpl_violations.push(violation);
        if self.overall_risk < severity {
            self.overall_risk = severity;
        }
    }

    pub fn add_license_header(&mut self, header: LicenseHeader) {
        self.license_headers.push(header);
    }

    pub fn add_trademark_usage(&mut self, usage: TrademarkUsage) {
        if !usage.compliant {
            self.findings.push(format!("Non-compliant trademark usage: {}", usage.term));
        }
        self.trademark_usage.push(usage);
    }

    pub fn set_firmware_analysis(&mut self, analysis: FirmwareAnalysis) {
        if analysis.is_nda_protected || analysis.is_proprietary {
            self.is_safe_to_distribute = false;
            self.overall_risk = LegalRisk::Critical;
        }
        self.firmware_analysis = analysis;
    }

    pub fn add_finding(&mut self, finding: String) {
        self.findings.push(finding);
    }

    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }

    pub fn has_critical_issues(&self) -> bool {
        !self.dmca_notices.is_empty()
            || self.gpl_violations.iter().any(|v| v.severity == LegalRisk::Critical)
            || self.firmware_analysis.is_nda_protected
    }

    pub fn get_license_consistency(&self) -> f32 {
        if self.license_headers.is_empty() {
            return 0.0;
        }

        let consistent = self
            .license_headers
            .iter()
            .filter(|h| h.has_header && h.license_type == LicenseType::GPL3)
            .count();

        (consistent as f32 / self.license_headers.len() as f32) * 100.0
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== LEGAL SCAN REPORT ===\n\n");

        report.push_str(&format!("Scan Date: {}\n", self.scan_date));
        report.push_str(&format!("Overall Risk Level: {:?}\n", self.overall_risk));
        report.push_str(&format!("Safe to Distribute: {}\n\n", self.is_safe_to_distribute));

        report.push_str(&format!("DMCA Notices: {}\n", self.dmca_notices.len()));
        for notice in &self.dmca_notices {
            report.push_str(&format!("  - {} ({})\n", notice.repository, notice.date));
        }

        report.push_str(&format!("\nGPL Violations: {}\n", self.gpl_violations.len()));
        for violation in &self.gpl_violations {
            report.push_str(&format!("  - {} ({:?})\n", violation.file, violation.severity));
        }

        report.push_str(&format!("\nLicense Headers: {}\n", self.license_headers.len()));
        report.push_str(&format!("  Consistency: {:.1}%\n", self.get_license_consistency()));

        report.push_str(&format!("\nTrademark Usage: {}\n", self.trademark_usage.len()));
        for usage in &self.trademark_usage {
            let status = if usage.compliant { "✓" } else { "✗" };
            report.push_str(&format!("  {} {}\n", status, usage.term));
        }

        report.push_str(&format!("\nFirmware Analysis:\n"));
        report.push_str(&format!("  Proprietary: {}\n", self.firmware_analysis.is_proprietary));
        report.push_str(&format!("  NDA Protected: {}\n", self.firmware_analysis.is_nda_protected));
        report.push_str(&format!("  Apple Firmware: {}\n", self.firmware_analysis.is_apple_firmware));

        report.push_str(&format!("\nFindings: {}\n", self.findings.len()));
        for finding in &self.findings {
            report.push_str(&format!("  - {}\n", finding));
        }

        report.push_str(&format!("\nRecommendations: {}\n", self.recommendations.len()));
        for rec in &self.recommendations {
            report.push_str(&format!("  - {}\n", rec));
        }

        report
    }
}

impl Default for LegalScanResult {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialOrd for LegalRisk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LegalRisk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_val = match self {
            LegalRisk::None => 0,
            LegalRisk::Low => 1,
            LegalRisk::Medium => 2,
            LegalRisk::High => 3,
            LegalRisk::Critical => 4,
        };
        let other_val = match other {
            LegalRisk::None => 0,
            LegalRisk::Low => 1,
            LegalRisk::Medium => 2,
            LegalRisk::High => 3,
            LegalRisk::Critical => 4,
        };
        self_val.cmp(&other_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_scan_creation() {
        let scan = LegalScanResult::new();
        assert_eq!(scan.overall_risk, LegalRisk::None);
        assert!(scan.is_safe_to_distribute);
    }

    #[test]
    fn test_add_dmca_notice() {
        let mut scan = LegalScanResult::new();
        let notice = DMCANotice {
            date: "2025-11-21".to_string(),
            repository: "test/repo".to_string(),
            reason: "Copyright claim".to_string(),
            status: "Active".to_string(),
        };
        scan.add_dmca_notice(notice);
        assert_eq!(scan.dmca_notices.len(), 1);
        assert_eq!(scan.overall_risk, LegalRisk::Critical);
        assert!(!scan.is_safe_to_distribute);
    }

    #[test]
    fn test_license_consistency() {
        let mut scan = LegalScanResult::new();
        scan.add_license_header(LicenseHeader {
            file_path: "file1.rs".to_string(),
            has_header: true,
            license_type: LicenseType::GPL3,
            year: Some("2025".to_string()),
            author: Some("Test".to_string()),
        });
        scan.add_license_header(LicenseHeader {
            file_path: "file2.rs".to_string(),
            has_header: true,
            license_type: LicenseType::GPL3,
            year: Some("2025".to_string()),
            author: Some("Test".to_string()),
        });
        assert_eq!(scan.get_license_consistency(), 100.0);
    }

    #[test]
    fn test_trademark_compliance() {
        let mut scan = LegalScanResult::new();
        scan.add_trademark_usage(TrademarkUsage {
            term: "AirPods".to_string(),
            context: "Descriptive use".to_string(),
            compliant: true,
            reason: "Used descriptively only".to_string(),
        });
        assert_eq!(scan.trademark_usage.len(), 1);
    }

    #[test]
    fn test_firmware_analysis() {
        let mut scan = LegalScanResult::new();
        let analysis = FirmwareAnalysis {
            is_proprietary: false,
            is_nda_protected: false,
            is_apple_firmware: false,
            risk_level: LegalRisk::None,
        };
        scan.set_firmware_analysis(analysis);
        assert!(scan.is_safe_to_distribute);
    }

    #[test]
    fn test_gpl_violation() {
        let mut scan = LegalScanResult::new();
        let violation = GPLViolation {
            file: "test.rs".to_string(),
            violation_type: "Missing header".to_string(),
            description: "GPL-3.0 header missing".to_string(),
            severity: LegalRisk::High,
        };
        scan.add_gpl_violation(violation);
        assert_eq!(scan.gpl_violations.len(), 1);
        assert_eq!(scan.overall_risk, LegalRisk::High);
    }
}
