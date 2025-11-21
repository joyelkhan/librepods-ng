use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEntry {
    pub entry_id: String,
    pub change_type: ChangeType,
    pub title: String,
    pub description: String,
    pub author: String,
    pub pr_number: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Feature,
    Bugfix,
    Improvement,
    Security,
    Breaking,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNotes {
    pub version: String,
    pub release_date: String,
    pub summary: String,
    pub changes: Vec<ChangeEntry>,
    pub breaking_changes: Vec<String>,
    pub deprecations: Vec<String>,
    pub known_issues: Vec<String>,
    pub contributors: Vec<String>,
}

impl ReleaseNotes {
    pub fn new(version: String) -> Self {
        Self {
            version,
            release_date: "2025-11-21T00:00:00Z".to_string(),
            summary: String::new(),
            changes: Vec::new(),
            breaking_changes: Vec::new(),
            deprecations: Vec::new(),
            known_issues: Vec::new(),
            contributors: Vec::new(),
        }
    }

    pub fn add_change(&mut self, change: ChangeEntry) {
        if change.change_type == ChangeType::Breaking {
            self.breaking_changes.push(change.title.clone());
        }
        if change.change_type == ChangeType::Deprecated {
            self.deprecations.push(change.title.clone());
        }
        self.changes.push(change);
    }

    pub fn add_known_issue(&mut self, issue: String) {
        self.known_issues.push(issue);
    }

    pub fn add_contributor(&mut self, contributor: String) {
        if !self.contributors.contains(&contributor) {
            self.contributors.push(contributor);
        }
    }

    pub fn set_summary(&mut self, summary: String) {
        self.summary = summary;
    }

    pub fn get_changes_by_type(&self, change_type: ChangeType) -> Vec<&ChangeEntry> {
        self.changes.iter().filter(|c| c.change_type == change_type).collect()
    }

    pub fn get_feature_count(&self) -> usize {
        self.get_changes_by_type(ChangeType::Feature).len()
    }

    pub fn get_bugfix_count(&self) -> usize {
        self.get_changes_by_type(ChangeType::Bugfix).len()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Release {}\n\n", self.version));
        md.push_str(&format!("**Release Date**: {}\n\n", self.release_date));

        if !self.summary.is_empty() {
            md.push_str(&format!("{}\n\n", self.summary));
        }

        if !self.breaking_changes.is_empty() {
            md.push_str("## ⚠️ Breaking Changes\n\n");
            for change in &self.breaking_changes {
                md.push_str(&format!("- {}\n", change));
            }
            md.push_str("\n");
        }

        let features = self.get_changes_by_type(ChangeType::Feature);
        if !features.is_empty() {
            md.push_str("## ✨ Features\n\n");
            for change in features {
                md.push_str(&format!("- {} (#{:?})\n", change.title, change.pr_number));
            }
            md.push_str("\n");
        }

        let bugfixes = self.get_changes_by_type(ChangeType::Bugfix);
        if !bugfixes.is_empty() {
            md.push_str("## 🐛 Bug Fixes\n\n");
            for change in bugfixes {
                md.push_str(&format!("- {}\n", change.title));
            }
            md.push_str("\n");
        }

        let improvements = self.get_changes_by_type(ChangeType::Improvement);
        if !improvements.is_empty() {
            md.push_str("## 📈 Improvements\n\n");
            for change in improvements {
                md.push_str(&format!("- {}\n", change.title));
            }
            md.push_str("\n");
        }

        if !self.known_issues.is_empty() {
            md.push_str("## 🔍 Known Issues\n\n");
            for issue in &self.known_issues {
                md.push_str(&format!("- {}\n", issue));
            }
            md.push_str("\n");
        }

        if !self.contributors.is_empty() {
            md.push_str("## 👥 Contributors\n\n");
            for contributor in &self.contributors {
                md.push_str(&format!("- {}\n", contributor));
            }
        }

        md
    }
}

impl Default for ReleaseNotes {
    fn default() -> Self {
        Self::new("1.0.0".to_string())
    }
}

pub struct ReleaseNotesGenerator {
    notes: HashMap<String, ReleaseNotes>,
}

impl ReleaseNotesGenerator {
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
        }
    }

    pub fn create_notes(&mut self, version: String) -> &mut ReleaseNotes {
        let notes = ReleaseNotes::new(version.clone());
        self.notes.insert(version.clone(), notes);
        self.notes.get_mut(&version).unwrap()
    }

    pub fn get_notes(&self, version: &str) -> Option<&ReleaseNotes> {
        self.notes.get(version)
    }

    pub fn get_all_notes(&self) -> Vec<&ReleaseNotes> {
        self.notes.values().collect()
    }
}

impl Default for ReleaseNotesGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_release_notes_generator() {
        let mut generator = ReleaseNotesGenerator::new();
        generator.create_notes("1.0.0".to_string());
        assert_eq!(generator.get_all_notes().len(), 1);
    }
}
