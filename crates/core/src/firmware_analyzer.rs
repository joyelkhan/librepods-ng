use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FirmwareAnalyzer {
    known_versions: HashMap<String, FirmwareInfo>,
    device_models: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FirmwareInfo {
    pub version: String,
    pub device_model: String,
    pub release_date: String,
    pub features: Vec<String>,
    pub build_number: String,
}

impl FirmwareAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            known_versions: HashMap::new(),
            device_models: vec![
                "AirPods2".to_string(),
                "AirPods3".to_string(),
                "AirPods4".to_string(),
                "AirPodsProGen1".to_string(),
                "AirPodsProGen2".to_string(),
                "AirPodsProGen3".to_string(),
                "AirPodsMax".to_string(),
                "BeatsFitPro".to_string(),
            ],
        };
        analyzer.initialize_known_versions();
        analyzer
    }

    fn initialize_known_versions(&mut self) {
        let versions = vec![
            ("5E135", "AirPodsProGen2", "2025-11-21", vec!["BatteryMonitoring", "ANC", "SpatialAudio"]),
            ("5D134", "AirPodsProGen2", "2025-10-15", vec!["BatteryMonitoring", "ANC"]),
            ("5C133", "AirPodsMax", "2025-09-20", vec!["BatteryMonitoring", "ANC", "AdaptiveTransparency"]),
            ("5B132", "AirPods4", "2025-08-10", vec!["BatteryMonitoring", "ANC"]),
            ("5A131", "AirPods3", "2025-07-05", vec!["BatteryMonitoring", "ANC"]),
        ];

        for (version, model, date, features) in versions {
            self.known_versions.insert(
                version.to_string(),
                FirmwareInfo {
                    version: version.to_string(),
                    device_model: model.to_string(),
                    release_date: date.to_string(),
                    features: features.iter().map(|s| s.to_string()).collect(),
                    build_number: format!("BUILD_{}", version),
                },
            );
        }
    }

    pub fn get_firmware_info(&self, version: &str) -> Option<&FirmwareInfo> {
        self.known_versions.get(version)
    }

    pub fn add_firmware_version(&mut self, info: FirmwareInfo) {
        self.known_versions.insert(info.version.clone(), info);
    }

    pub fn is_known_version(&self, version: &str) -> bool {
        self.known_versions.contains_key(version)
    }

    pub fn get_latest_version(&self) -> Option<&FirmwareInfo> {
        self.known_versions.values().max_by_key(|f| f.release_date.as_str())
    }

    pub fn get_versions_for_model(&self, model: &str) -> Vec<&FirmwareInfo> {
        self.known_versions
            .values()
            .filter(|f| f.device_model == model)
            .collect()
    }

    pub fn detect_version_changes(&self, old_versions: Vec<String>, new_versions: Vec<String>) -> (Vec<String>, Vec<String>) {
        let old_set: std::collections::HashSet<_> = old_versions.into_iter().collect();
        let new_set: std::collections::HashSet<_> = new_versions.into_iter().collect();

        let added: Vec<String> = new_set.difference(&old_set).cloned().collect();
        let removed: Vec<String> = old_set.difference(&new_set).cloned().collect();

        (added, removed)
    }

    pub fn extract_features_from_version(&self, version: &str) -> Vec<String> {
        self.get_firmware_info(version)
            .map(|info| info.features.clone())
            .unwrap_or_default()
    }

    pub fn is_valid_device_model(&self, model: &str) -> bool {
        self.device_models.contains(&model.to_string())
    }

    pub fn get_all_device_models(&self) -> Vec<String> {
        self.device_models.clone()
    }

    pub fn get_all_known_versions(&self) -> Vec<String> {
        self.known_versions.keys().cloned().collect()
    }

    pub fn generate_firmware_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== FIRMWARE ANALYSIS REPORT ===\n\n");

        report.push_str(&format!("Known Versions: {}\n", self.known_versions.len()));
        report.push_str(&format!("Device Models: {}\n\n", self.device_models.len()));

        if let Some(latest) = self.get_latest_version() {
            report.push_str(&format!("Latest Version: {}\n", latest.version));
            report.push_str(&format!("Device: {}\n", latest.device_model));
            report.push_str(&format!("Release Date: {}\n", latest.release_date));
            report.push_str(&format!("Features: {}\n\n", latest.features.len()));
            for feature in &latest.features {
                report.push_str(&format!("  - {}\n", feature));
            }
        }

        report.push_str("\nAll Known Versions:\n");
        let mut versions: Vec<_> = self.known_versions.values().collect();
        versions.sort_by(|a, b| b.release_date.cmp(&a.release_date));

        for fw in versions {
            report.push_str(&format!("  {} ({}) - {}\n", fw.version, fw.device_model, fw.release_date));
        }

        report
    }
}

impl Default for FirmwareAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = FirmwareAnalyzer::new();
        assert_eq!(analyzer.known_versions.len(), 5);
        assert_eq!(analyzer.device_models.len(), 8);
    }

    #[test]
    fn test_get_firmware_info() {
        let analyzer = FirmwareAnalyzer::new();
        let info = analyzer.get_firmware_info("5E135");
        assert!(info.is_some());
        assert_eq!(info.unwrap().device_model, "AirPodsProGen2");
    }

    #[test]
    fn test_is_known_version() {
        let analyzer = FirmwareAnalyzer::new();
        assert!(analyzer.is_known_version("5E135"));
        assert!(!analyzer.is_known_version("UNKNOWN"));
    }

    #[test]
    fn test_get_latest_version() {
        let analyzer = FirmwareAnalyzer::new();
        let latest = analyzer.get_latest_version();
        assert!(latest.is_some());
    }

    #[test]
    fn test_get_versions_for_model() {
        let analyzer = FirmwareAnalyzer::new();
        let versions = analyzer.get_versions_for_model("AirPodsProGen2");
        assert!(versions.len() > 0);
    }

    #[test]
    fn test_detect_version_changes() {
        let analyzer = FirmwareAnalyzer::new();
        let old = vec!["5E135".to_string(), "5D134".to_string()];
        let new = vec!["5E135".to_string(), "5C133".to_string()];
        let (added, removed) = analyzer.detect_version_changes(old, new);
        assert_eq!(added.len(), 1);
        assert_eq!(removed.len(), 1);
    }

    #[test]
    fn test_is_valid_device_model() {
        let analyzer = FirmwareAnalyzer::new();
        assert!(analyzer.is_valid_device_model("AirPods2"));
        assert!(!analyzer.is_valid_device_model("UnknownModel"));
    }

    #[test]
    fn test_extract_features() {
        let analyzer = FirmwareAnalyzer::new();
        let features = analyzer.extract_features_from_version("5E135");
        assert!(features.len() > 0);
    }
}
