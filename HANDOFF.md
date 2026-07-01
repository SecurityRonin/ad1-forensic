# `ad1-forensic` — Status

**Implemented.** A pure-Rust AD1 reader (`ad1-core`, import `use ad1::…`) and
forensic auditor (`ad1-forensic`), built with strict TDD against the AD1
reverse-engineering prior art in §2.

- **Reader** — `Ad1Reader::open` detects `ADSEGMENTEDFILE` / refuses `ADCRYPT`,
  parses the segment + logical headers, walks the file tree (iterative, cycle-
  guarded), and exposes per-entry path / size / stored MD5+SHA1 / timestamps.
  `read_at` inflates only the zlib chunks a range overlaps, across `.ad1`/`.ad2`…
  segments. No `unsafe`; panic-free on malformed input (bounds-checked reads,
  capped allocations, overflow-safe arithmetic).
- **Auditor** — `ad1_forensic::audit(path)` returns `forensicnomicon` findings:
  `AD1-HASH-MISMATCH` (stored vs recomputed MD5/SHA1), `AD1-ENCRYPTED`,
  `AD1-SEGMENT-MISSING`, `AD1-SIZE-LIE`, `AD1-UNREADABLE`.
- **Quality** — 43 tests + an env-gated tier-1 test, two `cargo-fuzz` targets
  (found & fixed 3 real bugs), 100% function coverage, `cargo deny`/clippy/fmt
  clean, CI + fuzz + release + docs workflows.
- **Tier-1 validated** — against the real 48 GiB FTK `userbss.ad1`: all 299,729
  files' recomputed MD5 **and** SHA1 match FTK's stored hashes, 0 mismatches
  (see `docs/validation.md`).

Canonical docs: [README](README.md) · [docs/format.md](docs/format.md) (on-disk
layout) · [docs/validation.md](docs/validation.md) (how correctness is graded).

## 1. Why this exists

No Rust AD1 reader existed (crates.io had nothing; libewf/TSK don't support AD1).
AD1 is FTK Imager's *logical* evidence container — a tree of files + metadata +
zlib-compressed data + stored per-file hashes, segmented like a split E01. It
lets investigators hand a *logical* package straight to artifact parsers.

## 2. References (the de-facto spec)

- **`al3ks1s/AD1-tools`** — <https://github.com/al3ks1s/AD1-tools> — the de-facto
  spec + oracle (`ad1info`/`ad1extract`/`ad1check`/`ad1mount`). The byte layout in
  `docs/format.md` is derived from its `libad1`.
- **Cerbero "AD1 Format Package"** — <https://blog.cerbero.io/ad1-format-package/>.
- **DFIRScience "What is an AD1?"** — <https://dfir.science/2021/09/What-is-an-AD1.html>.

## 3. Remaining work

- **Tier-1 validation — DONE.** Reconciled against the real 48 GiB FTK
  `userbss.ad1` (2025 Magnet Summit CTF): all 299,729 files' recomputed MD5 +
  SHA1 match FTK's stored hashes, 0 mismatches (`core/tests/tier1_real.rs`,
  env-gated on `AD1_USERBSS`; details in `docs/validation.md`).
- **issen integration (separate repo):** AD1 holds *files*, not blocks, so it is
  a **collection** format — wire it as an `issen_unpack::CollectionProvider`
  (probe magic → open via `ad1-core` → feed the file tree straight to the
  artifact parsers), not into the container→partition→filesystem disk pipeline.
- **`ad1` crate name:** re-check availability on crates.io right before publish.
- **`ADCRYPT` decryption:** out of scope for v1 (detected and refused today).
