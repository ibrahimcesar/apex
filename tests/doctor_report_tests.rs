// Doctor report and diagnostic tests
// Tests the doctor report formatting and summary logic

use xcargo::doctor::{CheckResult, CheckStatus, DoctorReport};

#[test]
fn test_empty_report() {
    let report = DoctorReport::new();
    let summary = report.summary();

    assert_eq!(summary.total, 0);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.warnings, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.critical, 0);
}

#[test]
fn test_report_with_passing_checks() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Test 1".to_string(),
        status: CheckStatus::Pass,
        message: "OK".to_string(),
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Test 2".to_string(),
        status: CheckStatus::Pass,
        message: "OK".to_string(),
        suggestion: None,
    });

    let summary = report.summary();
    assert_eq!(summary.total, 2);
    assert_eq!(summary.passed, 2);
    assert_eq!(summary.warnings, 0);
    assert_eq!(summary.failed, 0);
}

#[test]
fn test_report_with_warnings() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Warning Check".to_string(),
        status: CheckStatus::Warning,
        message: "Minor issue - Details here".to_string(),
        suggestion: Some("Fix this".to_string()),
    });

    let summary = report.summary();
    assert_eq!(summary.total, 1);
    assert_eq!(summary.warnings, 1);
    assert!(!report.has_critical_failures());
}

#[test]
fn test_report_with_failures() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Fail Check".to_string(),
        status: CheckStatus::Fail,
        message: "Something failed".to_string(),
        
        suggestion: None,
    });

    let summary = report.summary();
    assert_eq!(summary.total, 1);
    assert_eq!(summary.failed, 1);
    assert!(!report.has_critical_failures());
}

#[test]
fn test_report_with_critical_failures() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Critical Check".to_string(),
        status: CheckStatus::Critical,
        message: "Critical failure - System cannot function".to_string(),
        suggestion: Some("Reinstall".to_string()),
    });

    let summary = report.summary();
    assert_eq!(summary.total, 1);
    assert_eq!(summary.critical, 1);
    assert!(report.has_critical_failures());
}

#[test]
fn test_report_mixed_statuses() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Pass".to_string(),
        status: CheckStatus::Pass,
        message: "OK".to_string(),
        
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Warning".to_string(),
        status: CheckStatus::Warning,
        message: "Warning".to_string(),
        
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Fail".to_string(),
        status: CheckStatus::Fail,
        message: "Failed".to_string(),
        
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Critical".to_string(),
        status: CheckStatus::Critical,
        message: "Critical".to_string(),
        
        suggestion: None,
    });

    let summary = report.summary();
    assert_eq!(summary.total, 4);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.warnings, 1);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.critical, 1);
    assert!(report.has_critical_failures());
}

#[test]
fn test_check_result_with_message() {
    let check = CheckResult {
        name: "Detailed Check".to_string(),
        status: CheckStatus::Pass,
        message: "Success - Additional details".to_string(),
        suggestion: None,
    };

    assert_eq!(check.name, "Detailed Check");
    assert_eq!(check.status, CheckStatus::Pass);
    assert!(!check.message.is_empty());
    assert!(check.suggestion.is_none());
}

#[test]
fn test_check_result_with_suggestion() {
    let check = CheckResult {
        name: "Check with Suggestion".to_string(),
        status: CheckStatus::Warning,
        message: "Minor issue".to_string(),
        
        suggestion: Some("Try this fix".to_string()),
    };

    assert_eq!(check.status, CheckStatus::Warning);
    assert!(check.suggestion.is_some());
    assert_eq!(check.suggestion.unwrap(), "Try this fix");
}

#[test]
fn test_check_status_equality() {
    assert_eq!(CheckStatus::Pass, CheckStatus::Pass);
    assert_ne!(CheckStatus::Pass, CheckStatus::Fail);
    assert_ne!(CheckStatus::Warning, CheckStatus::Critical);
}

#[test]
fn test_report_display() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Display Test".to_string(),
        status: CheckStatus::Pass,
        message: "OK".to_string(),
        
        suggestion: None,
    });

    // Just verify display doesn't panic
    report.display();
}

#[test]
fn test_multiple_critical_failures() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Critical 1".to_string(),
        status: CheckStatus::Critical,
        message: "Error 1".to_string(),
        
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Critical 2".to_string(),
        status: CheckStatus::Critical,
        message: "Error 2".to_string(),
        
        suggestion: None,
    });

    assert!(report.has_critical_failures());
    let summary = report.summary();
    assert_eq!(summary.critical, 2);
}

#[test]
fn test_no_critical_failures_with_other_statuses() {
    let mut report = DoctorReport::new();

    report.add_check(CheckResult {
        name: "Pass".to_string(),
        status: CheckStatus::Pass,
        message: "OK".to_string(),
        
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Warning".to_string(),
        status: CheckStatus::Warning,
        message: "Warning".to_string(),
        
        suggestion: None,
    });

    report.add_check(CheckResult {
        name: "Fail".to_string(),
        status: CheckStatus::Fail,
        message: "Failed".to_string(),
        
        suggestion: None,
    });

    assert!(!report.has_critical_failures());
}

#[test]
fn test_summary_with_only_warnings() {
    let mut report = DoctorReport::new();

    for i in 0..5 {
        report.add_check(CheckResult {
            name: format!("Warning {}", i),
            status: CheckStatus::Warning,
            message: "Warning".to_string(),
            
            suggestion: None,
        });
    }

    let summary = report.summary();
    assert_eq!(summary.total, 5);
    assert_eq!(summary.warnings, 5);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.critical, 0);
}

#[test]
fn test_check_result_debug_format() {
    let check = CheckResult {
        name: "Debug Test".to_string(),
        status: CheckStatus::Pass,
        message: "OK".to_string(),
        
        suggestion: None,
    };

    let debug_str = format!("{:?}", check);
    assert!(debug_str.contains("Debug Test"));
}

#[test]
fn test_report_default_creation() {
    let report = DoctorReport::default();
    assert_eq!(report.summary().total, 0);
}
