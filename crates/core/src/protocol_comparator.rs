use crate::protocol_drift::{DriftStatus, FeatureDrift, MessageTypeDrift, ProtocolDriftReport, UUIDDrift};
use std::collections::{HashMap, HashSet};

pub struct ProtocolComparator {
    base_message_types: HashMap<u8, String>,
    base_uuids: HashMap<String, String>,
    base_features: HashSet<String>,
}

impl ProtocolComparator {
    pub fn new() -> Self {
        let mut comparator = Self {
            base_message_types: HashMap::new(),
            base_uuids: HashMap::new(),
            base_features: HashSet::new(),
        };
        comparator.initialize_base_protocol();
        comparator
    }

    fn initialize_base_protocol(&mut self) {
        self.base_message_types.insert(0x01, "BatteryStatus".to_string());
        self.base_message_types.insert(0x02, "AncControl".to_string());
        self.base_message_types.insert(0x03, "EarDetection".to_string());
        self.base_message_types.insert(0x04, "FirmwareInfo".to_string());
        self.base_message_types.insert(0x05, "SpatialAudio".to_string());
        self.base_message_types.insert(0x06, "HeartRate".to_string());
        self.base_message_types.insert(0x07, "FindMy".to_string());
        self.base_message_types.insert(0x08, "ConversationAwareness".to_string());
        self.base_message_types.insert(0x09, "HearingAid".to_string());
        self.base_message_types.insert(0x0A, "DeviceRename".to_string());
        self.base_message_types.insert(0x0B, "MultipointControl".to_string());
        self.base_message_types.insert(0x0C, "AdaptiveTransparency".to_string());
        self.base_message_types.insert(0x0D, "LongPressActions".to_string());
        self.base_message_types.insert(0x0E, "CustomTransparency".to_string());
        self.base_message_types.insert(0x0F, "HeadGestures".to_string());

        self.base_uuids.insert(
            "7DFC9000-7D1C-4951-86AA-8D9728F8D66C".to_string(),
            "AAP_SERVICE".to_string(),
        );
        self.base_uuids.insert(
            "7DFC9001-7D1C-4951-86AA-8D9728F8D66C".to_string(),
            "AAP_CHARACTERISTIC".to_string(),
        );
        self.base_uuids.insert("180F".to_string(), "BATTERY_SERVICE".to_string());
        self.base_uuids.insert("180A".to_string(), "DEVICE_INFO_SERVICE".to_string());

        self.base_features.insert("BatteryMonitoring".to_string());
        self.base_features.insert("NoiseControl".to_string());
        self.base_features.insert("AdaptiveTransparency".to_string());
        self.base_features.insert("EarDetection".to_string());
        self.base_features.insert("ConversationAwareness".to_string());
        self.base_features.insert("HeadGestures".to_string());
        self.base_features.insert("HearingAid".to_string());
        self.base_features.insert("CustomTransparency".to_string());
        self.base_features.insert("DeviceRename".to_string());
        self.base_features.insert("LongPressActions".to_string());
        self.base_features.insert("Multipoint".to_string());
        self.base_features.insert("FirmwareInfo".to_string());
        self.base_features.insert("FindMy".to_string());
        self.base_features.insert("HeartRate".to_string());
        self.base_features.insert("SpatialAudio".to_string());
    }

    pub fn compare_message_types(
        &self,
        upstream_types: HashMap<u8, String>,
    ) -> Vec<MessageTypeDrift> {
        let mut drifts = Vec::new();

        for (opcode, name) in &upstream_types {
            if let Some(base_name) = self.base_message_types.get(opcode) {
                if base_name != name {
                    drifts.push(MessageTypeDrift {
                        opcode: *opcode,
                        name: name.clone(),
                        status: DriftStatus::Modified,
                        previous_version: Some(base_name.clone()),
                        current_version: name.clone(),
                        breaking_change: true,
                    });
                }
            } else {
                drifts.push(MessageTypeDrift {
                    opcode: *opcode,
                    name: name.clone(),
                    status: DriftStatus::Added,
                    previous_version: None,
                    current_version: name.clone(),
                    breaking_change: false,
                });
            }
        }

        for opcode in self.base_message_types.keys() {
            if !upstream_types.contains_key(opcode) {
                drifts.push(MessageTypeDrift {
                    opcode: *opcode,
                    name: self.base_message_types[opcode].clone(),
                    status: DriftStatus::Removed,
                    previous_version: Some(self.base_message_types[opcode].clone()),
                    current_version: String::new(),
                    breaking_change: true,
                });
            }
        }

        drifts
    }

    pub fn compare_uuids(&self, upstream_uuids: HashMap<String, String>) -> Vec<UUIDDrift> {
        let mut drifts = Vec::new();

        for (uuid, name) in &upstream_uuids {
            if !self.base_uuids.contains_key(uuid) {
                drifts.push(UUIDDrift {
                    uuid: uuid.clone(),
                    name: name.clone(),
                    status: DriftStatus::Added,
                    device_models: vec!["All".to_string()],
                    breaking_change: false,
                });
            }
        }

        for uuid in self.base_uuids.keys() {
            if !upstream_uuids.contains_key(uuid) {
                drifts.push(UUIDDrift {
                    uuid: uuid.clone(),
                    name: self.base_uuids[uuid].clone(),
                    status: DriftStatus::Removed,
                    device_models: vec!["All".to_string()],
                    breaking_change: true,
                });
            }
        }

        drifts
    }

    pub fn compare_features(&self, upstream_features: HashSet<String>) -> Vec<FeatureDrift> {
        let mut drifts = Vec::new();

        for feature in &upstream_features {
            if !self.base_features.contains(feature) {
                drifts.push(FeatureDrift {
                    feature_name: feature.clone(),
                    status: DriftStatus::Added,
                    affected_models: vec!["All".to_string()],
                    breaking_change: false,
                });
            }
        }

        for feature in &self.base_features {
            if !upstream_features.contains(feature) {
                drifts.push(FeatureDrift {
                    feature_name: feature.clone(),
                    status: DriftStatus::Removed,
                    affected_models: vec!["All".to_string()],
                    breaking_change: true,
                });
            }
        }

        drifts
    }

    pub fn generate_drift_report(
        &self,
        base_version: String,
        upstream_version: String,
        upstream_types: HashMap<u8, String>,
        upstream_uuids: HashMap<String, String>,
        upstream_features: HashSet<String>,
    ) -> ProtocolDriftReport {
        let mut report = ProtocolDriftReport::new(base_version, upstream_version);

        let message_drifts = self.compare_message_types(upstream_types);
        for drift in message_drifts {
            report.add_message_type_drift(drift);
        }

        let uuid_drifts = self.compare_uuids(upstream_uuids);
        for drift in uuid_drifts {
            report.add_uuid_drift(drift);
        }

        let feature_drifts = self.compare_features(upstream_features);
        for drift in feature_drifts {
            report.add_feature_drift(drift);
        }

        if report.breaking_changes > 0 {
            report.add_recommendation("Review breaking changes before upgrading".to_string());
        }

        if report.total_changes > 5 {
            report.add_recommendation("Significant protocol changes detected".to_string());
        }

        report
    }

    pub fn detect_protocol_drift(&self, upstream_types: HashMap<u8, String>) -> bool {
        self.compare_message_types(upstream_types).len() > 0
    }

    pub fn get_base_message_types(&self) -> &HashMap<u8, String> {
        &self.base_message_types
    }

    pub fn get_base_uuids(&self) -> &HashMap<String, String> {
        &self.base_uuids
    }

    pub fn get_base_features(&self) -> &HashSet<String> {
        &self.base_features
    }
}

impl Default for ProtocolComparator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparator_creation() {
        let comparator = ProtocolComparator::new();
        assert_eq!(comparator.base_message_types.len(), 15);
        assert_eq!(comparator.base_uuids.len(), 4);
        assert_eq!(comparator.base_features.len(), 15);
    }

    #[test]
    fn test_no_drift() {
        let comparator = ProtocolComparator::new();
        let upstream = comparator.base_message_types.clone();
        let drifts = comparator.compare_message_types(upstream);
        assert_eq!(drifts.len(), 0);
    }

    #[test]
    fn test_added_message_type() {
        let comparator = ProtocolComparator::new();
        let mut upstream = comparator.base_message_types.clone();
        upstream.insert(0x20, "NewType".to_string());
        let drifts = comparator.compare_message_types(upstream);
        assert!(drifts.iter().any(|d| d.status == DriftStatus::Added));
    }

    #[test]
    fn test_removed_message_type() {
        let comparator = ProtocolComparator::new();
        let mut upstream = comparator.base_message_types.clone();
        upstream.remove(&0x01);
        let drifts = comparator.compare_message_types(upstream);
        assert!(drifts.iter().any(|d| d.status == DriftStatus::Removed));
    }

    #[test]
    fn test_uuid_drift() {
        let comparator = ProtocolComparator::new();
        let mut upstream = comparator.base_uuids.clone();
        upstream.insert("NEW-UUID".to_string(), "NewService".to_string());
        let drifts = comparator.compare_uuids(upstream);
        assert!(drifts.len() > 0);
    }

    #[test]
    fn test_feature_drift() {
        let comparator = ProtocolComparator::new();
        let mut upstream = comparator.base_features.clone();
        upstream.insert("NewFeature".to_string());
        let drifts = comparator.compare_features(upstream);
        assert!(drifts.len() > 0);
    }
}
