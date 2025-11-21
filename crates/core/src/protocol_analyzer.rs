use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProtocolAnalyzer {
    known_message_types: HashMap<u8, String>,
    known_uuids: HashMap<String, String>,
    known_features: HashMap<String, bool>,
}

impl ProtocolAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            known_message_types: HashMap::new(),
            known_uuids: HashMap::new(),
            known_features: HashMap::new(),
        };
        analyzer.initialize_known_types();
        analyzer
    }

    fn initialize_known_types(&mut self) {
        self.known_message_types.insert(0x01, "BatteryStatus".to_string());
        self.known_message_types.insert(0x02, "AncControl".to_string());
        self.known_message_types.insert(0x03, "EarDetection".to_string());
        self.known_message_types.insert(0x04, "FirmwareInfo".to_string());
        self.known_message_types.insert(0x05, "SpatialAudio".to_string());
        self.known_message_types.insert(0x06, "HeartRate".to_string());
        self.known_message_types.insert(0x07, "FindMy".to_string());
        self.known_message_types.insert(0x08, "ConversationAwareness".to_string());
        self.known_message_types.insert(0x09, "HearingAid".to_string());
        self.known_message_types.insert(0x0A, "DeviceRename".to_string());
        self.known_message_types.insert(0x0B, "MultipointControl".to_string());
        self.known_message_types.insert(0x0C, "AdaptiveTransparency".to_string());
        self.known_message_types.insert(0x0D, "LongPressActions".to_string());
        self.known_message_types.insert(0x0E, "CustomTransparency".to_string());
        self.known_message_types.insert(0x0F, "HeadGestures".to_string());

        self.known_uuids.insert(
            "7DFC9000-7D1C-4951-86AA-8D9728F8D66C".to_string(),
            "AAP_SERVICE".to_string(),
        );
        self.known_uuids.insert(
            "7DFC9001-7D1C-4951-86AA-8D9728F8D66C".to_string(),
            "AAP_CHARACTERISTIC".to_string(),
        );
        self.known_uuids.insert("180F".to_string(), "BATTERY_SERVICE".to_string());
        self.known_uuids.insert("180A".to_string(), "DEVICE_INFO_SERVICE".to_string());

        self.known_features.insert("BatteryMonitoring".to_string(), true);
        self.known_features.insert("NoiseControl".to_string(), true);
        self.known_features.insert("AdaptiveTransparency".to_string(), true);
        self.known_features.insert("EarDetection".to_string(), true);
        self.known_features.insert("ConversationAwareness".to_string(), true);
        self.known_features.insert("HeadGestures".to_string(), true);
        self.known_features.insert("HearingAid".to_string(), true);
        self.known_features.insert("CustomTransparency".to_string(), true);
        self.known_features.insert("DeviceRename".to_string(), true);
        self.known_features.insert("LongPressActions".to_string(), true);
        self.known_features.insert("Multipoint".to_string(), true);
        self.known_features.insert("FirmwareInfo".to_string(), true);
        self.known_features.insert("FindMy".to_string(), true);
        self.known_features.insert("HeartRate".to_string(), true);
        self.known_features.insert("SpatialAudio".to_string(), true);
    }

    pub fn analyze_message_types(&self, incoming_types: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        let mut known = Vec::new();
        let mut unknown = Vec::new();

        for msg_type in incoming_types {
            if self.known_message_types.contains_key(&msg_type) {
                known.push(msg_type);
            } else {
                unknown.push(msg_type);
            }
        }

        (known, unknown)
    }

    pub fn analyze_uuids(&self, incoming_uuids: Vec<String>) -> (Vec<String>, Vec<String>) {
        let mut known = Vec::new();
        let mut unknown = Vec::new();

        for uuid in incoming_uuids {
            if self.known_uuids.contains_key(&uuid) {
                known.push(uuid);
            } else {
                unknown.push(uuid);
            }
        }

        (known, unknown)
    }

    pub fn analyze_features(&self, incoming_features: Vec<String>) -> (Vec<String>, Vec<String>) {
        let mut known = Vec::new();
        let mut unknown = Vec::new();

        for feature in incoming_features {
            if self.known_features.contains_key(&feature) {
                known.push(feature);
            } else {
                unknown.push(feature);
            }
        }

        (known, unknown)
    }

    pub fn get_message_type_name(&self, msg_type: u8) -> Option<String> {
        self.known_message_types.get(&msg_type).cloned()
    }

    pub fn get_uuid_name(&self, uuid: &str) -> Option<String> {
        self.known_uuids.get(uuid).cloned()
    }

    pub fn is_known_message_type(&self, msg_type: u8) -> bool {
        self.known_message_types.contains_key(&msg_type)
    }

    pub fn is_known_uuid(&self, uuid: &str) -> bool {
        self.known_uuids.contains_key(uuid)
    }

    pub fn is_known_feature(&self, feature: &str) -> bool {
        self.known_features.contains_key(feature)
    }

    pub fn get_all_message_types(&self) -> Vec<(u8, String)> {
        self.known_message_types
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }

    pub fn get_all_uuids(&self) -> Vec<(String, String)> {
        self.known_uuids
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn get_all_features(&self) -> Vec<String> {
        self.known_features.keys().cloned().collect()
    }

    pub fn detect_drift(&self, incoming_types: Vec<u8>, incoming_uuids: Vec<String>) -> bool {
        let (_, unknown_types) = self.analyze_message_types(incoming_types);
        let (_, unknown_uuids) = self.analyze_uuids(incoming_uuids);

        !unknown_types.is_empty() || !unknown_uuids.is_empty()
    }

    pub fn generate_analysis_report(&self, incoming_types: Vec<u8>, incoming_uuids: Vec<String>) -> String {
        let (known_types, unknown_types) = self.analyze_message_types(incoming_types);
        let (known_uuids, unknown_uuids) = self.analyze_uuids(incoming_uuids);

        let mut report = String::new();
        report.push_str("=== PROTOCOL ANALYSIS REPORT ===\n\n");

        report.push_str(&format!("Message Types:\n"));
        report.push_str(&format!("  Known: {}\n", known_types.len()));
        report.push_str(&format!("  Unknown: {}\n\n", unknown_types.len()));

        if !unknown_types.is_empty() {
            report.push_str("  New Message Types:\n");
            for msg_type in &unknown_types {
                report.push_str(&format!("    - 0x{:02X}\n", msg_type));
            }
            report.push_str("\n");
        }

        report.push_str(&format!("UUIDs:\n"));
        report.push_str(&format!("  Known: {}\n", known_uuids.len()));
        report.push_str(&format!("  Unknown: {}\n\n", unknown_uuids.len()));

        if !unknown_uuids.is_empty() {
            report.push_str("  New UUIDs:\n");
            for uuid in &unknown_uuids {
                report.push_str(&format!("    - {}\n", uuid));
            }
            report.push_str("\n");
        }

        report.push_str(&format!("Protocol Drift: {}\n", self.detect_drift(known_types.clone(), known_uuids)));

        report
    }
}

impl Default for ProtocolAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = ProtocolAnalyzer::new();
        assert_eq!(analyzer.known_message_types.len(), 15);
        assert_eq!(analyzer.known_uuids.len(), 4);
        assert_eq!(analyzer.known_features.len(), 15);
    }

    #[test]
    fn test_analyze_known_message_types() {
        let analyzer = ProtocolAnalyzer::new();
        let types = vec![0x01, 0x02, 0x03];
        let (known, unknown) = analyzer.analyze_message_types(types);
        assert_eq!(known.len(), 3);
        assert_eq!(unknown.len(), 0);
    }

    #[test]
    fn test_analyze_unknown_message_types() {
        let analyzer = ProtocolAnalyzer::new();
        let types = vec![0x01, 0xFF, 0xFE];
        let (known, unknown) = analyzer.analyze_message_types(types);
        assert_eq!(known.len(), 1);
        assert_eq!(unknown.len(), 2);
    }

    #[test]
    fn test_get_message_type_name() {
        let analyzer = ProtocolAnalyzer::new();
        let name = analyzer.get_message_type_name(0x01);
        assert_eq!(name, Some("BatteryStatus".to_string()));
    }

    #[test]
    fn test_is_known_message_type() {
        let analyzer = ProtocolAnalyzer::new();
        assert!(analyzer.is_known_message_type(0x01));
        assert!(!analyzer.is_known_message_type(0xFF));
    }

    #[test]
    fn test_detect_drift() {
        let analyzer = ProtocolAnalyzer::new();
        let types = vec![0x01, 0x02];
        let uuids = vec!["7DFC9000-7D1C-4951-86AA-8D9728F8D66C".to_string()];
        assert!(!analyzer.detect_drift(types, uuids));

        let types_with_unknown = vec![0x01, 0xFF];
        let uuids_with_unknown = vec!["UNKNOWN-UUID".to_string()];
        assert!(analyzer.detect_drift(types_with_unknown, uuids_with_unknown));
    }
}
