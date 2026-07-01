# 0001 — Build a pure-Rust AD1 reader

**Status:** Accepted

## Context

AccessData AD1 ("Custom Content Image", FTK Imager) is a logical evidence
container. Before writing any code we searched crates.io / PyPI / npm / GitHub
for existing implementations (Research-First):

- **crates.io:** nothing for AD1 (`ad1`, `accessdata`, `ftk imager`, `logical
  image`, `custom-content-image` all empty).
- **libewf / TSK:** do not support AD1, so the open-source disk-forensics stack
  cannot open it.
- **Reference implementations:** `al3ks1s/AD1-tools` (C, GPL), the Cerbero AD1
  package (closed), and format write-ups (DFIRScience).

## Decision

Build a new pure-Rust reader (`ad1-core`) plus auditor (`ad1-forensic`). No
`unsafe`, no C dependencies (flate2 `rust_backend` for zlib, RustCrypto for
hashes), so the crate is portable and fits the fleet's C-free tree.

## Consequences

- Fills a real ecosystem gap; the fleet (issen) can ingest AD1 logical packages.
- We own correctness — hence the validation strategy in [0002](0002-tiered-independent-oracle-validation.md).
- The C reference remains valuable as the byte-layout spec and a cross-check
  oracle, even though we do not link it.

## Alternatives considered

- **Bind the C `libad1`** — rejected: pulls a GPL C dependency into an
  `#![forbid(unsafe_code)]`, Apache-2.0, pure-Rust tree; cross-compilation and
  supply-chain cost.
- **Don't support AD1** — rejected: AD1 is a secondary format (FTK Imager's
  *logical* acquisition output; E01 dominates imaging), but it is still
  encountered *and* unreadable anywhere in the OSS/Rust/TSK stack. The gap — real
  usage with zero open-source support — is the case for building, not prevalence.
