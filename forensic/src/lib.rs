//! `ad1-forensic` — anomaly auditor for **AccessData AD1** logical images.
//!
//! Emits graded [`forensicnomicon::report::Finding`]s for AD1-specific anomalies:
//! per-file stored-vs-computed MD5/SHA1 mismatches (tamper), tree/metadata
//! inconsistencies, `ADCRYPT` (encrypted) images, segment-chain gaps, and
//! oversize/lying length fields. Per the fleet principle it may read the AD1
//! structure *lower-level* than `ad1-core` exposes where the audit needs raw
//! layout the reader normalizes away.
//!
//! STATUS: **SCAFFOLD ONLY** — see `../HANDOFF.md`.

#![forbid(unsafe_code)]

/// Anomaly kinds the AD1 auditor reports (placeholder — expand per HANDOFF.md).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Ad1Anomaly {
    /// A file's stored hash does not match the recomputed hash (tamper signal).
    StoredHashMismatch,
    /// The image is `ADCRYPT` (encrypted) — content cannot be read.
    Encrypted,
    /// A referenced segment (`.adN`) is missing from the chain.
    MissingSegment,
}

/// Audit an opened AD1 image and return findings. NOT YET IMPLEMENTED.
///
/// # Errors
/// Scaffold — returns an empty finding set.
pub fn audit(_image: &ad1::Ad1Reader) -> Vec<Ad1Anomaly> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anomaly_kinds_exist() {
        assert_ne!(Ad1Anomaly::Encrypted, Ad1Anomaly::MissingSegment);
    }
}
