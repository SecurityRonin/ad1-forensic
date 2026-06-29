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
- **Status (2026-06-30):** **not downloaded.** The Drive mirror returns Google's
  anonymous-download quota error ("try again, up to 24 h"). Retry the Drive link
  or fetch via a signed-in browser. Record MD5/SHA256 + size here once pulled.
- **License/redistribution:** NIST CFReDS dataset — verify the dataset terms
  before redistributing; the bytes are **not** committed regardless.
- **Used by (planned):** an env-gated test (`AD1_USERBSS=/path/to/userbss.ad1`)
  reconciling `ad1-core` extraction + recomputed hashes against the AD1's stored
  hashes and `ad1extract`/`ad1check` (al3ks1s/AD1-tools) / FTK Imager.
