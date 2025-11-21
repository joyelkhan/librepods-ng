use crate::legal_scan::DMCANotice;
use std::collections::HashMap;

pub struct DMCAScanner {
    known_notices: HashMap<String, DMCANotice>,
    blocked_repositories: Vec<String>,
    flagged_keywords: Vec<String>,
}

impl DMCAScanner {
    pub fn new() -> Self {
        let mut scanner = Self {
            known_notices: HashMap::new(),
            blocked_repositories: Vec::new(),
            flagged_keywords: vec![
                "proprietary".to_string(),
                "confidential".to_string(),
                "trade secret".to_string(),
                "nda".to_string(),
                "restricted".to_string(),
            ],
        };
        scanner.initialize_known_notices();
        scanner
    }

    fn initialize_known_notices(&mut self) {
        // No known DMCA notices for librepods-ng
    }

    pub fn scan_repository(&self, repo_name: &str, description: &str) -> Vec<DMCANotice> {
        let mut notices = Vec::new();

        if self.blocked_repositories.contains(&repo_name.to_string()) {
            notices.push(DMCANotice {
                date: "2025-11-21".to_string(),
                repository: repo_name.to_string(),
                reason: "Repository is on blocked list".to_string(),
                status: "Active".to_string(),
            });
        }

        for keyword in &self.flagged_keywords {
            if description.to_lowercase().contains(keyword) {
                notices.push(DMCANotice {
                    date: "2025-11-21".to_string(),
                    repository: repo_name.to_string(),
                    reason: format!("Flagged keyword detected: {}", keyword),
                    status: "Review".to_string(),
                });
            }
        }

        notices
    }

    pub fn check_for_takedown(&self, repo_name: &str) -> bool {
        self.known_notices.contains_key(repo_name) || self.blocked_repositories.contains(&repo_name.to_string())
    }

    pub fn get_notice(&self, repo_name: &str) -> Option<&DMCANotice> {
        self.known_notices.get(repo_name)
    }

    pub fn add_blocked_repository(&mut self, repo: String) {
        self.blocked_repositories.push(repo);
    }

    pub fn is_repository_safe(&self, repo_name: &str) -> bool {
        !self.check_for_takedown(repo_name)
    }

    pub fn scan_commit_messages(&self, messages: Vec<String>) -> Vec<String> {
        let mut flagged = Vec::new();

        for message in messages {
            for keyword in &self.flagged_keywords {
                if message.to_lowercase().contains(keyword) {
                    flagged.push(message.clone());
                    break;
                }
            }
        }

        flagged
    }

    pub fn generate_dmca_report(&self, repo_name: &str, description: &str) -> String {
        let mut report = String::new();
        report.push_str("=== DMCA SCAN REPORT ===\n\n");

        report.push_str(&format!("Repository: {}\n", repo_name));
        report.push_str(&format!("Safe: {}\n\n", self.is_repository_safe(repo_name)));

        let notices = self.scan_repository(repo_name, description);
        report.push_str(&format!("Notices Found: {}\n", notices.len()));

        for notice in notices {
            report.push_str(&format!("  - {} ({})\n", notice.reason, notice.status));
        }

        report
    }
}

impl Default for DMCAScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = DMCAScanner::new();
        assert!(scanner.is_repository_safe("librepods-ng"));
    }

    #[test]
    fn test_scan_repository() {
        let scanner = DMCAScanner::new();
        let notices = scanner.scan_repository("test-repo", "A normal repository");
        assert_eq!(notices.len(), 0);
    }

    #[test]
    fn test_flagged_keyword_detection() {
        let scanner = DMCAScanner::new();
        let notices = scanner.scan_repository("test-repo", "This is proprietary code");
        assert!(notices.len() > 0);
    }

    #[test]
    fn test_blocked_repository() {
        let mut scanner = DMCAScanner::new();
        scanner.add_blocked_repository("blocked-repo".to_string());
        assert!(!scanner.is_repository_safe("blocked-repo"));
    }

    #[test]
    fn test_scan_commit_messages() {
        let scanner = DMCAScanner::new();
        let messages = vec![
            "Fix bug".to_string(),
            "Add proprietary code".to_string(),
            "Update docs".to_string(),
        ];
        let flagged = scanner.scan_commit_messages(messages);
        assert_eq!(flagged.len(), 1);
    }
}
