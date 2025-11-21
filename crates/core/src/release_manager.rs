use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseVersion {
    pub version: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub build_metadata: Option<String>,
}

impl ReleaseVersion {
    pub fn new(version: String) -> Self {
        let parts: Vec<&str> = version.split('-').collect();
        let base_parts: Vec<&str> = parts[0].split('.').collect();
        
        let major = base_parts.get(0).and_then(|v| v.parse().ok()).unwrap_or(0);
        let minor = base_parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        let patch = base_parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(0);
        
        let prerelease = parts.get(1).map(|s| s.to_string());
        let build_metadata = parts.get(2).map(|s| s.to_string());
        
        Self {
            version,
            major,
            minor,
            patch,
            prerelease,
            build_metadata,
        }
    }

    pub fn is_prerelease(&self) -> bool {
        self.prerelease.is_some()
    }

    pub fn is_stable(&self) -> bool {
        !self.is_prerelease()
    }

    pub fn get_semver(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseArtifact {
    pub artifact_id: String,
    pub artifact_type: ArtifactType,
    pub file_path: String,
    pub file_size: u64,
    pub checksum: String,
    pub checksum_algorithm: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactType {
    Binary,
    Library,
    Documentation,
    SourceCode,
    SBOM,
    ReleaseNotes,
    Changelog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub release_id: String,
    pub version: ReleaseVersion,
    pub release_date: String,
    pub artifacts: Vec<ReleaseArtifact>,
    pub release_notes: String,
    pub changelog: String,
    pub is_draft: bool,
    pub is_prerelease: bool,
    pub download_count: u64,
}

impl Release {
    pub fn new(version: String) -> Self {
        let release_version = ReleaseVersion::new(version.clone());
        Self {
            release_id: format!("release-{}", version),
            version: release_version,
            release_date: "2025-11-21T00:00:00Z".to_string(),
            artifacts: Vec::new(),
            release_notes: String::new(),
            is_draft: true,
            is_prerelease: false,
            download_count: 0,
            changelog: String::new(),
        }
    }

    pub fn add_artifact(&mut self, artifact: ReleaseArtifact) {
        self.artifacts.push(artifact);
    }

    pub fn set_release_notes(&mut self, notes: String) {
        self.release_notes = notes;
    }

    pub fn set_changelog(&mut self, changelog: String) {
        self.changelog = changelog;
    }

    pub fn publish(&mut self) {
        self.is_draft = false;
        self.is_prerelease = self.version.is_prerelease();
    }

    pub fn get_total_size(&self) -> u64 {
        self.artifacts.iter().map(|a| a.file_size).sum()
    }

    pub fn get_artifacts_by_type(&self, artifact_type: ArtifactType) -> Vec<&ReleaseArtifact> {
        self.artifacts.iter().filter(|a| a.artifact_type == artifact_type).collect()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== RELEASE REPORT ===\n\n");

        report.push_str(&format!("Version: {}\n", self.version.version));
        report.push_str(&format!("Release Date: {}\n", self.release_date));
        report.push_str(&format!("Status: {}\n", if self.is_draft { "Draft" } else { "Published" }));
        report.push_str(&format!("Prerelease: {}\n\n", self.is_prerelease));

        report.push_str(&format!("Artifacts: {}\n", self.artifacts.len()));
        report.push_str(&format!("Total Size: {:.1} MB\n\n", self.get_total_size() as f32 / 1024.0 / 1024.0));

        report.push_str("Artifacts:\n");
        for artifact in &self.artifacts {
            report.push_str(&format!("  {} ({:?}) - {:.1} MB\n", 
                artifact.artifact_id, artifact.artifact_type, artifact.file_size as f32 / 1024.0 / 1024.0));
        }

        report
    }
}

impl Default for Release {
    fn default() -> Self {
        Self::new("1.0.0".to_string())
    }
}

pub struct ReleaseManager {
    releases: HashMap<String, Release>,
}

impl ReleaseManager {
    pub fn new() -> Self {
        Self {
            releases: HashMap::new(),
        }
    }

    pub fn create_release(&mut self, version: String) -> &mut Release {
        let release = Release::new(version.clone());
        self.releases.insert(version, release);
        self.releases.values_mut().last().unwrap()
    }

    pub fn get_release(&self, version: &str) -> Option<&Release> {
        self.releases.get(version)
    }

    pub fn get_latest_release(&self) -> Option<&Release> {
        self.releases.values().max_by_key(|r| (r.version.major, r.version.minor, r.version.patch))
    }

    pub fn publish_release(&mut self, version: &str) -> bool {
        if let Some(release) = self.releases.get_mut(version) {
            release.publish();
            true
        } else {
            false
        }
    }

    pub fn get_all_releases(&self) -> Vec<&Release> {
        self.releases.values().collect()
    }

    pub fn get_stable_releases(&self) -> Vec<&Release> {
        self.releases.values().filter(|r| r.version.is_stable()).collect()
    }

    pub fn get_prerelease_releases(&self) -> Vec<&Release> {
        self.releases.values().filter(|r| r.version.is_prerelease()).collect()
    }
}

impl Default for ReleaseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_version() {
        let version = ReleaseVersion::new("1.0.0".to_string());
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert!(!version.is_prerelease());
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
    fn test_release_manager() {
        let mut manager = ReleaseManager::new();
        manager.create_release("1.0.0".to_string());
        assert_eq!(manager.get_all_releases().len(), 1);
    }
}
