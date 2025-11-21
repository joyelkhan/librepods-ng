use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareVersionInfo {
    pub version: String,
    pub device_model: String,
    pub release_date: String,
    pub features: Vec<String>,
    pub build_number: String,
    pub protocol_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionChange {
    pub old_version: String,
    pub new_version: String,
    pub device_model: String,
    pub change_type: VersionChangeType,
    pub is_breaking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionChangeType {
    Major,
    Minor,
    Patch,
    Unknown,
}

pub struct FirmwareVersionAnalyzer {
    known_versions: HashMap<String, FirmwareVersionInfo>,
    version_history: Vec<(String, String)>,
}

impl FirmwareVersionAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            known_versions: HashMap::new(),
            version_history: Vec::new(),
        };
        analyzer.initialize_versions();
        analyzer
    }

    fn initialize_versions(&mut self) {
        let versions = vec![
            ("5E135", "AirPodsProGen2", "2025-11-21", vec!["BatteryMonitoring", "ANC", "SpatialAudio"], "5E135", "1.0.0"),
            ("5D134", "AirPodsProGen2", "2025-10-15", vec!["BatteryMonitoring", "ANC"], "5D134", "1.0.0"),
            ("5C133", "AirPodsMax", "2025-09-20", vec!["BatteryMonitoring", "ANC", "AdaptiveTransparency"], "5C133", "1.0.0"),
            ("5B132", "AirPods4", "2025-08-10", vec!["BatteryMonitoring", "ANC"], "5B132", "1.0.0"),
            ("5A131", "AirPods3", "2025-07-05", vec!["BatteryMonitoring", "ANC"], "5A131", "1.0.0"),
        ];

        for (version, model, date, features, build, proto_version) in versions {
            self.known_versions.insert(
                version.to_string(),
                FirmwareVersionInfo {
                    version: version.to_string(),
                    device_model: model.to_string(),
                    release_date: date.to_string(),
                    features: features.iter().map(|s| s.to_string()).collect(),
                    build_number: build.to_string(),
                    protocol_version: proto_version.to_string(),
                },
            );
        }
    }

    pub fn get_version_info(&self, version: &str) -> Option<&FirmwareVersionInfo> {
        self.known_versions.get(version)
    }

    pub fn add_version(&mut self, info: FirmwareVersionInfo) {
        self.known_versions.insert(info.version.clone(), info);
    }

    pub fn detect_version_changes(
        &self,
        old_versions: Vec<String>,
        new_versions: Vec<String>,
    ) -> Vec<VersionChange> {
        let mut changes = Vec::new();

        for new_version in &new_versions {
            if !old_versions.contains(new_version) {
                if let Some(info) = self.get_version_info(new_version) {
                    let change_type = self.determine_change_type(new_version);
                    changes.push(VersionChange {
                        old_version: "N/A".to_string(),
                        new_version: new_version.clone(),
                        device_model: info.device_model.clone(),
                        change_type,
                        is_breaking: false,
                    });
                }
            }
        }

        for old_version in &old_versions {
            if !new_versions.contains(old_version) {
                if let Some(info) = self.get_version_info(old_version) {
                    changes.push(VersionChange {
                        old_version: old_version.clone(),
                        new_version: "N/A".to_string(),
                        device_model: info.device_model.clone(),
                        change_type: VersionChangeType::Unknown,
                        is_breaking: true,
                    });
                }
            }
        }

        changes
    }

    fn determine_change_type(&self, version: &str) -> VersionChangeType {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() >= 2 {
            if let (Ok(major), Ok(minor)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                if major > 0 {
                    return VersionChangeType::Major;
                } else if minor > 0 {
                    return VersionChangeType::Minor;
                } else {
                    return VersionChangeType::Patch;
                }
            }
        }
        VersionChangeType::Unknown
    }

    pub fn get_versions_for_model(&self, model: &str) -> Vec<&FirmwareVersionInfo> {
        self.known_versions
            .values()
            .filter(|v| v.device_model == model)
            .collect()
    }

    pub fn get_latest_version(&self) -> Option<&FirmwareVersionInfo> {
        self.known_versions.values().max_by_key(|v| v.release_date.as_str())
    }

    pub fn extract_features_from_version(&self, version: &str) -> Vec<String> {
        self.get_version_info(version)
            .map(|info| info.features.clone())
            .unwrap_or_default()
    }

    pub fn compare_feature_sets(
        &self,
        old_version: &str,
        new_version: &str,
    ) -> (Vec<String>, Vec<String>) {
        let old_features = self.extract_features_from_version(old_version);
        let new_features = self.extract_features_from_version(new_version);

        let added: Vec<String> = new_features
            .iter()
            .filter(|f| !old_features.contains(f))
            .cloned()
            .collect();

        let removed: Vec<String> = old_features
            .iter()
            .filter(|f| !new_features.contains(f))
            .cloned()
            .collect();

        (added, removed)
    }

    pub fn generate_version_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== FIRMWARE VERSION ANALYSIS ===\n\n");

        report.push_str(&format!("Known Versions: {}\n", self.known_versions.len()));

        if let Some(latest) = self.get_latest_version() {
            report.push_str(&format!("\nLatest Version: {}\n", latest.version));
            report.push_str(&format!("Device: {}\n", latest.device_model));
            report.push_str(&format!("Release Date: {}\n", latest.release_date));
            report.push_str(&format!("Protocol Version: {}\n", latest.protocol_version));
            report.push_str(&format!("Features: {}\n", latest.features.len()));
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

impl Default for FirmwareVersionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = FirmwareVersionAnalyzer::new();
        assert!(analyzer.known_versions.len() > 0);
    }

    #[test]
    fn test_get_version_info() {
        let analyzer = FirmwareVersionAnalyzer::new();
        let info = analyzer.get_version_info("5E135");
        assert!(info.is_some());
    }

    #[test]
    fn test_detect_version_changes() {
        let analyzer = FirmwareVersionAnalyzer::new();
        let old = vec!["5E135".to_string(), "5D134".to_string()];
        let new = vec!["5E135".to_string(), "5C133".to_string()];
        let changes = analyzer.detect_version_changes(old, new);
        assert!(changes.len() > 0);
    }

    #[test]
    fn test_get_versions_for_model() {
        let analyzer = FirmwareVersionAnalyzer::new();
        let versions = analyzer.get_versions_for_model("AirPodsProGen2");
        assert!(versions.len() > 0);
    }

    #[test]
    fn test_compare_feature_sets() {
        let analyzer = FirmwareVersionAnalyzer::new();
        let (added, removed) = analyzer.compare_feature_sets("5D134", "5E135");
        assert_eq!(added.len(), 1);
        assert_eq!(removed.len(), 0);
    }

    #[test]
    fn test_extract_features() {
        let analyzer = FirmwareVersionAnalyzer::new();
        let features = analyzer.extract_features_from_version("5E135");
        assert!(features.len() > 0);
    }
}
