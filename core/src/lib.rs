//! `ad1` — pure-Rust reader for the **AccessData AD1** logical image container.
//!
//! STATUS: RED — public API surface only; behavior is stubbed so the integration
//! tests fail until the reader is implemented (see `../HANDOFF.md`).

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
#![allow(clippy::missing_const_for_fn, unused_variables)]

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
    #[error("unsupported AD1 feature: {0}")]
    Unsupported(String),
    #[error("malformed AD1 structure: {0}")]
    Malformed(String),
}

/// One node in the AD1 logical file tree.
#[derive(Debug, Clone)]
pub struct Ad1Entry {
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub item_type: u32,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub modified: Option<String>,
    pub accessed: Option<String>,
    pub changed: Option<String>,
    pub(crate) zlib_addr: u64,
}

/// A reader over an AD1 logical image (its first segment plus any `.ad2`…).
#[derive(Debug)]
pub struct Ad1Reader {
    entries: Vec<Ad1Entry>,
}

impl Ad1Reader {
    /// Open an AD1 image — NOT YET IMPLEMENTED.
    ///
    /// # Errors
    /// Always returns [`Ad1Error::Unsupported`] in this RED stub.
    pub fn open(first_segment: &Path) -> Result<Self, Ad1Error> {
        Err(Ad1Error::Unsupported("ad1-core not implemented".into()))
    }

    #[must_use]
    pub fn entries(&self) -> &[Ad1Entry] {
        &self.entries
    }

    #[must_use]
    pub fn image_version(&self) -> u32 {
        0
    }

    #[must_use]
    pub fn chunk_size(&self) -> u32 {
        0
    }

    #[must_use]
    pub fn segment_count(&self) -> u32 {
        0
    }

    /// Read decompressed bytes — NOT YET IMPLEMENTED.
    ///
    /// # Errors
    /// Never errors in this RED stub; always reports 0 bytes read.
    pub fn read_at(
        &self,
        entry: &Ad1Entry,
        offset: u64,
        buf: &mut [u8],
    ) -> Result<usize, Ad1Error> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_is_the_documented_string() {
        assert_eq!(&AD1_SEGMENTED_MARKER[..15], b"ADSEGMENTEDFILE");
    }
}
