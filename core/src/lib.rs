//! `ad1` — pure-Rust reader for the **AccessData AD1** logical image container
//! (FTK Imager "Custom Content Image").
//!
//! AD1 is a *logical* file/folder container — NOT a sector-level disk image. It
//! stores a tree of files + per-file metadata (name, timestamps, attributes) +
//! the file data in zlib-compressed chunks + stored MD5/SHA1 hashes, across one
//! or more segments (`.ad1`, `.ad2`, …). So this reader exposes a **virtual
//! filesystem** (path → bytes + metadata), like a zip/tar reader — there is no
//! block device / partition / filesystem layer underneath it.
//!
//! STATUS: **SCAFFOLD ONLY** — nothing below is implemented. See
//! `../HANDOFF.md` for the format spec, oracle, and the strict-TDD build plan.

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod bytes;

use std::path::Path;

/// Marker string present in every AD1 segment header.
pub const AD1_SEGMENTED_MARKER: &[u8] = b"ADSEGMENTEDFILE\x00";

/// Errors from reading an AD1 image.
#[derive(Debug, thiserror::Error)]
pub enum Ad1Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not an AD1 image: {0}")]
    NotAd1(String),
    #[error("unsupported AD1 feature: {0}")] // e.g. ADCRYPT (encrypted)
    Unsupported(String),
    #[error("malformed AD1 structure: {0}")]
    Malformed(String),
}

/// One node in the AD1 logical file tree.
#[derive(Debug, Clone)]
pub struct Ad1Entry {
    /// Logical path within the image (POSIX-style, `/`-separated).
    pub path: String,
    /// True for a directory node (no file data).
    pub is_dir: bool,
    /// Uncompressed file size in bytes (0 for directories).
    pub size: u64,
    // TODO: timestamps, attributes, and the stored MD5/SHA1 the auditor verifies.
}

/// A reader over an AD1 logical image (its first segment plus any `.ad2`…).
pub struct Ad1Reader {
    // TODO: segment handles, the parsed file tree, the offset/chunk map.
}

impl Ad1Reader {
    /// Open an AD1 image given the path to its **first** segment (`*.ad1`);
    /// subsequent segments are discovered alongside it.
    ///
    /// # Errors
    /// Not yet implemented — always returns [`Ad1Error::Unsupported`].
    pub fn open(_first_segment: &Path) -> Result<Self, Ad1Error> {
        Err(Ad1Error::Unsupported(
            "ad1-core is a scaffold — see HANDOFF.md".into(),
        ))
    }

    /// The logical file tree (depth-first, directories before their children).
    #[must_use]
    pub fn entries(&self) -> &[Ad1Entry] {
        &[]
    }

    /// Read up to `buf.len()` decompressed bytes of `entry` starting at `offset`.
    ///
    /// # Errors
    /// Not yet implemented.
    pub fn read_at(
        &self,
        _entry: &Ad1Entry,
        _offset: u64,
        _buf: &mut [u8],
    ) -> Result<usize, Ad1Error> {
        Err(Ad1Error::Unsupported("not implemented".into()))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn marker_is_the_documented_string() {
        assert_eq!(&AD1_SEGMENTED_MARKER[..15], b"ADSEGMENTEDFILE");
    }

    #[test]
    fn open_scaffold_errors_cleanly() {
        assert!(Ad1Reader::open(Path::new("/nonexistent.ad1")).is_err());
    }
}
