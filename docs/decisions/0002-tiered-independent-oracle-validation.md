# 0002 — Tiered, independent-oracle validation

**Status:** Accepted

## Context

We authored both the reader and its tests. A parser that emits values and is
"validated" only by fixtures we also authored can ship green while wrong — the
LZNT1 trap: a fixture encoded to the same bug the decoder has agrees with it.
The byte offsets came from one C reference (`al3ks1s/AD1-tools`), so an offset we
misread would be misread identically by a fixture builder that shares them.

## Decision

Grade every correctness claim by *who confirms it* and require an oracle
independent of `ad1-core` for value-producing paths:

- **Tier 2 (crafted fixtures):** the `testfix` builder compresses file data with
  **flate2** and computes stored hashes with **RustCrypto md-5/sha1** —
  libraries independent of the reader. When the reader decompresses a file and
  its recomputed hash equals the stored hash, the chunk table, logical
  addressing, and inflate are confirmed against ground truth the reader did not
  produce. Residual: encoder and decoder share structural offsets.
- **Tier 2 (fuzzing):** two `cargo-fuzz` targets for robustness (a property, not
  a value to oracle-check).
- **Tier 1 (real data):** reconcile against **FTK Imager**'s stored hashes in a
  real AD1 (`userbss.ad1`, 2025 Magnet Summit CTF). FTK is a fully independent
  implementation, so agreement confirms the structural offsets tier-2 could not.

## Consequences

- Documented honestly in `docs/validation.md`, labelled by tier.
- Tier-1 closed the residual: all 299,729 files reconciled (MD5 + SHA1), 0
  mismatches, against FTK's hashes.
- The real 48 GiB image is gitignored + env-gated (`AD1_USERBSS`), so CI stays
  green without it.

## Alternatives considered

- **Self-encoded round-trip only** — rejected: the LZNT1 trap; proves
  self-consistency, not correctness.
- **Ship on tier-2 alone** — acceptable interim, but the shared-offset residual
  made tier-1 worth pursuing; done once the real image was obtainable.
