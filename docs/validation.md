# Validation

How `ad1-core`/`ad1-forensic` correctness is established, graded by **who
confirms it** (most-trustworthy first).

## Summary

| Aspect | Tier | Status |
|---|---|---|
| zlib chunk decompression + logical addressing | 2 | ✅ recomputed MD5/SHA1 match independent (flate2 + RustCrypto) ground truth |
| Structural offsets (headers, item, metadata, chunk table) | 3 | ✅ crafted fixture from the al3ks1s C spec; ⏳ awaiting tier-1 real-data confirmation |
| Robustness / panic-freedom on malformed input | 2 | ✅ 18 malformed-input tests + two cargo-fuzz targets (3 real bugs found & fixed) |
| End-to-end vs FTK Imager / `ad1extract` on `userbss.ad1` | 1 | ⏳ **pending** — see below |

## Tier 2 — independent oracle on crafted data

The test fixtures are built by an encoder that follows the al3ks1s/AD1-tools
layout, but the file **data** is compressed by [`flate2`](https://crates.io/crates/flate2)
and the **stored hashes** are computed by RustCrypto
[`md-5`](https://crates.io/crates/md-5)/[`sha1`](https://crates.io/crates/sha1) —
libraries independent of `ad1-core`. So when the reader decompresses a file and
its recomputed MD5 equals the stored MD5, the chunk table walk, cross-segment
logical addressing, and zlib inflate are confirmed against ground truth the
reader did not produce (`read_at_whole_file_matches_data_and_stored_hash`,
`reads_file_data_spanning_multiple_segments`).

The **residual tier-3 risk**: the encoder and decoder share the *structural
offsets* (both take them from the same C reference). A systematic offset error
would pass these tests. That is exactly what the tier-1 step below closes.

## Tier 2 — fuzzing

Two `cargo-fuzz` targets (`fuzz_reader`, `fuzz_forensic`) drive arbitrary bytes
through `open → entries → read_at` and `audit`. They found and fixed three real
defects: a 72 GB allocation from an attacker-controlled `segment_count`, a
`count + 1` overflow on a `u64::MAX` chunk count, and a `cur - chunk_base`
underflow on a short non-last chunk. Each has a deterministic regression test.

## Tier 1 — real-world data (pending)

The authoritative check is reconciling extraction + hashes against an independent
implementation on a **real** AD1: `userbss.ad1` from the
[2025 Magnet Virtual Summit CTF](https://cfreds.nist.gov/all/Hexordia/2025MVSCTF)
(Hexordia / Kevin Pagano), versus **FTK Imager**'s stored hashes and
`ad1extract`/`ad1check`.

**Status:** not yet downloaded. NIST CFReDS hosts the set only via a Google Drive
mirror that is currently anonymous-download-quota throttled (Google: "try again,
up to 24 hours"). Once retrieved, an env-gated test
(`AD1_USERBSS=/path/to/userbss.ad1`) will reconcile `ad1-core`'s file list and
recomputed hashes against the AD1's FTK-written stored hashes and report any
divergence here.
