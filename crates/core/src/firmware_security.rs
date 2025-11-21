use crate::legal_scan::{FirmwareAnalysis, LegalRisk};

pub struct FirmwareSecurityAnalyzer {
    apple_firmware_signatures: Vec<String>,
    nda_keywords: Vec<String>,
    proprietary_indicators: Vec<String>,
}

impl FirmwareSecurityAnalyzer {
    pub fn new() -> Self {
        Self {
            apple_firmware_signatures: vec![
                "Apple".to_string(),
                "Copyright Apple".to_string(),
                "Apple Inc.".to_string(),
                "Proprietary".to_string(),
                "Confidential".to_string(),
            ],
            nda_keywords: vec![
                "NDA".to_string(),
                "Non-Disclosure".to_string(),
                "Confidential".to_string(),
                "Trade Secret".to_string(),
                "Restricted".to_string(),
            ],
            proprietary_indicators: vec![
                "Proprietary".to_string(),
                "Closed Source".to_string(),
                "License Required".to_string(),
                "Commercial".to_string(),
            ],
        }
    }

    pub fn analyze_firmware(&self, firmware_data: &str) -> FirmwareAnalysis {
        let is_apple_firmware = self.detect_apple_firmware(firmware_data);
        let is_nda_protected = self.detect_nda_protection(firmware_data);
        let is_proprietary = self.detect_proprietary(firmware_data);

        let risk_level = if is_nda_protected || is_proprietary {
            LegalRisk::Critical
        } else if is_apple_firmware {
            LegalRisk::High
        } else {
            LegalRisk::None
        };

        FirmwareAnalysis {
            is_proprietary,
            is_nda_protected,
            is_apple_firmware,
            risk_level,
        }
    }

    fn detect_apple_firmware(&self, data: &str) -> bool {
        self.apple_firmware_signatures
            .iter()
            .any(|sig| data.contains(sig))
    }

    fn detect_nda_protection(&self, data: &str) -> bool {
        self.nda_keywords.iter().any(|keyword| data.contains(keyword))
    }

    fn detect_proprietary(&self, data: &str) -> bool {
        self.proprietary_indicators
            .iter()
            .any(|indicator| data.contains(indicator))
    }

    pub fn check_firmware_file(&self, file_path: &str, content: &str) -> FirmwareAnalysis {
        let mut analysis = self.analyze_firmware(content);

        if file_path.ends_with(".bin") || file_path.ends_with(".hex") {
            analysis.is_proprietary = true;
        }

        analysis
    }

    pub fn scan_firmware_bundle(&self, files: Vec<(String, String)>) -> Vec<FirmwareAnalysis> {
        files
            .iter()
            .map(|(path, content)| self.check_firmware_file(path, content))
            .collect()
    }

    pub fn is_safe_to_distribute(&self, analysis: &FirmwareAnalysis) -> bool {
        !analysis.is_nda_protected && !analysis.is_proprietary && !analysis.is_apple_firmware
    }

    pub fn get_risk_assessment(&self, analysis: &FirmwareAnalysis) -> String {
        if analysis.is_nda_protected {
            "CRITICAL: NDA-protected firmware detected".to_string()
        } else if analysis.is_proprietary {
            "CRITICAL: Proprietary firmware detected".to_string()
        } else if analysis.is_apple_firmware {
            "HIGH: Apple firmware detected".to_string()
        } else {
            "SAFE: No proprietary firmware detected".to_string()
        }
    }

    pub fn generate_firmware_report(&self, analysis: &FirmwareAnalysis) -> String {
        let mut report = String::new();
        report.push_str("=== FIRMWARE SECURITY ANALYSIS ===\n\n");

        report.push_str(&format!("Apple Firmware: {}\n", analysis.is_apple_firmware));
        report.push_str(&format!("NDA Protected: {}\n", analysis.is_nda_protected));
        report.push_str(&format!("Proprietary: {}\n", analysis.is_proprietary));
        report.push_str(&format!("Risk Level: {:?}\n", analysis.risk_level));
        report.push_str(&format!("Safe to Distribute: {}\n\n", self.is_safe_to_distribute(analysis)));

        report.push_str(&format!("Assessment: {}\n", self.get_risk_assessment(analysis)));

        report
    }
}

impl Default for FirmwareSecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = FirmwareSecurityAnalyzer::new();
        assert!(!analyzer.apple_firmware_signatures.is_empty());
    }

    #[test]
    fn test_detect_apple_firmware() {
        let analyzer = FirmwareSecurityAnalyzer::new();
        let data = "Copyright Apple Inc. 2025";
        assert!(analyzer.detect_apple_firmware(data));
    }

    #[test]
    fn test_detect_nda_protection() {
        let analyzer = FirmwareSecurityAnalyzer::new();
        let data = "This is NDA protected material";
        assert!(analyzer.detect_nda_protection(data));
    }

    #[test]
    fn test_detect_proprietary() {
        let analyzer = FirmwareSecurityAnalyzer::new();
        let data = "Proprietary firmware code";
        assert!(analyzer.detect_proprietary(data));
    }

    #[test]
    fn test_analyze_firmware() {
        let analyzer = FirmwareSecurityAnalyzer::new();
        let analysis = analyzer.analyze_firmware("Normal firmware data");
        assert!(!analysis.is_apple_firmware);
        assert!(!analysis.is_nda_protected);
        assert!(!analysis.is_proprietary);
    }

    #[test]
    fn test_safe_to_distribute() {
        let analyzer = FirmwareSecurityAnalyzer::new();
        let safe_analysis = FirmwareAnalysis {
            is_proprietary: false,
            is_nda_protected: false,
            is_apple_firmware: false,
            risk_level: LegalRisk::None,
        };
        assert!(analyzer.is_safe_to_distribute(&safe_analysis));

        let unsafe_analysis = FirmwareAnalysis {
            is_proprietary: true,
            is_nda_protected: false,
            is_apple_firmware: false,
            risk_level: LegalRisk::Critical,
        };
        assert!(!analyzer.is_safe_to_distribute(&unsafe_analysis));
    }

    #[test]
    fn test_risk_assessment() {
        let analyzer = FirmwareSecurityAnalyzer::new();
        let analysis = FirmwareAnalysis {
            is_proprietary: false,
            is_nda_protected: true,
            is_apple_firmware: false,
            risk_level: LegalRisk::Critical,
        };
        let assessment = analyzer.get_risk_assessment(&analysis);
        assert!(assessment.contains("CRITICAL"));
    }
}
