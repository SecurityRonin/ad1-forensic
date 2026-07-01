# 0003 — Panic-free, allocation-bounded parsing for untrusted input

**Status:** Accepted

## Context

An AD1 image is attacker-controllable input. Every length, offset, count, and
address in it is untrusted. A parser that indexes slices directly, trusts a
declared size, or does unchecked arithmetic will panic (DoS) or allocate
unboundedly on crafted input.

## Decision

A "Paranoid Gatekeeper" posture, enforced structurally:

- **All integer reads go through bounds-checked helpers** (`bytes::u32_le` /
  `u64_le`) that return `0` when the range is out of bounds — no raw slice
  indexing anywhere else, so the parser is panic-free by construction.
- **Logical addressing is centralized** in `SegmentSet::read`, which caps any
  single read at the image's real data size (allocation-bomb guard) and spans
  segments in one place.
- **Every length/offset/count is range-checked before use** with explicit caps
  (`MAX_NAME_LEN`, `MAX_META_DATA`, `MAX_ENTRIES`, `MAX_SEGMENTS`,
  `MAX_CHUNK_SIZE`), and arithmetic near untrusted values is overflow-safe
  (`saturating_*`, `count >= max` instead of `count + 1 > max`).
- **The tree walk is iterative with a visited-set cycle guard** — a deep/wide or
  cyclic tree cannot overflow the stack or loop forever.
- **Lints + fuzz** back this up: `unwrap_used`/`expect_used = deny`,
  `#![forbid(unsafe_code)]`, and two fuzz targets whose invariant is "never
  panic."

## Consequences

- Fuzzing found and fixed three real defects this posture is designed to prevent:
  a 72 GB allocation from an attacker `segment_count`, a `count + 1` overflow on
  a `u64::MAX` chunk count, and a `cur - chunk_base` underflow on a short chunk.
  Each has a deterministic regression test.
- Malformed input yields a loud `Ad1Error::Malformed` (with the offending value)
  or a graceful degrade, never a crash or silent wrong output.

## Alternatives considered

- **`Result`-returning integer reads** — rejected: threads `?` through every
  field read for no gain; returning `0` on OOB plus explicit up-front range
  checks is simpler and equally safe.
- **Recursive tree walk** — rejected: stack-overflow and cycle risk on hostile
  input.
