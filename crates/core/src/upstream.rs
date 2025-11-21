use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamRepository {
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub url: String,
    pub description: String,
    pub stars: u32,
    pub forks: u32,
    pub open_issues: u32,
    pub language: String,
    pub license: String,
    pub updated_at: String,
    pub pushed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub date: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub body: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub size: u64,
    pub download_count: u32,
    pub digest: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub commit_sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDefinition {
    pub service_uuid: String,
    pub characteristic_uuid: String,
    pub battery_service_uuid: String,
    pub device_info_service_uuid: String,
    pub message_types: HashMap<u8, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareVersion {
    pub version: String,
    pub device_model: String,
    pub release_date: String,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamDiff {
    pub repository: UpstreamRepository,
    pub latest_commits: Vec<Commit>,
    pub latest_releases: Vec<Release>,
    pub tags: Vec<Tag>,
    pub protocol_definitions: ProtocolDefinition,
    pub firmware_versions: Vec<FirmwareVersion>,
    pub new_message_types: Vec<String>,
    pub new_uuids: Vec<String>,
    pub new_features: Vec<String>,
    pub dependency_changes: HashMap<String, String>,
    pub last_sync: String,
}

impl UpstreamDiff {
    pub fn new() -> Self {
        Self {
            repository: UpstreamRepository {
                name: String::new(),
                full_name: String::new(),
                owner: String::new(),
                url: String::new(),
                description: String::new(),
                stars: 0,
                forks: 0,
                open_issues: 0,
                language: String::new(),
                license: String::new(),
                updated_at: String::new(),
                pushed_at: String::new(),
            },
            latest_commits: Vec::new(),
            latest_releases: Vec::new(),
            tags: Vec::new(),
            protocol_definitions: ProtocolDefinition {
                service_uuid: String::new(),
                characteristic_uuid: String::new(),
                battery_service_uuid: String::new(),
                device_info_service_uuid: String::new(),
                message_types: HashMap::new(),
            },
            firmware_versions: Vec::new(),
            new_message_types: Vec::new(),
            new_uuids: Vec::new(),
            new_features: Vec::new(),
            dependency_changes: HashMap::new(),
            last_sync: "2025-11-21T23:55:00Z".to_string(),
        }
    }

    pub fn add_commit(&mut self, commit: Commit) {
        self.latest_commits.push(commit);
    }

    pub fn add_release(&mut self, release: Release) {
        self.latest_releases.push(release);
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn add_firmware(&mut self, firmware: FirmwareVersion) {
        self.firmware_versions.push(firmware);
    }

    pub fn detect_protocol_drift(&self) -> bool {
        !self.new_message_types.is_empty() || !self.new_uuids.is_empty()
    }

    pub fn has_breaking_changes(&self) -> bool {
        self.new_message_types.len() > 2 || self.new_uuids.len() > 1
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for UpstreamDiff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upstream_diff_creation() {
        let diff = UpstreamDiff::new();
        assert_eq!(diff.latest_commits.len(), 0);
        assert_eq!(diff.latest_releases.len(), 0);
        assert!(!diff.detect_protocol_drift());
    }

    #[test]
    fn test_add_commit() {
        let mut diff = UpstreamDiff::new();
        let commit = Commit {
            sha: "abc123".to_string(),
            message: "test commit".to_string(),
            author: "Test Author".to_string(),
            date: "2025-11-21".to_string(),
            url: "https://github.com/test".to_string(),
        };
        diff.add_commit(commit);
        assert_eq!(diff.latest_commits.len(), 1);
    }

    #[test]
    fn test_protocol_drift_detection() {
        let mut diff = UpstreamDiff::new();
        assert!(!diff.detect_protocol_drift());
        diff.new_message_types.push("NewType".to_string());
        assert!(diff.detect_protocol_drift());
    }

    #[test]
    fn test_breaking_changes_detection() {
        let mut diff = UpstreamDiff::new();
        assert!(!diff.has_breaking_changes());
        diff.new_message_types.push("Type1".to_string());
        diff.new_message_types.push("Type2".to_string());
        diff.new_message_types.push("Type3".to_string());
        assert!(diff.has_breaking_changes());
    }
}
