//! `ad1-forensic` — anomaly auditor for **AccessData AD1** logical images.
//!
//! STATUS: RED — public API surface only; `audit` returns no findings until the
//! auditor is implemented (see `../HANDOFF.md`).

#![forbid(unsafe_code)]
#![allow(unused_variables)]

use forensicnomicon::report::Finding;
use std::path::Path;

/// Audit an AD1 image (given its first segment) and return forensic findings.
///
/// NOT YET IMPLEMENTED — returns an empty finding set in this RED stub.
#[must_use]
pub fn audit(first_segment: &Path) -> Vec<Finding> {
    Vec::new()
}
