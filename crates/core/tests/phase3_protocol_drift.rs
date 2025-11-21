use librepods_core::protocol_drift::*;
use librepods_core::protocol_comparator::ProtocolComparator;
use librepods_core::firmware_version_analyzer::FirmwareVersionAnalyzer;

#[test]
fn test_phase3_complete_protocol_drift_analysis() {
    let comparator = ProtocolComparator::new();

    let base_version = "1.0.0".to_string();
    let upstream_version = "1.0.0".to_string();

    let upstream_types = comparator.get_base_message_types().clone();
    let upstream_uuids = comparator.get_base_uuids().clone();
    let upstream_features = comparator.get_base_features().clone();

    let report = comparator.generate_drift_report(
        base_version,
        upstream_version,
        upstream_types,
        upstream_uuids,
        upstream_features,
    );

    assert!(!report.has_drift);
    assert_eq!(report.total_changes, 0);
    assert_eq!(report.drift_severity, DriftSeverity::None);
}

#[test]
fn test_protocol_drift_report_creation() {
    let report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());

    assert_eq!(report.base_version, "1.0.0");
    assert_eq!(report.upstream_version, "1.0.1");
    assert!(!report.has_drift);
    assert_eq!(report.drift_severity, DriftSeverity::None);
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
    assert_eq!(report.message_type_drifts.len(), 1);
}

#[test]
fn test_protocol_comparator_no_drift() {
    let comparator = ProtocolComparator::new();
    let base_types = comparator.get_base_message_types().clone();

    let drifts = comparator.compare_message_types(base_types);
    assert_eq!(drifts.len(), 0);
}

#[test]
fn test_protocol_comparator_added_type() {
    let comparator = ProtocolComparator::new();
    let mut upstream = comparator.get_base_message_types().clone();
    upstream.insert(0x20, "NewMessageType".to_string());

    let drifts = comparator.compare_message_types(upstream);
    assert!(drifts.iter().any(|d| d.status == DriftStatus::Added));
}

#[test]
fn test_protocol_comparator_removed_type() {
    let comparator = ProtocolComparator::new();
    let mut upstream = comparator.get_base_message_types().clone();
    upstream.remove(&0x01);

    let drifts = comparator.compare_message_types(upstream);
    assert!(drifts.iter().any(|d| d.status == DriftStatus::Removed && d.breaking_change));
}

#[test]
fn test_protocol_comparator_modified_type() {
    let comparator = ProtocolComparator::new();
    let mut upstream = comparator.get_base_message_types().clone();
    upstream.insert(0x01, "ModifiedBatteryStatus".to_string());

    let drifts = comparator.compare_message_types(upstream);
    assert!(drifts.iter().any(|d| d.status == DriftStatus::Modified));
}

#[test]
fn test_uuid_drift_detection() {
    let comparator = ProtocolComparator::new();
    let mut upstream = comparator.get_base_uuids().clone();
    upstream.insert("NEW-UUID-1234".to_string(), "NewService".to_string());

    let drifts = comparator.compare_uuids(upstream);
    assert!(drifts.iter().any(|d| d.status == DriftStatus::Added));
}

#[test]
fn test_feature_drift_detection() {
    let comparator = ProtocolComparator::new();
    let mut upstream = comparator.get_base_features().clone();
    upstream.insert("NewFeature".to_string());

    let drifts = comparator.compare_features(upstream);
    assert!(drifts.iter().any(|d| d.status == DriftStatus::Added));
}

#[test]
fn test_firmware_version_analyzer() {
    let analyzer = FirmwareVersionAnalyzer::new();

    let info = analyzer.get_version_info("5E135");
    assert!(info.is_some());
    assert_eq!(info.unwrap().device_model, "AirPodsProGen2");
}

#[test]
fn test_firmware_version_changes() {
    let analyzer = FirmwareVersionAnalyzer::new();

    let old_versions = vec!["5E135".to_string(), "5D134".to_string()];
    let new_versions = vec!["5E135".to_string(), "5C133".to_string()];

    let changes = analyzer.detect_version_changes(old_versions, new_versions);
    assert!(changes.len() > 0);
}

#[test]
fn test_firmware_feature_comparison() {
    let analyzer = FirmwareVersionAnalyzer::new();

    let (added, removed) = analyzer.compare_feature_sets("5D134", "5E135");
    assert_eq!(added.len(), 1);
    assert_eq!(removed.len(), 0);
}

#[test]
fn test_drift_severity_none() {
    let report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.0".to_string());
    assert_eq!(report.drift_severity, DriftSeverity::None);
}

#[test]
fn test_drift_severity_minor() {
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
    assert_eq!(report.drift_severity, DriftSeverity::Minor);
}

#[test]
fn test_drift_severity_moderate() {
    let mut report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());

    for i in 0..6 {
        let drift = MessageTypeDrift {
            opcode: 0x10 + i as u8,
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
fn test_drift_severity_major() {
    let mut report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());

    for i in 0..3 {
        let drift = MessageTypeDrift {
            opcode: 0x10 + i as u8,
            name: format!("Type{}", i),
            status: DriftStatus::Removed,
            previous_version: Some(format!("Type{}", i)),
            current_version: String::new(),
            breaking_change: true,
        };
        report.add_message_type_drift(drift);
    }

    assert_eq!(report.drift_severity, DriftSeverity::Major);
}

#[test]
fn test_drift_report_generation() {
    let report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());
    let report_text = report.generate_report();

    assert!(report_text.contains("PROTOCOL DRIFT ANALYSIS REPORT"));
    assert!(report_text.contains("1.0.0"));
    assert!(report_text.contains("1.0.1"));
}

#[test]
fn test_protocol_comparator_full_drift_report() {
    let comparator = ProtocolComparator::new();

    let mut upstream_types = comparator.get_base_message_types().clone();
    upstream_types.insert(0x20, "NewType".to_string());
    upstream_types.remove(&0x01);

    let upstream_uuids = comparator.get_base_uuids().clone();
    let upstream_features = comparator.get_base_features().clone();

    let report = comparator.generate_drift_report(
        "1.0.0".to_string(),
        "1.0.1".to_string(),
        upstream_types,
        upstream_uuids,
        upstream_features,
    );

    assert!(report.has_drift);
    assert!(report.breaking_changes > 0);
}

#[test]
fn test_firmware_version_report() {
    let analyzer = FirmwareVersionAnalyzer::new();
    let report = analyzer.generate_version_report();

    assert!(report.contains("FIRMWARE VERSION ANALYSIS"));
    assert!(report.contains("Known Versions"));
}

#[test]
fn test_firmware_versions_for_model() {
    let analyzer = FirmwareVersionAnalyzer::new();
    let versions = analyzer.get_versions_for_model("AirPodsProGen2");

    assert!(versions.len() > 0);
}

#[test]
fn test_latest_firmware_version() {
    let analyzer = FirmwareVersionAnalyzer::new();
    let latest = analyzer.get_latest_version();

    assert!(latest.is_some());
    assert_eq!(latest.unwrap().version, "5E135");
}

#[test]
fn test_extract_firmware_features() {
    let analyzer = FirmwareVersionAnalyzer::new();
    let features = analyzer.extract_features_from_version("5E135");

    assert!(features.len() > 0);
    assert!(features.contains(&"BatteryMonitoring".to_string()));
}

#[test]
fn test_drift_status_enum() {
    assert_eq!(DriftStatus::Added, DriftStatus::Added);
    assert_ne!(DriftStatus::Added, DriftStatus::Removed);
}

#[test]
fn test_complete_phase3_workflow() {
    let comparator = ProtocolComparator::new();
    let analyzer = FirmwareVersionAnalyzer::new();

    let base_types = comparator.get_base_message_types().clone();
    let base_uuids = comparator.get_base_uuids().clone();
    let base_features = comparator.get_base_features().clone();

    let mut upstream_types = base_types.clone();
    upstream_types.insert(0x20, "NewType".to_string());

    let report = comparator.generate_drift_report(
        "1.0.0".to_string(),
        "1.0.1".to_string(),
        upstream_types,
        base_uuids,
        base_features,
    );

    assert!(report.has_drift);

    let old_versions = vec!["5E135".to_string()];
    let new_versions = vec!["5E135".to_string(), "5C133".to_string()];

    let changes = analyzer.detect_version_changes(old_versions, new_versions);
    assert!(changes.len() > 0);
}

#[test]
fn test_protocol_drift_recommendations() {
    let mut report = ProtocolDriftReport::new("1.0.0".to_string(), "1.0.1".to_string());

    for i in 0..5 {
        let drift = MessageTypeDrift {
            opcode: 0x10 + i as u8,
            name: format!("Type{}", i),
            status: DriftStatus::Removed,
            previous_version: Some(format!("Type{}", i)),
            current_version: String::new(),
            breaking_change: true,
        };
        report.add_message_type_drift(drift);
    }

    report.add_recommendation("Test recommendation".to_string());
    assert!(report.recommendations.len() > 0);
}
