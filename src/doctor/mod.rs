//! System diagnostics and health checks
//!
//! This module provides the `xcargo doctor` command functionality to help users
//! diagnose and troubleshoot their cross-compilation setup.

mod checks;
mod report;

pub use checks::{Check, CheckResult, CheckStatus};
pub use report::DoctorReport;

use crate::error::Result;
use crate::output::helpers;

/// Run all diagnostic checks and display the report
pub fn run() -> Result<()> {
    helpers::section("xcargo doctor - System Diagnostics");
    println!("Checking your cross-compilation environment...\n");

    let mut report = DoctorReport::new();

    // Run all checks
    report.add_check(checks::check_rustup());
    report.add_check(checks::check_cargo());
    report.add_check(checks::check_default_toolchain());
    report.add_check(checks::check_installed_targets());
    report.add_check(checks::check_zig());
    report.add_check(checks::check_docker());
    report.add_check(checks::check_podman());
    report.add_check(checks::check_common_linkers());
    report.add_check(checks::check_config_file());

    // Display the report
    report.display();

    // Return success/failure based on critical checks
    if report.has_critical_failures() {
        Err(crate::error::Error::Config(
            "Critical system checks failed. See diagnostics above.".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doctor_run() {
        // Doctor should not panic, but may return error
        let _ = run();
    }
}
