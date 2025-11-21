use librepods_core::legal_scan::*;
use librepods_core::dmca_scanner::DMCAScanner;
use librepods_core::gpl_checker::GPLChecker;
use librepods_core::trademark_checker::TrademarkChecker;
use librepods_core::firmware_security::FirmwareSecurityAnalyzer;

#[test]
fn test_phase2_complete_legal_scan() {
    let mut scan_result = LegalScanResult::new();

    scan_result.add_finding("Upstream repository: kavishdevar/librepods".to_string());
    scan_result.add_finding("License: GPL-3.0".to_string());
    scan_result.add_finding("No DMCA notices detected".to_string());

    assert_eq!(scan_result.findings.len(), 3);
    assert!(scan_result.is_safe_to_distribute);
    assert_eq!(scan_result.overall_risk, LegalRisk::None);
}

#[test]
fn test_dmca_scanner_integration() {
    let scanner = DMCAScanner::new();

    assert!(scanner.is_repository_safe("librepods-ng"));
    assert!(scanner.is_repository_safe("kavishdevar/librepods"));

    let notices = scanner.scan_repository("test-repo", "A normal repository");
    assert_eq!(notices.len(), 0);

    let notices_with_keyword = scanner.scan_repository("test-repo", "This contains proprietary code");
    assert!(notices_with_keyword.len() > 0);
}

#[test]
fn test_gpl_checker_integration() {
    let checker = GPLChecker::new();

    let gpl_header = "// GPL-3.0-or-later WITH Classpath-exception-2.0\n// Copyright 2025";
    let header = checker.check_license_header("core.rs", gpl_header);
    assert!(header.has_header);
    assert_eq!(header.license_type, LicenseType::GPL3);

    assert!(checker.check_license_compatibility("GPL-3.0"));
    assert!(checker.check_license_compatibility("MIT"));
    assert!(!checker.check_license_compatibility("Proprietary"));
}

#[test]
fn test_trademark_checker_integration() {
    let checker = TrademarkChecker::new();

    let usage = checker.check_trademark_usage("AirPods", "descriptive use");
    assert!(usage.compliant);

    let usage_brand = checker.check_trademark_usage("Apple", "brand name");
    assert!(!usage_brand.compliant);

    let text = "This project works with AirPods devices";
    let usages = checker.scan_text(text);
    assert!(usages.len() > 0);
}

#[test]
fn test_firmware_security_analyzer_integration() {
    let analyzer = FirmwareSecurityAnalyzer::new();

    let safe_firmware = "Normal firmware data";
    let analysis = analyzer.analyze_firmware(safe_firmware);
    assert!(!analysis.is_apple_firmware);
    assert!(!analysis.is_nda_protected);
    assert!(!analysis.is_proprietary);
    assert!(analyzer.is_safe_to_distribute(&analysis));

    let apple_firmware = "Copyright Apple Inc. 2025";
    let analysis_apple = analyzer.analyze_firmware(apple_firmware);
    assert!(analysis_apple.is_apple_firmware);
    assert_eq!(analysis_apple.risk_level, LegalRisk::High);

    let nda_firmware = "This is NDA protected material";
    let analysis_nda = analyzer.analyze_firmware(nda_firmware);
    assert!(analysis_nda.is_nda_protected);
    assert_eq!(analysis_nda.risk_level, LegalRisk::Critical);
}

#[test]
fn test_legal_scan_with_dmca_notice() {
    let mut scan = LegalScanResult::new();

    let notice = DMCANotice {
        date: "2025-11-21".to_string(),
        repository: "test/repo".to_string(),
        reason: "Copyright claim".to_string(),
        status: "Active".to_string(),
    };

    scan.add_dmca_notice(notice);

    assert_eq!(scan.dmca_notices.len(), 1);
    assert_eq!(scan.overall_risk, LegalRisk::Critical);
    assert!(!scan.is_safe_to_distribute);
}

#[test]
fn test_legal_scan_with_gpl_violations() {
    let mut scan = LegalScanResult::new();

    let violation = GPLViolation {
        file: "test.rs".to_string(),
        violation_type: "Missing header".to_string(),
        description: "GPL-3.0 header missing".to_string(),
        severity: LegalRisk::High,
    };

    scan.add_gpl_violation(violation);

    assert_eq!(scan.gpl_violations.len(), 1);
    assert_eq!(scan.overall_risk, LegalRisk::High);
}

#[test]
fn test_legal_scan_with_trademark_issues() {
    let mut scan = LegalScanResult::new();

    let usage = TrademarkUsage {
        term: "AirPods".to_string(),
        context: "descriptive use".to_string(),
        compliant: true,
        reason: "Used descriptively only".to_string(),
    };

    scan.add_trademark_usage(usage);

    assert_eq!(scan.trademark_usage.len(), 1);
    assert!(scan.is_safe_to_distribute);
}

#[test]
fn test_legal_scan_with_firmware_analysis() {
    let mut scan = LegalScanResult::new();

    let analysis = FirmwareAnalysis {
        is_proprietary: false,
        is_nda_protected: false,
        is_apple_firmware: false,
        risk_level: LegalRisk::None,
    };

    scan.set_firmware_analysis(analysis);

    assert!(scan.is_safe_to_distribute);
    assert_eq!(scan.overall_risk, LegalRisk::None);
}

#[test]
fn test_legal_scan_critical_firmware() {
    let mut scan = LegalScanResult::new();

    let critical_analysis = FirmwareAnalysis {
        is_proprietary: true,
        is_nda_protected: true,
        is_apple_firmware: true,
        risk_level: LegalRisk::Critical,
    };

    scan.set_firmware_analysis(critical_analysis);

    assert!(!scan.is_safe_to_distribute);
    assert_eq!(scan.overall_risk, LegalRisk::Critical);
}

#[test]
fn test_license_consistency_check() {
    let mut scan = LegalScanResult::new();

    for i in 0..5 {
        scan.add_license_header(LicenseHeader {
            file_path: format!("file{}.rs", i),
            has_header: true,
            license_type: LicenseType::GPL3,
            year: Some("2025".to_string()),
            author: Some("Test".to_string()),
        });
    }

    assert_eq!(scan.get_license_consistency(), 100.0);
}

#[test]
fn test_legal_scan_report_generation() {
    let scan = LegalScanResult::new();
    let report = scan.generate_report();

    assert!(report.contains("LEGAL SCAN REPORT"));
    assert!(report.contains("Overall Risk Level"));
    assert!(report.contains("Safe to Distribute"));
}

#[test]
fn test_dmca_scanner_report() {
    let scanner = DMCAScanner::new();
    let report = scanner.generate_dmca_report("librepods-ng", "AirPods control framework");

    assert!(report.contains("DMCA SCAN REPORT"));
    assert!(report.contains("librepods-ng"));
}

#[test]
fn test_gpl_checker_report() {
    let checker = GPLChecker::new();
    let files = vec![
        LicenseHeader {
            file_path: "file1.rs".to_string(),
            has_header: true,
            license_type: LicenseType::GPL3,
            year: Some("2025".to_string()),
            author: None,
        },
        LicenseHeader {
            file_path: "file2.rs".to_string(),
            has_header: false,
            license_type: LicenseType::Unknown,
            year: None,
            author: None,
        },
    ];

    let report = checker.generate_gpl_report(files);
    assert!(report.contains("GPL COMPLIANCE REPORT"));
    assert!(report.contains("50.0%"));
}

#[test]
fn test_trademark_checker_report() {
    let checker = TrademarkChecker::new();
    let usages = vec![TrademarkUsage {
        term: "AirPods".to_string(),
        context: "descriptive use".to_string(),
        compliant: true,
        reason: "Compliant usage".to_string(),
    }];

    let report = checker.generate_trademark_report(usages);
    assert!(report.contains("TRADEMARK COMPLIANCE REPORT"));
    assert!(report.contains("AirPods"));
}

#[test]
fn test_firmware_security_report() {
    let analyzer = FirmwareSecurityAnalyzer::new();
    let analysis = FirmwareAnalysis {
        is_proprietary: false,
        is_nda_protected: false,
        is_apple_firmware: false,
        risk_level: LegalRisk::None,
    };

    let report = analyzer.generate_firmware_report(&analysis);
    assert!(report.contains("FIRMWARE SECURITY ANALYSIS"));
    assert!(report.contains("Safe to Distribute: true"));
}

#[test]
fn test_legal_risk_ordering() {
    assert!(LegalRisk::None < LegalRisk::Low);
    assert!(LegalRisk::Low < LegalRisk::Medium);
    assert!(LegalRisk::Medium < LegalRisk::High);
    assert!(LegalRisk::High < LegalRisk::Critical);
}

#[test]
fn test_complete_phase2_workflow() {
    let mut scan = LegalScanResult::new();

    let dmca_scanner = DMCAScanner::new();
    let gpl_checker = GPLChecker::new();
    let trademark_checker = TrademarkChecker::new();
    let firmware_analyzer = FirmwareSecurityAnalyzer::new();

    assert!(dmca_scanner.is_repository_safe("librepods-ng"));

    let gpl_header = "// GPL-3.0\n// Copyright 2025";
    let header = gpl_checker.check_license_header("core.rs", gpl_header);
    scan.add_license_header(header);

    let trademark_usage = trademark_checker.check_trademark_usage("AirPods", "descriptive use");
    scan.add_trademark_usage(trademark_usage);

    let firmware_analysis = firmware_analyzer.analyze_firmware("Normal firmware");
    scan.set_firmware_analysis(firmware_analysis);

    assert!(scan.is_safe_to_distribute);
    assert_eq!(scan.overall_risk, LegalRisk::None);

    let report = scan.generate_report();
    assert!(report.contains("LEGAL SCAN REPORT"));
}
