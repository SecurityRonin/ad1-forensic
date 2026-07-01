# Test data provenance

This crate's automated tests build **crafted AD1 fixtures in memory** via the
`ad1::testfix` module (feature `testfix`) — no binary fixtures are committed.
Those fixtures use independent `flate2` zlib and RustCrypto md5/sha1 as ground
truth (see [docs/validation.md](../../docs/validation.md)).

Real-world AD1 images are large and are **gitignored** (`/tests/data/*.ad*`) and
downloaded manually.

## userbss.ad1 (pending download)

- **Source:** 2025 Magnet Virtual Summit CTF — authored by the **Hexordia** team
  (Kevin Pagano) with Champlain College DFA interns, for Magnet Forensics.
- **Authoritative host:** NIST **CFReDS** — <https://cfreds.nist.gov/all/Hexordia/2025MVSCTF>
  (the "NIST hosted" link redirects to a Google Drive mirror; there is no
  NIST-direct download).
- **Drive file id:** `1ImeVi8BzHcuLDOV7LhAle9kRnZOMFb64` (folder `1gHQsfx1hqCv-V4Anm2eYukB4jPtraNBH`).
- **Type:** AccessData AD1 logical image (FTK Imager) with FTK-computed stored
  per-file hashes — the independent oracle for tier-1 validation.
- **Status:** **downloaded**, placed at `tests/data/userbss.ad1` in this repo.
  Size **51,678,663,221 bytes** (48 GiB), single segment, magic `ADSEGMENTEDFILE`.
  **MD5** `0b6b53e3475b97ae8b3bd3c1e7cec2d9` (verified in place). Gitignored
  (`/tests/data/*.ad*`) — not committed.
- **License/redistribution:** NIST CFReDS dataset — verify the dataset terms
  before redistributing; the bytes are **not** committed regardless.
- **Used by:** the env-gated tier-1 test `core/tests/tier1_real.rs`
  (`AD1_USERBSS=/path/to/userbss.ad1`) reconciles `ad1-core`'s recomputed MD5/SHA1
  against the AD1's FTK-written stored hashes. Image parses to **316,682** entries;
  all **299,729** files match **MD5 + SHA1, 0 mismatches** (see `docs/validation.md`).
