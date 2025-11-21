use crate::error::{Error, Result};
use crate::upstream::{Commit, FirmwareVersion, ProtocolDefinition, Release, Tag, UpstreamDiff, UpstreamRepository};
use std::collections::HashMap;

pub struct DataIngestionEngine {
    upstream_diff: UpstreamDiff,
    protocol_cache: HashMap<String, String>,
    firmware_cache: HashMap<String, FirmwareVersion>,
}

impl DataIngestionEngine {
    pub fn new() -> Self {
        Self {
            upstream_diff: UpstreamDiff::new(),
            protocol_cache: HashMap::new(),
            firmware_cache: HashMap::new(),
        }
    }

    pub fn ingest_repository_metadata(
        &mut self,
        name: String,
        full_name: String,
        owner: String,
        url: String,
        description: String,
        stars: u32,
        forks: u32,
        open_issues: u32,
        language: String,
        license: String,
        updated_at: String,
        pushed_at: String,
    ) -> Result<()> {
        self.upstream_diff.repository = UpstreamRepository {
            name,
            full_name,
            owner,
            url,
            description,
            stars,
            forks,
            open_issues,
            language,
            license,
            updated_at,
            pushed_at,
        };
        Ok(())
    }

    pub fn ingest_commits(&mut self, commits: Vec<Commit>) -> Result<()> {
        for commit in commits {
            self.upstream_diff.add_commit(commit);
        }
        Ok(())
    }

    pub fn ingest_releases(&mut self, releases: Vec<Release>) -> Result<()> {
        for release in releases {
            self.upstream_diff.add_release(release);
        }
        Ok(())
    }

    pub fn ingest_tags(&mut self, tags: Vec<Tag>) -> Result<()> {
        for tag in tags {
            self.upstream_diff.add_tag(tag);
        }
        Ok(())
    }

    pub fn ingest_protocol_definitions(&mut self, protocol: ProtocolDefinition) -> Result<()> {
        self.upstream_diff.protocol_definitions = protocol;
        Ok(())
    }

    pub fn ingest_firmware_versions(&mut self, firmware_versions: Vec<FirmwareVersion>) -> Result<()> {
        for firmware in firmware_versions {
            self.upstream_diff.add_firmware(firmware.clone());
            self.firmware_cache.insert(firmware.version.clone(), firmware);
        }
        Ok(())
    }

    pub fn detect_new_message_types(&mut self, new_types: Vec<String>) -> Result<()> {
        self.upstream_diff.new_message_types = new_types;
        Ok(())
    }

    pub fn detect_new_uuids(&mut self, new_uuids: Vec<String>) -> Result<()> {
        self.upstream_diff.new_uuids = new_uuids;
        Ok(())
    }

    pub fn detect_new_features(&mut self, new_features: Vec<String>) -> Result<()> {
        self.upstream_diff.new_features = new_features;
        Ok(())
    }

    pub fn ingest_dependency_changes(&mut self, changes: HashMap<String, String>) -> Result<()> {
        self.upstream_diff.dependency_changes = changes;
        Ok(())
    }

    pub fn get_latest_commit(&self) -> Option<&Commit> {
        self.upstream_diff.latest_commits.first()
    }

    pub fn get_latest_release(&self) -> Option<&Release> {
        self.upstream_diff.latest_releases.first()
    }

    pub fn get_firmware_by_version(&self, version: &str) -> Option<&FirmwareVersion> {
        self.firmware_cache.get(version)
    }

    pub fn has_protocol_drift(&self) -> bool {
        self.upstream_diff.detect_protocol_drift()
    }

    pub fn has_breaking_changes(&self) -> bool {
        self.upstream_diff.has_breaking_changes()
    }

    pub fn get_diff(&self) -> &UpstreamDiff {
        &self.upstream_diff
    }

    pub fn get_diff_mut(&mut self) -> &mut UpstreamDiff {
        &mut self.upstream_diff
    }

    pub fn serialize_diff(&self) -> Result<String> {
        self.upstream_diff
            .serialize_to_json()
            .map_err(|_| Error::ParseError("Failed to serialize upstream diff".to_string()))
    }

    pub fn generate_ingestion_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== UPSTREAM DATA INGESTION REPORT ===\n\n");

        report.push_str(&format!("Repository: {}\n", self.upstream_diff.repository.full_name));
        report.push_str(&format!("Owner: {}\n", self.upstream_diff.repository.owner));
        report.push_str(&format!("Stars: {}\n", self.upstream_diff.repository.stars));
        report.push_str(&format!("Forks: {}\n", self.upstream_diff.repository.forks));
        report.push_str(&format!("Open Issues: {}\n\n", self.upstream_diff.repository.open_issues));

        report.push_str(&format!("Latest Commits: {}\n", self.upstream_diff.latest_commits.len()));
        if let Some(commit) = self.get_latest_commit() {
            report.push_str(&format!("  Latest: {} ({})\n", commit.sha[..8].to_string(), commit.date));
        }

        report.push_str(&format!("\nLatest Releases: {}\n", self.upstream_diff.latest_releases.len()));
        if let Some(release) = self.get_latest_release() {
            report.push_str(&format!("  Latest: {} ({})\n", release.tag_name, release.published_at));
        }

        report.push_str(&format!("\nTags: {}\n", self.upstream_diff.tags.len()));
        report.push_str(&format!("Firmware Versions: {}\n", self.upstream_diff.firmware_versions.len()));

        report.push_str(&format!("\nProtocol Drift: {}\n", self.has_protocol_drift()));
        report.push_str(&format!("Breaking Changes: {}\n", self.has_breaking_changes()));

        report.push_str(&format!("\nNew Message Types: {}\n", self.upstream_diff.new_message_types.len()));
        for msg_type in &self.upstream_diff.new_message_types {
            report.push_str(&format!("  - {}\n", msg_type));
        }

        report.push_str(&format!("\nNew UUIDs: {}\n", self.upstream_diff.new_uuids.len()));
        for uuid in &self.upstream_diff.new_uuids {
            report.push_str(&format!("  - {}\n", uuid));
        }

        report.push_str(&format!("\nNew Features: {}\n", self.upstream_diff.new_features.len()));
        for feature in &self.upstream_diff.new_features {
            report.push_str(&format!("  - {}\n", feature));
        }

        report.push_str(&format!("\nDependency Changes: {}\n", self.upstream_diff.dependency_changes.len()));
        for (dep, version) in &self.upstream_diff.dependency_changes {
            report.push_str(&format!("  - {}: {}\n", dep, version));
        }

        report.push_str(&format!("\nLast Sync: {}\n", self.upstream_diff.last_sync));

        report
    }
}

impl Default for DataIngestionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = DataIngestionEngine::new();
        assert_eq!(engine.upstream_diff.latest_commits.len(), 0);
    }

    #[test]
    fn test_ingest_repository_metadata() {
        let mut engine = DataIngestionEngine::new();
        let result = engine.ingest_repository_metadata(
            "librepods".to_string(),
            "kavishdevar/librepods".to_string(),
            "kavishdevar".to_string(),
            "https://github.com/kavishdevar/librepods".to_string(),
            "AirPods liberated".to_string(),
            14382,
            434,
            52,
            "Kotlin".to_string(),
            "GPL-3.0".to_string(),
            "2025-11-21".to_string(),
            "2025-11-21".to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(engine.upstream_diff.repository.stars, 14382);
    }

    #[test]
    fn test_ingest_commits() {
        let mut engine = DataIngestionEngine::new();
        let commits = vec![
            Commit {
                sha: "abc123".to_string(),
                message: "test commit 1".to_string(),
                author: "Author 1".to_string(),
                date: "2025-11-21".to_string(),
                url: "https://github.com/test1".to_string(),
            },
            Commit {
                sha: "def456".to_string(),
                message: "test commit 2".to_string(),
                author: "Author 2".to_string(),
                date: "2025-11-20".to_string(),
                url: "https://github.com/test2".to_string(),
            },
        ];
        let result = engine.ingest_commits(commits);
        assert!(result.is_ok());
        assert_eq!(engine.upstream_diff.latest_commits.len(), 2);
    }

    #[test]
    fn test_protocol_drift_detection() {
        let mut engine = DataIngestionEngine::new();
        assert!(!engine.has_protocol_drift());
        let _ = engine.detect_new_message_types(vec!["NewType".to_string()]);
        assert!(engine.has_protocol_drift());
    }

    #[test]
    fn test_breaking_changes_detection() {
        let mut engine = DataIngestionEngine::new();
        assert!(!engine.has_breaking_changes());
        let _ = engine.detect_new_message_types(vec![
            "Type1".to_string(),
            "Type2".to_string(),
            "Type3".to_string(),
        ]);
        assert!(engine.has_breaking_changes());
    }

    #[test]
    fn test_generate_report() {
        let engine = DataIngestionEngine::new();
        let report = engine.generate_ingestion_report();
        assert!(report.contains("UPSTREAM DATA INGESTION REPORT"));
    }
}
