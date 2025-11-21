use librepods_core::release_manager::*;
use librepods_core::sbom_generator::*;
use librepods_core::release_notes::*;

#[test]
fn test_phase7_complete_release_artifacts() {
    let mut release_mgr = ReleaseManager::new();
    let release = release_mgr.create_release("1.0.0".to_string());

    let binary = ReleaseArtifact {
        artifact_id: "binary-linux".to_string(),
        artifact_type: ArtifactType::Binary,
        file_path: "/releases/librepods-1.0.0-linux".to_string(),
        file_size: 50 * 1024 * 1024,
        checksum: "abc123".to_string(),
        checksum_algorithm: "sha256".to_string(),
    };

    release.add_artifact(binary);
    release.set_release_notes("Initial release".to_string());
    release.publish();

    let mut sbom_gen = SBOMGenerator::new();
    let sbom = sbom_gen.create_sbom("sbom-1".to_string(), "1.0.0".to_string());

    let component = Component {
        component_id: "serde".to_string(),
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        component_type: ComponentType::Library,
        license: "MIT".to_string(),
        supplier: "serde-rs".to_string(),
        purl: "pkg:cargo/serde@1.0.0".to_string(),
    };

    sbom.add_component(component);

    let mut notes_gen = ReleaseNotesGenerator::new();
    let notes = notes_gen.create_notes("1.0.0".to_string());

    let change = ChangeEntry {
        entry_id: "feat-1".to_string(),
        change_type: ChangeType::Feature,
        title: "Initial release".to_string(),
        description: "First production release".to_string(),
        author: "dev-team".to_string(),
        pr_number: Some(1),
    };

    notes.add_change(change);
    notes.set_summary("Production ready release".to_string());

    assert!(!release.is_draft);
    assert_eq!(sbom.get_total_components(), 1);
    assert_eq!(notes.get_feature_count(), 1);
}

#[test]
fn test_release_version() {
    let version = ReleaseVersion::new("1.0.0".to_string());
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert!(!version.is_prerelease());
}

#[test]
fn test_release_version_prerelease() {
    let version = ReleaseVersion::new("1.0.0-rc.1".to_string());
    assert!(version.is_prerelease());
}

#[test]
fn test_release_creation() {
    let release = Release::new("1.0.0".to_string());
    assert_eq!(release.version.version, "1.0.0");
    assert!(release.is_draft);
}

#[test]
fn test_add_artifact() {
    let mut release = Release::new("1.0.0".to_string());
    let artifact = ReleaseArtifact {
        artifact_id: "binary-1".to_string(),
        artifact_type: ArtifactType::Binary,
        file_path: "/path/to/binary".to_string(),
        file_size: 1024 * 1024,
        checksum: "abc123".to_string(),
        checksum_algorithm: "sha256".to_string(),
    };
    release.add_artifact(artifact);
    assert_eq!(release.artifacts.len(), 1);
}

#[test]
fn test_publish_release() {
    let mut release = Release::new("1.0.0".to_string());
    release.publish();
    assert!(!release.is_draft);
}

#[test]
fn test_release_total_size() {
    let mut release = Release::new("1.0.0".to_string());
    for i in 0..3 {
        let artifact = ReleaseArtifact {
            artifact_id: format!("artifact-{}", i),
            artifact_type: ArtifactType::Binary,
            file_path: format!("/path/to/artifact{}", i),
            file_size: 1024 * 1024,
            checksum: "abc123".to_string(),
            checksum_algorithm: "sha256".to_string(),
        };
        release.add_artifact(artifact);
    }
    assert_eq!(release.get_total_size(), 3 * 1024 * 1024);
}

#[test]
fn test_sbom_creation() {
    let sbom = SBOM::new("sbom-1".to_string(), "1.0.0".to_string());
    assert_eq!(sbom.sbom_id, "sbom-1");
    assert_eq!(sbom.get_total_components(), 0);
}

#[test]
fn test_add_component() {
    let mut sbom = SBOM::new("sbom-1".to_string(), "1.0.0".to_string());
    let component = Component {
        component_id: "comp-1".to_string(),
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        component_type: ComponentType::Library,
        license: "MIT".to_string(),
        supplier: "serde-rs".to_string(),
        purl: "pkg:cargo/serde@1.0.0".to_string(),
    };
    sbom.add_component(component);
    assert_eq!(sbom.get_total_components(), 1);
}

#[test]
fn test_license_tracking() {
    let mut sbom = SBOM::new("sbom-1".to_string(), "1.0.0".to_string());
    for i in 0..3 {
        let component = Component {
            component_id: format!("comp-{}", i),
            name: format!("lib{}", i),
            version: "1.0.0".to_string(),
            component_type: ComponentType::Library,
            license: "MIT".to_string(),
            supplier: "test".to_string(),
            purl: format!("pkg:cargo/lib{}@1.0.0", i),
        };
        sbom.add_component(component);
    }
    assert_eq!(sbom.licenses.get("MIT"), Some(&3));
}

#[test]
fn test_release_notes_creation() {
    let notes = ReleaseNotes::new("1.0.0".to_string());
    assert_eq!(notes.version, "1.0.0");
}

#[test]
fn test_add_change() {
    let mut notes = ReleaseNotes::new("1.0.0".to_string());
    let change = ChangeEntry {
        entry_id: "change-1".to_string(),
        change_type: ChangeType::Feature,
        title: "Add new feature".to_string(),
        description: "A new feature".to_string(),
        author: "dev".to_string(),
        pr_number: Some(123),
    };
    notes.add_change(change);
    assert_eq!(notes.get_feature_count(), 1);
}

#[test]
fn test_breaking_changes() {
    let mut notes = ReleaseNotes::new("1.0.0".to_string());
    let change = ChangeEntry {
        entry_id: "change-1".to_string(),
        change_type: ChangeType::Breaking,
        title: "API change".to_string(),
        description: "Breaking change".to_string(),
        author: "dev".to_string(),
        pr_number: None,
    };
    notes.add_change(change);
    assert_eq!(notes.breaking_changes.len(), 1);
}

#[test]
fn test_markdown_generation() {
    let mut notes = ReleaseNotes::new("1.0.0".to_string());
    notes.set_summary("Initial release".to_string());
    let md = notes.generate_markdown();
    assert!(md.contains("# Release 1.0.0"));
}

#[test]
fn test_release_manager() {
    let mut manager = ReleaseManager::new();
    manager.create_release("1.0.0".to_string());
    assert_eq!(manager.get_all_releases().len(), 1);
}

#[test]
fn test_get_latest_release() {
    let mut manager = ReleaseManager::new();
    manager.create_release("1.0.0".to_string());
    manager.create_release("1.1.0".to_string());
    
    let latest = manager.get_latest_release().unwrap();
    assert_eq!(latest.version.version, "1.1.0");
}

#[test]
fn test_stable_releases() {
    let mut manager = ReleaseManager::new();
    manager.create_release("1.0.0".to_string());
    manager.create_release("1.1.0-rc.1".to_string());
    
    let stable = manager.get_stable_releases();
    assert_eq!(stable.len(), 1);
}

#[test]
fn test_prerelease_releases() {
    let mut manager = ReleaseManager::new();
    manager.create_release("1.0.0".to_string());
    manager.create_release("1.1.0-rc.1".to_string());
    
    let prerelease = manager.get_prerelease_releases();
    assert_eq!(prerelease.len(), 1);
}

#[test]
fn test_sbom_generator() {
    let mut generator = SBOMGenerator::new();
    generator.create_sbom("sbom-1".to_string(), "1.0.0".to_string());
    assert_eq!(generator.get_all_sboms().len(), 1);
}

#[test]
fn test_release_notes_generator() {
    let mut generator = ReleaseNotesGenerator::new();
    generator.create_notes("1.0.0".to_string());
    assert_eq!(generator.get_all_notes().len(), 1);
}

#[test]
fn test_release_report_generation() {
    let mut release = Release::new("1.0.0".to_string());
    let artifact = ReleaseArtifact {
        artifact_id: "binary-1".to_string(),
        artifact_type: ArtifactType::Binary,
        file_path: "/path/to/binary".to_string(),
        file_size: 1024 * 1024,
        checksum: "abc123".to_string(),
        checksum_algorithm: "sha256".to_string(),
    };
    release.add_artifact(artifact);
    
    let report = release.generate_report();
    assert!(report.contains("RELEASE REPORT"));
}

#[test]
fn test_sbom_report_generation() {
    let mut sbom = SBOM::new("sbom-1".to_string(), "1.0.0".to_string());
    let component = Component {
        component_id: "comp-1".to_string(),
        name: "serde".to_string(),
        version: "1.0.0".to_string(),
        component_type: ComponentType::Library,
        license: "MIT".to_string(),
        supplier: "serde-rs".to_string(),
        purl: "pkg:cargo/serde@1.0.0".to_string(),
    };
    sbom.add_component(component);
    
    let report = sbom.generate_report();
    assert!(report.contains("SBOM REPORT"));
}

#[test]
fn test_release_notes_markdown() {
    let mut notes = ReleaseNotes::new("1.0.0".to_string());
    notes.set_summary("Initial release".to_string());
    
    let change = ChangeEntry {
        entry_id: "feat-1".to_string(),
        change_type: ChangeType::Feature,
        title: "Add feature".to_string(),
        description: "New feature".to_string(),
        author: "dev".to_string(),
        pr_number: Some(1),
    };
    notes.add_change(change);
    
    let md = notes.generate_markdown();
    assert!(md.contains("✨ Features"));
}

#[test]
fn test_complete_phase7_workflow() {
    let mut release_mgr = ReleaseManager::new();
    let release = release_mgr.create_release("1.0.0-rivers".to_string());

    for i in 0..3 {
        let artifact = ReleaseArtifact {
            artifact_id: format!("artifact-{}", i),
            artifact_type: ArtifactType::Binary,
            file_path: format!("/releases/librepods-1.0.0-rivers-{}", i),
            file_size: 50 * 1024 * 1024,
            checksum: format!("checksum{}", i),
            checksum_algorithm: "sha256".to_string(),
        };
        release.add_artifact(artifact);
    }

    release.set_release_notes("Production release".to_string());
    release.publish();

    let mut sbom_gen = SBOMGenerator::new();
    let sbom = sbom_gen.create_sbom("sbom-rivers".to_string(), "1.0.0-rivers".to_string());

    for i in 0..5 {
        let component = Component {
            component_id: format!("dep-{}", i),
            name: format!("dependency-{}", i),
            version: "1.0.0".to_string(),
            component_type: ComponentType::Library,
            license: "GPL-3.0".to_string(),
            supplier: "librepods".to_string(),
            purl: format!("pkg:cargo/dependency-{}@1.0.0", i),
        };
        sbom.add_component(component);
    }

    let mut notes_gen = ReleaseNotesGenerator::new();
    let notes = notes_gen.create_notes("1.0.0-rivers".to_string());

    let feature = ChangeEntry {
        entry_id: "feat-complete".to_string(),
        change_type: ChangeType::Feature,
        title: "Complete Phase 7 implementation".to_string(),
        description: "Release artifacts ready".to_string(),
        author: "librepods-agent".to_string(),
        pr_number: None,
    };

    notes.add_change(feature);
    notes.set_summary("Production-ready release".to_string());
    notes.add_contributor("Rivers Engineering".to_string());

    assert!(!release.is_draft);
    assert_eq!(release.artifacts.len(), 3);
    assert_eq!(sbom.get_total_components(), 5);
    assert_eq!(notes.get_feature_count(), 1);
}
