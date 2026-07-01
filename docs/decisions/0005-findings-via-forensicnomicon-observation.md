# 0005 — Emit findings via `forensicnomicon::Observation` with stable codes

**Status:** Accepted

## Context

`ad1-forensic` must report anomalies (hash mismatch, encryption, missing
segment, size lie, unreadable) in a way the fleet's tools (`disk4n6`, `issen`)
render uniformly. `forensicnomicon::report` is the shared, leaf-level report
vocabulary the other analyzers normalize into.

## Decision

Define a typed `Ad1Anomaly` enum and implement `forensicnomicon::Observation` on
it, so the canonical `Finding` is assembled in one place (`to_finding`). Anomaly
`code` strings are a stable, scheme-prefixed contract: `AD1-HASH-MISMATCH`,
`AD1-ENCRYPTED`, `AD1-SEGMENT-MISSING`, `AD1-SIZE-LIE`, `AD1-UNREADABLE`.
Findings are observations with evidence (both stored and recomputed hashes
shown), never assertions of intent, per the report model's contract.

## Consequences

- Findings render consistently across the fleet; codes are machine-filterable.
- Hash verification recomputes MD5/SHA1 over `ad1-core`'s streamed `read_at`
  (bounded memory) — the same independent-oracle logic as validation, now as a
  runtime tamper check.
- `AD1-HASH-MISMATCH` classifies as `Integrity`, `AD1-ENCRYPTED` as `Provenance`
  (a property of the image), the rest as `Structure`.

## Alternatives considered

- **A bespoke finding type** — rejected: fragments the fleet report model and
  loses uniform rendering.
- **Route the auditor's raw-layout needs only through `ad1-core`** — kept open:
  the fleet principle allows dropping to raw segment bytes for slack/malformed
  nodes the reader normalizes away; not needed for the v1 anomaly set.
