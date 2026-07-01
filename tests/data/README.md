# tests/data — AD1 Test Corpus

Real AccessData AD1 logical images used as Tier-1 oracles for `ad1-core` (the
reader) and `ad1-forensic` (the auditor). Large `.ad*` images are **gitignored**
(`/tests/data/*.ad*`) and downloaded manually — never committed. Tests read them
in place and are **env-gated** (skip cleanly when the file is absent).

For the fleet-wide corpus inventory see `issen/tests/data/README.md`
(`magnet-summit-2025-ctf/`) and `issen/docs/corpus-catalog.md`.

Automated tests that do NOT need a real image build crafted AD1 fixtures in memory
via the `ad1::testfix` module (feature `testfix`), using independent `flate2` zlib
and RustCrypto md5/sha1 as ground truth (tier-2; see `docs/validation.md`).

## `userbss.ad1`

- **Source:** 2025 Magnet Virtual Summit CTF — authored by the **Hexordia** team
  (Kevin Pagano et al.) with Champlain College DFA interns, for Magnet Forensics.
- **Catalog page:** NIST **CFReDS** — <https://cfreds.nist.gov/all/Hexordia/2025MVSCTF>
  (public index). CFReDS hosts the dataset on Google Drive.
- **Google Drive folder:** <https://drive.google.com/drive/folders/1qLwXFZTZidkx1tWpG8uenVQnX6zWF-Oa>
  — `userbss.ad1` is Drive id `1ImeVi8BzHcuLDOV7LhAle9kRnZOMFb64`.
- **Writeup:** <https://www.magnetforensics.com/blog/announcing-the-winners-of-the-2025-magnet-virtual-summit-ctf/>
- **Format:** AccessData **AD1** (`ADSEGMENTEDFILE` / `ADLOGICALIMAGE`),
  `Custom Content Image([Multi])`. Single segment (`.ad1`).
- **Contents:** a logical capture of an NTFS volume labelled **`OS`**
  (`E:\:OS [NTFS]`, NTFS 3.1), user profile / `bss` material, timestamps in 2024.
  A tree of files + per-file metadata and stored MD5/SHA1, with file data in
  zlib-compressed chunks.
- **Size:** 51,678,663,221 bytes (≈ 48.1 GiB).
- **MD5:** `0b6b53e3475b97ae8b3bd3c1e7cec2d9`
- **SHA256:** `743e1e89e1d4fa9d6f75d91e820f6dd02d2d906e1bab70eb4731a2fdb4458e7c`
- **Redistribution / license:** public CTF dataset published via NIST CFReDS for
  research and education. Not redistributed from this repo — gitignored; download
  it yourself from the sources above.
- **Used by:** the AD1 reader/auditor integration tests (env-gated). Ground truth
  is cross-checked against the `al3ks1s/AD1-tools` oracle — `ad1info` (header +
  file tree), `ad1extract` (byte-identical file extraction), `ad1check` (stored
  per-file hash verification).
- **Tier-1 result:** `ad1-core` parses it to 316,682 entries (version 4, 64 KiB
  chunks); all **299,729** files' recomputed MD5 **and** SHA1 match FTK Imager's
  stored hashes — 0 mismatches (`core/tests/tier1_real.rs`, run with
  `AD1_USERBSS=tests/data/userbss.ad1`; see `docs/validation.md`).

### Verifying your copy

```sh
shasum -a 256 tests/data/userbss.ad1
# 743e1e89e1d4fa9d6f75d91e820f6dd02d2d906e1bab70eb4731a2fdb4458e7c
```
