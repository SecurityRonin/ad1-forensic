# ad1-forensic

Pure-Rust reader (`ad1-core`) and forensic auditor (`ad1-forensic`) for the
**AccessData AD1** logical image container (FTK Imager "Custom Content Image").

AD1 is a *logical* container — a tree of files/folders with per-file metadata and
zlib-compressed data — so the reader exposes a virtual filesystem (path → bytes +
stored hashes), like a zip/tar reader, with no disk/partition/filesystem layer.

```rust
use ad1::Ad1Reader;

let img = Ad1Reader::open(std::path::Path::new("evidence.ad1"))?;
for entry in img.entries() {
    println!("{} ({} bytes)", entry.path, entry.size);
}
# Ok::<(), ad1::Ad1Error>(())
```

- **[Format](format.md)** — the AD1 on-disk layout.
- **[Validation](validation.md)** — how correctness is established (and its tiers).
- No `unsafe`, panic-free on malformed input, fuzzed.

See the [README](https://github.com/SecurityRonin/ad1-forensic) for the full guide.
