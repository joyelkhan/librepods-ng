use librepods_core::ingestion::DataIngestionEngine;
use librepods_core::protocol_analyzer::ProtocolAnalyzer;
use librepods_core::firmware_analyzer::FirmwareAnalyzer;
use librepods_core::upstream::{Commit, Release, ReleaseAsset, Tag, ProtocolDefinition, FirmwareVersion};
use std::collections::HashMap;

#[test]
fn test_phase1_complete_ingestion_workflow() {
    let mut engine = DataIngestionEngine::new();

    engine
        .ingest_repository_metadata(
            "librepods".to_string(),
            "kavishdevar/librepods".to_string(),
            "kavishdevar".to_string(),
            "https://github.com/kavishdevar/librepods".to_string(),
            "AirPods liberated from Apple's ecosystem".to_string(),
            14382,
            434,
            52,
            "Kotlin".to_string(),
            "GPL-3.0".to_string(),
            "2025-11-21T20:25:45Z".to_string(),
            "2025-11-21T18:18:06Z".to_string(),
        )
        .expect("Failed to ingest repository metadata");

    let commits = vec![
        Commit {
            sha: "a06c6734005d52dd02b29ae21c8b4de1b1b19e30".to_string(),
            message: "android(i18n): add pt translation (#297)".to_string(),
            author: "Gabriel Oliveira".to_string(),
            date: "2025-11-21T18:18:06Z".to_string(),
            url: "https://github.com/kavishdevar/librepods/commit/a06c6734005d52dd02b29ae21c8b4de1b1b19e30".to_string(),
        },
        Commit {
            sha: "a80680ff7383cfe00f8f88f09e59dc1a9cdd66fc".to_string(),
            message: "docs: fix broken link to hearing aid gist (#304)".to_string(),
            author: "Subhrajyoti Sen".to_string(),
            date: "2025-11-21T18:07:54Z".to_string(),
            url: "https://github.com/kavishdevar/librepods/commit/a80680ff7383cfe00f8f88f09e59dc1a9cdd66fc".to_string(),
        },
    ];

    engine.ingest_commits(commits).expect("Failed to ingest commits");

    let releases = vec![
        Release {
            tag_name: "linux-v0.1.0".to_string(),
            name: "linux-v0.1.0".to_string(),
            draft: false,
            prerelease: true,
            created_at: "2025-11-10T08:34:40Z".to_string(),
            published_at: "2025-11-10T08:40:41Z".to_string(),
            body: "Linux release".to_string(),
            assets: vec![
                ReleaseAsset {
                    name: "librepods".to_string(),
                    size: 14621200,
                    download_count: 929,
                    digest: "sha256:589c62ff2d98a72083e8942a5e726a08c8b87b213d5ea12aff2a5a57b8dab148".to_string(),
                    content_type: "application/octet-stream".to_string(),
                },
            ],
        },
    ];

    engine.ingest_releases(releases).expect("Failed to ingest releases");

    let tags = vec![
        Tag {
            name: "v0.2.0-alpha".to_string(),
            commit_sha: "993ba1ba085ec550ff579d69b608e063cc63611a".to_string(),
        },
        Tag {
            name: "v0.1.0-rc.4".to_string(),
            commit_sha: "b8e9765aff50e85c4e75fe526adc5d1c6c13ac43".to_string(),
        },
    ];

    engine.ingest_tags(tags).expect("Failed to ingest tags");

    let mut protocol_defs = HashMap::new();
    protocol_defs.insert(0x01u8, "BatteryStatus".to_string());
    protocol_defs.insert(0x02u8, "AncControl".to_string());
    protocol_defs.insert(0x03u8, "EarDetection".to_string());

    let protocol = ProtocolDefinition {
        service_uuid: "7DFC9000-7D1C-4951-86AA-8D9728F8D66C".to_string(),
        characteristic_uuid: "7DFC9001-7D1C-4951-86AA-8D9728F8D66C".to_string(),
        battery_service_uuid: "180F".to_string(),
        device_info_service_uuid: "180A".to_string(),
        message_types: protocol_defs,
    };

    engine
        .ingest_protocol_definitions(protocol)
        .expect("Failed to ingest protocol definitions");

    let firmware_versions = vec![
        FirmwareVersion {
            version: "5E135".to_string(),
            device_model: "AirPodsProGen2".to_string(),
            release_date: "2025-11-21".to_string(),
            features: vec!["BatteryMonitoring".to_string(), "ANC".to_string(), "SpatialAudio".to_string()],
        },
        FirmwareVersion {
            version: "5D134".to_string(),
            device_model: "AirPodsProGen2".to_string(),
            release_date: "2025-10-15".to_string(),
            features: vec!["BatteryMonitoring".to_string(), "ANC".to_string()],
        },
    ];

    engine
        .ingest_firmware_versions(firmware_versions)
        .expect("Failed to ingest firmware versions");

    engine
        .detect_new_message_types(vec![])
        .expect("Failed to detect new message types");

    engine
        .detect_new_uuids(vec![])
        .expect("Failed to detect new UUIDs");

    engine
        .detect_new_features(vec![])
        .expect("Failed to detect new features");

    let mut dependencies = HashMap::new();
    dependencies.insert("serde".to_string(), "1.0".to_string());
    dependencies.insert("tokio".to_string(), "1.35".to_string());

    engine
        .ingest_dependency_changes(dependencies)
        .expect("Failed to ingest dependency changes");

    let diff = engine.get_diff();
    assert_eq!(diff.repository.full_name, "kavishdevar/librepods");
    assert_eq!(diff.repository.stars, 14382);
    assert_eq!(diff.latest_commits.len(), 2);
    assert_eq!(diff.latest_releases.len(), 1);
    assert_eq!(diff.tags.len(), 2);
    assert_eq!(diff.firmware_versions.len(), 2);
    assert!(!diff.detect_protocol_drift());
    assert!(!diff.has_breaking_changes());

    let report = engine.generate_ingestion_report();
    assert!(report.contains("UPSTREAM DATA INGESTION REPORT"));
    assert!(report.contains("kavishdevar/librepods"));
    assert!(report.contains("14382"));
}

#[test]
fn test_protocol_analyzer_integration() {
    let analyzer = ProtocolAnalyzer::new();

    let message_types = vec![0x01, 0x02, 0x03, 0xFF];
    let (known, unknown) = analyzer.analyze_message_types(message_types);

    assert_eq!(known.len(), 3);
    assert_eq!(unknown.len(), 1);
    assert_eq!(unknown[0], 0xFF);

    let uuids = vec![
        "7DFC9000-7D1C-4951-86AA-8D9728F8D66C".to_string(),
        "UNKNOWN-UUID".to_string(),
    ];
    let (known_uuids, unknown_uuids) = analyzer.analyze_uuids(uuids);

    assert_eq!(known_uuids.len(), 1);
    assert_eq!(unknown_uuids.len(), 1);

    let has_drift = analyzer.detect_drift(vec![0x01, 0xFF], vec!["UNKNOWN-UUID".to_string()]);
    assert!(has_drift);

    let report = analyzer.generate_analysis_report(vec![0x01, 0x02], vec!["7DFC9000-7D1C-4951-86AA-8D9728F8D66C".to_string()]);
    assert!(report.contains("PROTOCOL ANALYSIS REPORT"));
}

#[test]
fn test_firmware_analyzer_integration() {
    let analyzer = FirmwareAnalyzer::new();

    assert!(analyzer.is_known_version("5E135"));
    assert!(!analyzer.is_known_version("UNKNOWN"));

    let info = analyzer.get_firmware_info("5E135");
    assert!(info.is_some());
    assert_eq!(info.unwrap().device_model, "AirPodsProGen2");

    let latest = analyzer.get_latest_version();
    assert!(latest.is_some());

    let versions = analyzer.get_versions_for_model("AirPodsProGen2");
    assert!(versions.len() > 0);

    let old_versions = vec!["5E135".to_string(), "5D134".to_string()];
    let new_versions = vec!["5E135".to_string(), "5C133".to_string()];
    let (added, removed) = analyzer.detect_version_changes(old_versions, new_versions);

    assert_eq!(added.len(), 1);
    assert_eq!(removed.len(), 1);

    let report = analyzer.generate_firmware_report();
    assert!(report.contains("FIRMWARE ANALYSIS REPORT"));
}

#[test]
fn test_protocol_drift_detection() {
    let mut engine = DataIngestionEngine::new();

    assert!(!engine.has_protocol_drift());

    engine
        .detect_new_message_types(vec!["NewType".to_string()])
        .expect("Failed to detect new message types");

    assert!(engine.has_protocol_drift());
}

#[test]
fn test_breaking_changes_detection() {
    let mut engine = DataIngestionEngine::new();

    assert!(!engine.has_breaking_changes());

    engine
        .detect_new_message_types(vec![
            "Type1".to_string(),
            "Type2".to_string(),
            "Type3".to_string(),
        ])
        .expect("Failed to detect new message types");

    assert!(engine.has_breaking_changes());
}

#[test]
fn test_serialization_to_json() {
    let mut engine = DataIngestionEngine::new();

    engine
        .ingest_repository_metadata(
            "test".to_string(),
            "test/repo".to_string(),
            "testuser".to_string(),
            "https://github.com/test/repo".to_string(),
            "Test repository".to_string(),
            100,
            10,
            5,
            "Rust".to_string(),
            "GPL-3.0".to_string(),
            "2025-11-21".to_string(),
            "2025-11-21".to_string(),
        )
        .expect("Failed to ingest metadata");

    let json = engine.serialize_diff().expect("Failed to serialize");
    assert!(json.contains("test/repo"));
    assert!(json.contains("100"));
}

#[test]
fn test_dependency_tracking() {
    let mut engine = DataIngestionEngine::new();

    let mut deps = HashMap::new();
    deps.insert("serde".to_string(), "1.0.0".to_string());
    deps.insert("tokio".to_string(), "1.35.0".to_string());
    deps.insert("nom".to_string(), "7.1.0".to_string());

    engine
        .ingest_dependency_changes(deps)
        .expect("Failed to ingest dependencies");

    let diff = engine.get_diff();
    assert_eq!(diff.dependency_changes.len(), 3);
    assert_eq!(diff.dependency_changes.get("serde"), Some(&"1.0.0".to_string()));
}

#[test]
fn test_latest_commit_retrieval() {
    let mut engine = DataIngestionEngine::new();

    let commits = vec![
        Commit {
            sha: "abc123".to_string(),
            message: "First commit".to_string(),
            author: "Author1".to_string(),
            date: "2025-11-20".to_string(),
            url: "https://github.com/test1".to_string(),
        },
        Commit {
            sha: "def456".to_string(),
            message: "Second commit".to_string(),
            author: "Author2".to_string(),
            date: "2025-11-21".to_string(),
            url: "https://github.com/test2".to_string(),
        },
    ];

    engine.ingest_commits(commits).expect("Failed to ingest commits");

    let latest = engine.get_latest_commit();
    assert!(latest.is_some());
    assert_eq!(latest.unwrap().sha, "abc123");
}

#[test]
fn test_firmware_feature_extraction() {
    let analyzer = FirmwareAnalyzer::new();

    let features = analyzer.extract_features_from_version("5E135");
    assert!(features.len() > 0);
    assert!(features.contains(&"BatteryMonitoring".to_string()));
}
