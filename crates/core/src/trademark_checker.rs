use crate::legal_scan::TrademarkUsage;
use std::collections::HashMap;

pub struct TrademarkChecker {
    protected_trademarks: HashMap<String, TrademarkPolicy>,
    compliant_usages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TrademarkPolicy {
    pub term: String,
    pub owner: String,
    pub requires_permission: bool,
    pub allowed_contexts: Vec<String>,
    pub forbidden_contexts: Vec<String>,
}

impl TrademarkChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            protected_trademarks: HashMap::new(),
            compliant_usages: vec![
                "descriptive use".to_string(),
                "nominative use".to_string(),
                "comparative advertising".to_string(),
            ],
        };
        checker.initialize_trademarks();
        checker
    }

    fn initialize_trademarks(&mut self) {
        self.protected_trademarks.insert(
            "AirPods".to_string(),
            TrademarkPolicy {
                term: "AirPods".to_string(),
                owner: "Apple Inc.".to_string(),
                requires_permission: false,
                allowed_contexts: vec![
                    "descriptive use".to_string(),
                    "nominative use".to_string(),
                    "compatibility statements".to_string(),
                ],
                forbidden_contexts: vec![
                    "brand name".to_string(),
                    "product endorsement".to_string(),
                    "trademark registration".to_string(),
                ],
            },
        );

        self.protected_trademarks.insert(
            "Apple".to_string(),
            TrademarkPolicy {
                term: "Apple".to_string(),
                owner: "Apple Inc.".to_string(),
                requires_permission: true,
                allowed_contexts: vec!["descriptive use".to_string()],
                forbidden_contexts: vec![
                    "brand name".to_string(),
                    "company name".to_string(),
                    "product name".to_string(),
                ],
            },
        );
    }

    pub fn check_trademark_usage(&self, term: &str, context: &str) -> TrademarkUsage {
        if let Some(policy) = self.protected_trademarks.get(term) {
            let compliant = policy.allowed_contexts.iter().any(|c| context.to_lowercase().contains(c))
                && !policy.forbidden_contexts.iter().any(|c| context.to_lowercase().contains(c));

            let reason = if compliant {
                "Usage is compliant with trademark policy".to_string()
            } else {
                "Usage may violate trademark policy".to_string()
            };

            TrademarkUsage {
                term: term.to_string(),
                context: context.to_string(),
                compliant,
                reason,
            }
        } else {
            TrademarkUsage {
                term: term.to_string(),
                context: context.to_string(),
                compliant: true,
                reason: "Trademark not in protected list".to_string(),
            }
        }
    }

    pub fn scan_text(&self, text: &str) -> Vec<TrademarkUsage> {
        let mut usages = Vec::new();

        for (term, _) in &self.protected_trademarks {
            if text.contains(term) {
                let context = self.extract_context(text, term);
                usages.push(self.check_trademark_usage(term, &context));
            }
        }

        usages
    }

    fn extract_context(&self, text: &str, term: &str) -> String {
        for line in text.lines() {
            if line.contains(term) {
                return line.to_string();
            }
        }
        term.to_string()
    }

    pub fn is_descriptive_use(&self, context: &str) -> bool {
        context.to_lowercase().contains("descriptive")
            || context.to_lowercase().contains("compatibility")
            || context.to_lowercase().contains("works with")
    }

    pub fn is_nominative_use(&self, context: &str) -> bool {
        context.to_lowercase().contains("nominative")
            || context.to_lowercase().contains("refers to")
            || context.to_lowercase().contains("compatible with")
    }

    pub fn check_project_description(&self, description: &str) -> Vec<TrademarkUsage> {
        self.scan_text(description)
    }

    pub fn check_readme(&self, readme_content: &str) -> Vec<TrademarkUsage> {
        self.scan_text(readme_content)
    }

    pub fn generate_trademark_report(&self, usages: Vec<TrademarkUsage>) -> String {
        let mut report = String::new();
        report.push_str("=== TRADEMARK COMPLIANCE REPORT ===\n\n");

        let compliant = usages.iter().filter(|u| u.compliant).count();
        let total = usages.len();

        report.push_str(&format!("Compliant usages: {}/{}\n\n", compliant, total));

        for usage in usages {
            let status = if usage.compliant { "✓" } else { "✗" };
            report.push_str(&format!("{} {}\n", status, usage.term));
            report.push_str(&format!("  Context: {}\n", usage.context));
            report.push_str(&format!("  Reason: {}\n\n", usage.reason));
        }

        report
    }
}

impl Default for TrademarkChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_creation() {
        let checker = TrademarkChecker::new();
        assert!(checker.protected_trademarks.contains_key("AirPods"));
    }

    #[test]
    fn test_descriptive_use() {
        let checker = TrademarkChecker::new();
        let usage = checker.check_trademark_usage("AirPods", "descriptive use");
        assert!(usage.compliant);
    }

    #[test]
    fn test_brand_name_violation() {
        let checker = TrademarkChecker::new();
        let usage = checker.check_trademark_usage("Apple", "brand name");
        assert!(!usage.compliant);
    }

    #[test]
    fn test_scan_text() {
        let checker = TrademarkChecker::new();
        let text = "This project works with AirPods devices";
        let usages = checker.scan_text(text);
        assert!(usages.len() > 0);
    }

    #[test]
    fn test_is_descriptive_use() {
        let checker = TrademarkChecker::new();
        assert!(checker.is_descriptive_use("descriptive use of AirPods"));
        assert!(checker.is_descriptive_use("works with AirPods"));
    }

    #[test]
    fn test_is_nominative_use() {
        let checker = TrademarkChecker::new();
        assert!(checker.is_nominative_use("compatible with AirPods"));
        assert!(checker.is_nominative_use("refers to AirPods"));
    }
}
