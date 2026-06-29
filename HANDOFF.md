# `ad1-forensic` — Implementation Handoff

**Status: SCAFFOLD ONLY.** This repo currently holds a buildable Cargo workspace
with two stub crates (`ad1-core`, `ad1-forensic`) and this document. **Nothing is
implemented.** A future session should read this top-to-bottom, then build per the
plan. Everything below is grounded in the AD1 reverse-engineering prior art cited
in §2 — verify against it, don't trust this doc's recollection of byte offsets.

---

## 0. Current state — read first

```
ad1-forensic/
├── Cargo.toml            # workspace (members: core, forensic); pure-Rust deps only
├── rust-toolchain.toml   # pinned 1.96.0 (dev); MSRV promise = 1.85 (in Cargo.toml)
├── .gitignore            # /target, Cargo.lock, large *.ad* test images
├── core/                 # crate `ad1-core`, [lib] name = "ad1"  → `use ad1::…`
│   └── src/lib.rs        # Ad1Reader / Ad1Entry / Ad1Error STUBS (open() returns Unsupported)
├── forensic/             # crate `ad1-forensic` (depends on ad1-core)
│   └── src/lib.rs        # Ad1Anomaly + audit() STUB
└── HANDOFF.md            # this file
```
`cargo build` should already be green (stubs only). First task: `cargo build && cargo test`
to confirm the scaffold compiles, then start §3.

## 1. Why this exists

- **No Rust AD1 reader exists.** crates.io has nothing for AccessData AD1
  (searched `ad1`, `accessdata`, `ftk imager`, `logical image`, `custom-content-image`).
  libewf does NOT support AD1 either, so TSK/BitCurator can't open it.
- **We have real AD1 test data:** `userbss.ad1` in the **2025 Magnet Virtual Summit
  CTF** set (Hexordia / Kevin Pagano), hosted on NIST CFReDS
  (<https://cfreds.nist.gov/all/Hexordia/2025MVSCTF>). Provenance is recorded in
  `issen/tests/data/README.md` → `magnet-summit-2025-ctf/`.
- **Fleet fit:** issen reads E01/raw/VMDK/VHD/VHDX/QCOW2/ISO containers but not
  AD1. AD1 lets investigators hand issen a *logical* evidence package directly.

## 2. The AD1 format (and the de-facto spec)

**AD1 is a LOGICAL container — a tree of files/folders + metadata, NOT a
sector-level disk image.** This is the single most important architectural fact
(see §5 for what it implies). It stores: the file hierarchy, per-file metadata
(name, timestamps, attributes), the file DATA in **zlib-compressed chunks**, and a
**stored MD5/SHA1 per file** (which the forensic auditor verifies). It is
**segmented** (`name.ad1`, `name.ad2`, …) like a split E01.

Reverse-engineering references (the spec is the *source* of these — read them):
- **`al3ks1s/AD1-tools`** — <https://github.com/al3ks1s/AD1-tools> — the de-facto
  spec + oracle: `ad1info` (header + file tree), `ad1extract` (extract files),
  `ad1check` (verify stored hashes), `ad1mount` (FUSE). Read its source for the
  segment header, the logical-image header, the tree-node layout, the chunk table,
  and the hash fields. **Validate `ad1-core` against `ad1extract`/`ad1check`.**
- **Cerbero "AD1 Format Package"** — <https://blog.cerbero.io/ad1-format-package/>
  — a second independent parser; useful cross-check.
- **DFIRScience "What is an AD1?"** — <https://dfir.science/2021/09/What-is-an-AD1.html>
  — format overview + the logical-not-physical clarification.

Known byte-level facts to confirm against the source above (do NOT code from this
list alone):
- Segment header contains the marker string **`ADSEGMENTEDFILE\x00`** (see
  `AD1_SEGMENTED_MARKER` in the stub). Some files begin `00 00 00 C1 0E …`.
- After the segmented-file header comes a **logical image header**, then a tree of
  file/folder nodes; each file node points at zlib-compressed data chunks and
  carries the stored MD5/SHA1.
- AD1 has **format versions** (commonly v3 and v4) with differing structures —
  detect and handle both, or fail loud on an unrecognized version (Show-the-bytes).
- **`ADCRYPT`** is an **encrypted** variant — detect it and **refuse** (return
  `Ad1Error::Unsupported`), never emit garbage. Decryption is out of scope v1.

## 3. `ad1-core` scope (the reader)

Public API (sketched in `core/src/lib.rs`):
- `Ad1Reader::open(first_segment: &Path)` — open the `.ad1` (+ discover `.ad2…`),
  parse the header chain + the full file tree. Bound RAM: keep the tree + a
  chunk-offset map, NOT the decompressed files.
- `entries() -> &[Ad1Entry]` — the logical tree (path, is_dir, size, + metadata
  and the stored hashes to add to `Ad1Entry`).
- `read_at(entry, offset, buf)` — positioned, decompress only the zlib chunks the
  range overlaps (selective, like the issen container readers — don't inflate a
  whole file to read a slice).

Build order (strict TDD, separate RED/GREEN commits each):
1. Segment header + marker detect + version dispatch (RED: reject non-AD1).
2. Logical-image header + the file-tree walk → `entries()` (oracle: `ad1info` tree).
3. Per-file zlib chunk map + `read_at` (oracle: `ad1extract` byte-identical files).
4. Multi-segment chaining (`.ad2…`).
5. `ADCRYPT` detection → `Unsupported` (no garbage).

## 4. `ad1-forensic` scope (the auditor)

Emit `forensicnomicon::report::Finding`s (the fleet report model — see issen
CLAUDE.md "The Reporting Model"). Anomaly codes (scheme-prefixed SCREAMING-KEBAB,
a published contract): `AD1-HASH-MISMATCH` (stored vs recomputed MD5/SHA1 — a
tamper signal), `AD1-ENCRYPTED` (ADCRYPT), `AD1-SEGMENT-MISSING`,
`AD1-TREE-CYCLE`, `AD1-SIZE-LIE` (chunk lengths vs declared size), etc.
Per the fleet principle (issen CLAUDE.md): **`ad1-forensic` is not required to
route through `ad1-core`** — it may read the raw segment bytes to see slack /
malformed nodes the reader normalizes away. Start on `ad1-core`, drop lower where
the audit needs it.

## 5. issen integration — AD1 is a COLLECTION, not a disk image

Because AD1 holds *files*, not blocks, it does **NOT** plug into the disk pipeline
(container → partition → filesystem). It is a **collection format**, like UAC /
Velociraptor / KAPE-zip. So integrate it as an **`issen_unpack::CollectionProvider`**
(in `issen-archive` or a new `issen-ad1` wrapper crate): probe the AD1 magic,
open via `ad1-core`, present the file tree, and feed those files **directly** to
the artifact parsers (no NTFS/ext4 navigation needed — AD1 already *is* the files).
Contrast with the E01/zran work: that gives a sector stream that needs a
filesystem reader on top; AD1 skips straight to files.

## 6. Naming + fleet standards (binding — from issen CLAUDE.md)

- **Pattern A** single-format repo: `ad1-core` (reader) + `ad1-forensic` (analyzer),
  repo `ad1-forensic`. The bare name `ad1` is free → reader PACKAGE is `ad1-core`
  with `[lib] name = "ad1"` (import `use ad1::…`); confirm `ad1` is still free
  before publishing.
- **Paranoid Gatekeeper** (AD1 = untrusted attacker-controllable input): panic-free
  — bounds-checked integer reads (out-of-range → 0, never panic), range-check every
  length/offset/chunk-count BEFORE use, cap allocations (no allocation bombs),
  `unwrap_used`/`expect_used = deny` (already in workspace lints). **One fuzz target
  per parsed structure** (segment header, tree node, chunk table) + a
  `fuzz_forensic` driving open→audit; `fuzz.yml` builds + smoke-runs them.
- **Validation (`docs/validation.md`)**: Tier-1 against an independent oracle —
  reconcile `ad1-core` file extraction + hashes against **`ad1extract`/`ad1check`**
  (al3ks1s) AND **FTK Imager** on the real `userbss.ad1`; explain any divergence.
- **100% line coverage** (`cargo llvm-cov --lib`, `// cov:unreachable` for
  provably-dead defensive arms).
- **README** (SecurityRonin standard: two-row badges = guarantees actually
  enforced; footer Privacy/Terms → GitHub Pages), **MkDocs** docs site (not
  rustdoc-only), **deny.toml** (Apache-2.0 + permissive only; the tree is pure-Rust
  so no C-license surprises), `.gitleaks.toml`, `clippy.toml`, `rustfmt.toml`,
  `.pre-commit-config.yaml`, `renovate.json`, **release.yml** (tag-driven; library
  → `crate` job only, no binary/Homebrew channels).
- **Test-data provenance**: a `tests/data/README.md` per the fleet standard; large
  `.ad1` images gitignored + downloaded; small clearly-licensed fixtures committed
  with an md5 manifest. Add an entry to the fleet `issen/docs/corpus-catalog.md`.

## 7. Open questions / risks

- **AD1 version coverage** (v3 vs v4) — confirm both layouts from al3ks1s source;
  fail loud (with the version bytes) on anything else.
- **ADCRYPT** — encrypted images: v1 detects + refuses; decryption is a later epic.
- **Chunk/compression specifics** — confirm zlib (not raw deflate) framing + the
  chunk-size/field widths from the oracle source; flate2 `rust_backend` is pure-Rust.
- **Endianness + 64-bit fields** — AD1 uses big-endian in places; verify per field.
- **Compat shim** — consider a thin "looks like a tar/zip reader" API so the issen
  CollectionProvider wiring mirrors the existing zip/tar collection code.
- **`ad1` crate name** — re-check availability on crates.io right before publish.

## 8. First three commits (suggested)

1. `chore: scaffold ad1-forensic workspace` (this scaffold — already here).
2. `test(ad1-core): RED — segment header + ADSEGMENTEDFILE detect + version`
   then `feat(ad1-core): GREEN — …` (real `.ad1` magic fixture or a crafted header).
3. `test(ad1-core): RED — file-tree walk vs ad1info` → GREEN, on `userbss.ad1`.
