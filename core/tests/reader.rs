//! Integration tests for `ad1-core` against spec-faithful crafted fixtures.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::cast_possible_truncation
)]
use std::fmt::Write as _;

mod common;

use ad1::{Ad1Error, Ad1Reader};
use md5::{Digest as _, Md5};
use std::path::PathBuf;

fn write_one(bytes: &[u8]) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("image.ad1");
    std::fs::write(&path, bytes).unwrap();
    (dir, path)
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

// ---- detection + headers (build step 1/2) -------------------------------

#[test]
fn rejects_non_ad1_and_shows_the_bytes() {
    let garbage = b"\x00\x00\x00\xc1\x0e not an ad1 file at all..........";
    let (_d, p) = write_one(garbage);
    match Ad1Reader::open(&p) {
        Err(Ad1Error::NotAd1(msg)) => {
            // Show-the-bytes: the diagnostic must include the offending signature.
            assert!(
                msg.contains("00") && msg.to_lowercase().contains("c1"),
                "msg={msg}"
            );
        }
        other => panic!("expected NotAd1, got {other:?}"),
    }
}

#[test]
fn refuses_adcrypt_encrypted_images() {
    let mut bytes = vec![0u8; 512];
    bytes[0..8].copy_from_slice(b"ADCRYPT\0");
    let (_d, p) = write_one(&bytes);
    match Ad1Reader::open(&p) {
        Err(Ad1Error::Unsupported(msg)) => {
            assert!(
                msg.to_lowercase().contains("encrypt") || msg.contains("ADCRYPT"),
                "msg={msg}"
            );
        }
        other => panic!("expected Unsupported(ADCRYPT), got {other:?}"),
    }
}

#[test]
fn opens_valid_image_and_parses_headers() {
    let built = common::build(common::sample_tree());
    let (_d, p) = write_one(&built.bytes);
    let r = Ad1Reader::open(&p).expect("should open crafted AD1");
    assert_eq!(r.segment_count(), 1);
    assert_eq!(r.image_version(), 4);
    assert_eq!(r.chunk_size(), common::CHUNK_SIZE);
}

// ---- file tree walk (build step 3) --------------------------------------

#[test]
fn entries_match_the_logical_tree_in_dfs_order() {
    let built = common::build(common::sample_tree());
    let (_d, p) = write_one(&built.bytes);
    let r = Ad1Reader::open(&p).unwrap();
    let got: Vec<_> = r
        .entries()
        .iter()
        .map(|e| (e.path.clone(), e.is_dir, e.size))
        .collect();
    let want: Vec<_> = built
        .expected
        .iter()
        .map(|e| (e.path.clone(), e.is_dir, e.size))
        .collect();
    assert_eq!(got, want);
}

#[test]
fn stored_hashes_are_exposed_for_files() {
    let built = common::build(common::sample_tree());
    let (_d, p) = write_one(&built.bytes);
    let r = Ad1Reader::open(&p).unwrap();
    for (entry, exp) in r.entries().iter().zip(built.expected.iter()) {
        assert_eq!(entry.md5, exp.md5, "md5 for {}", exp.path);
        assert_eq!(entry.sha1, exp.sha1, "sha1 for {}", exp.path);
    }
}

// ---- positioned decompression (build step 4) ----------------------------

fn find<'a>(r: &'a Ad1Reader, path: &str) -> &'a ad1::Ad1Entry {
    r.entries().iter().find(|e| e.path == path).expect("entry")
}

#[test]
fn read_at_whole_file_matches_data_and_stored_hash() {
    let built = common::build(common::sample_tree());
    let (_d, p) = write_one(&built.bytes);
    let r = Ad1Reader::open(&p).unwrap();
    let entry = find(&r, "root/sub/a.bin");
    let mut buf = vec![0u8; entry.size as usize];
    let n = r.read_at(entry, 0, &mut buf).unwrap();
    assert_eq!(n, entry.size as usize);
    let exp = built
        .expected
        .iter()
        .find(|e| e.path == "root/sub/a.bin")
        .unwrap();
    assert_eq!(&buf, exp.data.as_ref().unwrap());
    // Independent oracle: recomputed md5 equals the AD1-stored md5.
    assert_eq!(Some(hex(&Md5::digest(&buf))), entry.md5);
}

#[test]
fn read_at_partial_across_chunk_boundary() {
    let built = common::build(common::sample_tree());
    let (_d, p) = write_one(&built.bytes);
    let r = Ad1Reader::open(&p).unwrap();
    let entry = find(&r, "root/sub/a.bin");
    let exp = built
        .expected
        .iter()
        .find(|e| e.path == "root/sub/a.bin")
        .unwrap();
    let full = exp.data.as_ref().unwrap();
    // Read 100 bytes straddling the 64 KiB chunk boundary.
    let off = 65_500u64;
    let mut buf = vec![0u8; 100];
    let n = r.read_at(entry, off, &mut buf).unwrap();
    assert_eq!(n, 100);
    assert_eq!(&buf[..], &full[off as usize..off as usize + 100]);
}

#[test]
fn read_at_past_end_returns_zero() {
    let built = common::build(common::sample_tree());
    let (_d, p) = write_one(&built.bytes);
    let r = Ad1Reader::open(&p).unwrap();
    let entry = find(&r, "root/hello.txt");
    let mut buf = [0u8; 16];
    let n = r.read_at(entry, 9999, &mut buf).unwrap();
    assert_eq!(n, 0);
}

#[test]
fn read_at_small_file_exact() {
    let built = common::build(common::sample_tree());
    let (_d, p) = write_one(&built.bytes);
    let r = Ad1Reader::open(&p).unwrap();
    let entry = find(&r, "root/hello.txt");
    let mut buf = vec![0u8; entry.size as usize];
    let n = r.read_at(entry, 0, &mut buf).unwrap();
    assert_eq!(&buf[..n], b"Hello, AD1!\n");
}
