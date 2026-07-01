# Architecture Decision Records

Each ADR captures one significant, non-obvious decision and *why* — the rationale
that commit messages and code comments don't preserve well. Reverse-documented
from the initial implementation session; they record the reasoning as it stood
when the decision was made.

| ADR | Decision |
|---|---|
| [0001](0001-build-pure-rust-ad1-reader.md) | Build a pure-Rust AD1 reader (no ecosystem crate exists) |
| [0002](0002-tiered-independent-oracle-validation.md) | Validate against independent oracles, tiered (avoid the LZNT1 trap) |
| [0003](0003-panic-free-bounded-parsing.md) | Panic-free, allocation-bounded parsing for untrusted input |
| [0004](0004-detect-and-refuse-adcrypt.md) | Detect and refuse ADCRYPT; no decryption in v1 |
| [0005](0005-findings-via-forensicnomicon-observation.md) | Emit findings via `forensicnomicon::Observation` with stable codes |
