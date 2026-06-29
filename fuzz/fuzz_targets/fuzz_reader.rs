//! Fuzz the AD1 reader end to end: arbitrary bytes exercise the segment header,
//! logical header, tree-node, metadata, and chunk-table parsers via `open`, plus
//! positioned decompression via `read_at`. Invariant: never panic.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(dir) = tempfile::tempdir() else {
        return;
    };
    let path = dir.path().join("fuzz.ad1");
    if std::fs::write(&path, data).is_err() {
        return;
    }
    if let Ok(reader) = ad1::Ad1Reader::open(&path) {
        let mut buf = vec![0u8; 8192];
        for entry in reader.entries() {
            let mut off = 0u64;
            let mut guard = 0u32;
            while let Ok(n) = reader.read_at(entry, off, &mut buf) {
                if n == 0 {
                    break;
                }
                off += n as u64;
                guard += 1;
                if guard > 100_000 {
                    break; // bound runaway loops on crafted huge sizes
                }
            }
        }
    }
});
