//! Fuzz the forensic auditor end to end: arbitrary bytes through `audit`
//! (open → tree walk → hash recomputation → finding assembly). Never panic.
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
    let _ = ad1_forensic::audit(&path);
});
