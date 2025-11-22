//! Doctor report formatting and display

use super::{CheckResult, CheckStatus};
use colored::Colorize;

/// Doctor diagnostic report
#[derive(Debug, Default)]
pub struct DoctorReport {
    /// All check results
    checks: Vec<CheckResult>,
}

impl DoctorReport {
    /// Create a new empty report
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a check result to the report
    pub fn add_check(&mut self, check: CheckResult) {
        self.checks.push(check);
    }

    /// Check if there are any critical failures
    pub fn has_critical_failures(&self) -> bool {
        self.checks
            .iter()
            .any(|c| c.status == CheckStatus::Critical)
    }

    /// Get summary statistics
    pub fn summary(&self) -> ReportSummary {
        let mut summary = ReportSummary::default();

        for check in &self.checks {
            match check.status {
                CheckStatus::Pass => summary.passed += 1,
                CheckStatus::Warning => summary.warnings += 1,
                CheckStatus::Fail => summary.failed += 1,
                CheckStatus::Critical => summary.critical += 1,
            }
        }

        summary.total = self.checks.len();
        summary
    }

    /// Display the report to stdout
    pub fn display(&self) {
        // Display each check
        for check in &self.checks {
            self.display_check(check);
        }

        println!();

        // Display summary
        self.display_summary();
    }

    fn display_check(&self, check: &CheckResult) {
        let (icon, status_text, color_fn): (&str, &str, fn(&str) -> colored::ColoredString) =
            match check.status {
                CheckStatus::Pass => ("✓", "PASS", |s| s.green()),
                CheckStatus::Warning => ("⚠", "WARN", |s| s.yellow()),
                CheckStatus::Fail => ("✗", "FAIL", |s| s.red()),
                CheckStatus::Critical => ("✗", "CRIT", |s| s.bright_red().bold()),
            };

        println!(
            "{} {} {}",
            icon,
            color_fn(&format!("[{:^4}]", status_text)),
            check.name.bold()
        );
        println!("  {}", check.message.dimmed());

        if let Some(suggestion) = &check.suggestion {
            println!("  {} {}", "→".cyan(), suggestion.cyan());
        }

        println!();
    }

    fn display_summary(&self) {
        let summary = self.summary();

        println!("{}", "=".repeat(60).dimmed());
        println!("{}", "Summary".bold());
        println!("{}", "=".repeat(60).dimmed());

        println!(
            "  Total checks:      {}",
            summary.total.to_string().bold()
        );
        println!(
            "  {} Passed:          {}",
            "✓".green(),
            summary.passed.to_string().green()
        );

        if summary.warnings > 0 {
            println!(
                "  {} Warnings:        {}",
                "⚠".yellow(),
                summary.warnings.to_string().yellow()
            );
        }

        if summary.failed > 0 {
            println!(
                "  {} Failed:          {}",
                "✗".red(),
                summary.failed.to_string().red()
            );
        }

        if summary.critical > 0 {
            println!(
                "  {} Critical:        {}",
                "✗".bright_red().bold(),
                summary.critical.to_string().bright_red().bold()
            );
        }

        println!();

        // Overall status
        if summary.critical > 0 {
            println!(
                "{}",
                "❌ Critical issues found. xcargo may not function correctly."
                    .bright_red()
                    .bold()
            );
            println!("{}", "   Please address the issues above.".red());
        } else if summary.failed > 0 {
            println!(
                "{}",
                "⚠️  Some checks failed. Some features may not work.".yellow()
            );
            println!("{}", "   Review the issues above.".yellow());
        } else if summary.warnings > 0 {
            println!(
                "{}",
                "✓ System is functional. Some optional features unavailable."
                    .yellow()
            );
        } else {
            println!(
                "{}",
                "✓ All checks passed! Your system is ready for cross-compilation."
                    .green()
                    .bold()
            );
        }
    }
}

/// Summary statistics for the report
#[derive(Debug, Default)]
pub struct ReportSummary {
    /// Total number of checks
    pub total: usize,
    /// Number of passed checks
    pub passed: usize,
    /// Number of warnings
    pub warnings: usize,
    /// Number of failed checks
    pub failed: usize,
    /// Number of critical failures
    pub critical: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctor::CheckResult;

    #[test]
    fn test_report_new() {
        let report = DoctorReport::new();
        assert_eq!(report.checks.len(), 0);
    }

    #[test]
    fn test_report_add_check() {
        let mut report = DoctorReport::new();
        report.add_check(CheckResult::pass("test", "success"));
        assert_eq!(report.checks.len(), 1);
    }

    #[test]
    fn test_report_has_critical_failures() {
        let mut report = DoctorReport::new();
        assert!(!report.has_critical_failures());

        report.add_check(CheckResult::pass("test1", "ok"));
        assert!(!report.has_critical_failures());

        report.add_check(CheckResult::critical("test2", "error", "fix"));
        assert!(report.has_critical_failures());
    }

    #[test]
    fn test_report_summary() {
        let mut report = DoctorReport::new();

        report.add_check(CheckResult::pass("test1", "ok"));
        report.add_check(CheckResult::pass("test2", "ok"));
        report.add_check(CheckResult::warning("test3", "warn", "fix"));
        report.add_check(CheckResult::fail("test4", "fail", "fix"));
        report.add_check(CheckResult::critical("test5", "crit", "urgent"));

        let summary = report.summary();

        assert_eq!(summary.total, 5);
        assert_eq!(summary.passed, 2);
        assert_eq!(summary.warnings, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.critical, 1);
    }

    #[test]
    fn test_report_display() {
        let mut report = DoctorReport::new();
        report.add_check(CheckResult::pass("test", "success"));

        // Should not panic
        report.display();
    }
}
