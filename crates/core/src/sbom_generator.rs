use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub component_id: String,
    pub name: String,
    pub version: String,
    pub component_type: ComponentType,
    pub license: String,
    pub supplier: String,
    pub purl: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Library,
    Framework,
    Application,
    Container,
    OperatingSystem,
    Device,
    Firmware,
    Source,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub component_id: String,
    pub depends_on: Vec<String>,
    pub relationship: DependencyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    Depends,
    DevDependency,
    Optional,
    Transitive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBOM {
    pub sbom_id: String,
    pub version: String,
    pub created_date: String,
    pub components: Vec<Component>,
    pub dependencies: Vec<Dependency>,
    pub licenses: HashMap<String, usize>,
}

impl SBOM {
    pub fn new(sbom_id: String, version: String) -> Self {
        Self {
            sbom_id,
            version,
            created_date: "2025-11-21T00:00:00Z".to_string(),
            components: Vec::new(),
            dependencies: Vec::new(),
            licenses: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        let license_count = self.licenses.entry(component.license.clone()).or_insert(0);
        *license_count += 1;
        self.components.push(component);
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }

    pub fn get_components_by_type(&self, component_type: ComponentType) -> Vec<&Component> {
        self.components.iter().filter(|c| c.component_type == component_type).collect()
    }

    pub fn get_components_by_license(&self, license: &str) -> Vec<&Component> {
        self.components.iter().filter(|c| c.license == license).collect()
    }

    pub fn get_total_components(&self) -> usize {
        self.components.len()
    }

    pub fn get_unique_licenses(&self) -> Vec<String> {
        self.licenses.keys().cloned().collect()
    }

    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== SBOM REPORT ===\n\n");

        report.push_str(&format!("SBOM ID: {}\n", self.sbom_id));
        report.push_str(&format!("Version: {}\n", self.version));
        report.push_str(&format!("Created: {}\n\n", self.created_date));

        report.push_str(&format!("Total Components: {}\n", self.get_total_components()));
        report.push_str(&format!("Unique Licenses: {}\n\n", self.get_unique_licenses().len()));

        report.push_str("License Distribution:\n");
        for (license, count) in &self.licenses {
            report.push_str(&format!("  {}: {}\n", license, count));
        }

        report.push_str("\nComponents:\n");
        for component in &self.components {
            report.push_str(&format!("  {} v{} ({:?}) - {}\n", 
                component.name, component.version, component.component_type, component.license));
        }

        report
    }
}

impl Default for SBOM {
    fn default() -> Self {
        Self::new("sbom-1".to_string(), "1.0.0".to_string())
    }
}

pub struct SBOMGenerator {
    sboms: HashMap<String, SBOM>,
}

impl SBOMGenerator {
    pub fn new() -> Self {
        Self {
            sboms: HashMap::new(),
        }
    }

    pub fn create_sbom(&mut self, sbom_id: String, version: String) -> &mut SBOM {
        let sbom = SBOM::new(sbom_id.clone(), version);
        self.sboms.insert(sbom_id.clone(), sbom);
        self.sboms.get_mut(&sbom_id).unwrap()
    }

    pub fn get_sbom(&self, sbom_id: &str) -> Option<&SBOM> {
        self.sboms.get(sbom_id)
    }

    pub fn add_component(&mut self, sbom_id: &str, component: Component) {
        if let Some(sbom) = self.sboms.get_mut(sbom_id) {
            sbom.add_component(component);
        }
    }

    pub fn get_all_sboms(&self) -> Vec<&SBOM> {
        self.sboms.values().collect()
    }
}

impl Default for SBOMGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_sbom_generator() {
        let mut generator = SBOMGenerator::new();
        generator.create_sbom("sbom-1".to_string(), "1.0.0".to_string());
        assert_eq!(generator.get_all_sboms().len(), 1);
    }
}
