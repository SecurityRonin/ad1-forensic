# 0004 — Detect and refuse ADCRYPT; no decryption in v1

**Status:** Accepted

## Context

AD1 has an encrypted variant, `ADCRYPT` (AES-128/192/256, PBKDF2 with SHA-256/512
per the reference header). A forensic reader that silently produced garbage for
an encrypted image would fabricate evidence — the worst failure mode for a
forensic tool.

## Decision

Detect `ADCRYPT` from the leading signature and **refuse**:
`Ad1Reader::open` returns `Ad1Error::Unsupported` (with the signature bytes) and
the auditor emits an `AD1-ENCRYPTED` finding. Decryption is explicitly out of
scope for v1.

## Consequences

- No fabricated output: an encrypted image is reported as such, never decoded to
  plausible-but-wrong bytes.
- Decryption (key derivation, AES) is a later epic; when built it must reuse
  audited RustCrypto crates and validate against a real encrypted image, never a
  self-made round-trip.

## Alternatives considered

- **Attempt decryption now** — rejected: scope creep; requires key material and
  careful crypto, and would delay the core reader.
- **Best-effort parse of the ciphertext** — rejected: guarantees garbage output;
  a fail-loud refusal is the only forensically safe behavior.
