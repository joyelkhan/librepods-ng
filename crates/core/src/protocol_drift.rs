use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolElement {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub added_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTypeDrift {
    pub opcode: u8,
    pub name: String,
    pub status: DriftStatus,
    pub previous_version: Option<String>,
    pub current_version: String,
    pub breaking_change: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UUIDDrift {
    pub uuid: String,
    pub name: String,
    pub status: DriftStatus,
    pub device_models: Vec<String>,
    pub breaking_change: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDrift {
    pub feature_name: String,
    pub status: DriftStatus,
    pub affected_models: Vec<String>,
    pub breaking_change: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftStatus {
    Added,
    Modified,
    Deprecated,
    Removed,
    Unchanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDriftReport {
    pub scan_date: String,
    pub base_version: String,
    pub upstream_version: String,
    pub message_type_drifts: Vec<MessageTypeDrift>,
    pub uuid_drifts: Vec<UUIDDrift>,
    pub feature_drifts: Vec<FeatureDrift>,
    pub total_changes: usize,
    pub breaking_changes: usize,
    pub has_drift: bool,
    pub drift_severity: DriftSeverity,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftSeverity {
    None,
    Minor,
    Moderate,
    Major,
    Critical,
}

impl ProtocolDriftReport {
    pub fn new(base_version: String, upstream_version: String) -> Self {
        Self {
            scan_date: "2025-11-21T23:55:00Z".to_string(),
            base_version,
            upstream_version,
            message_type_drifts: Vec::new(),
            uuid_drifts: Vec::new(),
            feature_drifts: Vec::new(),
            total_changes: 0,
            breaking_changes: 0,
            has_drift: false,
            drift_severity: DriftSeverity::None,
            recommendations: Vec::new(),
        }
    }

    pub fn add_message_type_drift(&mut self, drift: MessageTypeDrift) {
        if drift.breaking_change {
            self.breaking_changes += 1;
        }
        self.total_changes += 1;
        self.message_type_drifts.push(drift);
        self.has_drift = true;
        self.update_severity();
    }

    pub fn add_uuid_drift(&mut self, drift: UUIDDrift) {
        if drift.breaking_change {
            self.breaking_changes += 1;
        }
        self.total_changes += 1;
        self.uuid_drifts.push(drift);
        self.has_drift = true;
        self.update_severity();
    }

    pub fn add_feature_drift(&mut self, drift: FeatureDrift) {
        if drift.breaking_change {
            self.breaking_changes += 1;
        }
        self.total_changes += 1;
        self.feature_drifts.push(drift);
        self.has_drift = true;
        self.update_severity();
    }

    fn update_severity(&mut self) {
        self.drift_severity = if self.breaking_changes > 3 {
            DriftSeverity::Critical
        } else if self.breaking_changes > 1 {
            DriftSeverity::Major
        } else if self.total_changes > 5 {
            DriftSeverity::Moderate
        } else if self.total_changes > 0 {
            DriftSeverity::Minor
        } else {
            DriftSeverity::None
        };
    }

    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== PROTOCOL DRIFT ANALYSIS REPORT ===\n\n");

        report.push_str(&format!("Base Version: {}\n", self.base_version));
        report.push_str(&format!("Upstream Version: {}\n", self.upstream_version));
        report.push_str(&format!("Scan Date: {}\n\n", self.scan_date));

        report.push_str(&format!("Total Changes: {}\n", self.total_changes));
        report.push_str(&format!("Breaking Changes: {}\n", self.breaking_changes));
        report.push_str(&format!("Has Drift: {}\n", self.has_drift));
        report.push_str(&format!("Drift Severity: {:?}\n\n", self.drift_severity));

        report.push_str(&format!("Message Type Drifts: {}\n", self.message_type_drifts.len()));
        for drift in &self.message_type_drifts {
            report.push_str(&format!("  0x{:02X} {} ({:?})\n", drift.opcode, drift.name, drift.status));
        }

        report.push_str(&format!("\nUUID Drifts: {}\n", self.uuid_drifts.len()));
        for drift in &self.uuid_drifts {
            report.push_str(&format!("  {} ({:?})\n", drift.name, drift.status));
        }

        report.push_str(&format!("\nFeature Drifts: {}\n", self.feature_drifts.len()));
        for drift in &self.feature_drifts {
            report.push_str(&format!("  {} ({:?})\n", drift.feature_name, drift.status));
        }

        report.push_str(&format!("\nRecommendations: {}\n", self.recommendations.len()));
        for rec in &self.recommendations {
            report.push_str(&format!("  - {}\n", rec));
        }

        report
    }
}

impl Default for ProtocolDriftReport {
    fn default() -> Self {
        Self::new("1.0.0".to_string(), "1.0.0".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_creation() {
        let report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());
        assert_eq!(report.base_version, "1.0.0");
        assert_eq!(report.upstream_version, "1.0.1");
        assert!(!report.has_drift);
    }

    #[test]
    fn test_add_message_type_drift() {
        let mut report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());
        let drift = MessageTypeDrift {
            opcode: 0x10,
            name: "NewType".to_string(),
            status: DriftStatus::Added,
            previous_version: None,
            current_version: "1.0.1".to_string(),
            breaking_change: false,
        };
        report.add_message_type_drift(drift);
        assert!(report.has_drift);
        assert_eq!(report.total_changes, 1);
    }

    #[test]
    fn test_severity_calculation() {
        let mut report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());
        for i in 0..5 {
            let drift = MessageTypeDrift {
                opcode: i,
                name: format!("Type{}", i),
                status: DriftStatus::Added,
                previous_version: None,
                current_version: "1.0.1".to_string(),
                breaking_change: false,
            };
            report.add_message_type_drift(drift);
        }
        assert_eq!(report.drift_severity, DriftSeverity::Moderate);
    }

    #[test]
    fn test_breaking_changes() {
        let mut report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());
        let drift = MessageTypeDrift {
            opcode: 0x10,
            name: "RemovedType".to_string(),
            status: DriftStatus::Removed,
            previous_version: Some("1.0.0".to_string()),
            current_version: "1.0.1".to_string(),
            breaking_change: true,
        };
        report.add_message_type_drift(drift);
        assert_eq!(report.breaking_changes, 1);
    }
}
