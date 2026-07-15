//! `impl FileSystem for Ad1Vfs` — RED scaffold (tests only; the implementation
//! lands in the following commit).

#[cfg(all(test, feature = "testfix"))]
mod tests {
    use super::*;
    use crate::testfix;
    use forensic_vfs::{
        Allocation, FileId, FileSystem, FsKind, NodeKind, RunAlloc, StreamId, TimeZonePolicy,
    };

    /// Build the canonical sample tree, write it to a tempdir as `image.ad1`, and
    /// open it through the adapter. Returns the tempdir (kept alive), the mounted
    /// filesystem, and the builder's expected per-entry facts (ground truth).
    fn open_sample() -> (tempfile::TempDir, Ad1Vfs, Vec<testfix::Expected>) {
        let built = testfix::build(testfix::sample_tree());
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("image.ad1");
        std::fs::write(&path, &built.bytes).unwrap();
        let fs = Ad1Vfs::open(&path).unwrap();
        (dir, fs, built.expected)
    }

    fn expected_of<'a>(exp: &'a [testfix::Expected], path: &str) -> &'a testfix::Expected {
        exp.iter().find(|e| e.path == path).expect("expected entry")
    }

    /// Resolve a `/`-separated path from the synthetic root via `lookup`.
    fn resolve(fs: &Ad1Vfs, parts: &[&[u8]]) -> FileId {
        let mut id = fs.root();
        for p in parts {
            id = fs.lookup(id, p).unwrap().unwrap();
        }
        id
    }

    /// Drain a file to EOF by looping `read_at`.
    fn read_all(fs: &Ad1Vfs, id: FileId) -> Vec<u8> {
        let mut out = Vec::new();
        let mut off = 0u64;
        loop {
            let mut buf = [0u8; 4096];
            let n = fs.read_at(id, StreamId::Default, off, &mut buf).unwrap();
            if n == 0 {
                break;
            }
            out.extend_from_slice(&buf[..n]);
            off += n as u64;
        }
        out
    }

    #[test]
    fn kind_root_zone_and_sectors() {
        let (_d, fs, _e) = open_sample();
        assert_eq!(fs.kind(), FsKind::Other);
        assert!(matches!(fs.root(), FileId::Opaque(0)));
        // AD1's timestamps are zoneless display strings, not epochs.
        assert_eq!(fs.timestamp_zone(), TimeZonePolicy::LocalUnknown);
        let ss = fs.sector_sizes();
        assert_eq!(ss.logical, 512);
        assert_eq!(ss.cluster_or_block, 512);
        assert!(ss.physical >= 512);
        assert_eq!(fs.meta(fs.root()).unwrap().kind, NodeKind::Dir);
    }

    #[test]
    fn lists_root_and_reaches_root_dir() {
        let (_d, fs, _e) = open_sample();
        let names: Vec<Vec<u8>> = fs
            .read_dir(fs.root())
            .unwrap()
            .map(|e| e.unwrap().name)
            .collect();
        assert!(
            names.iter().any(|n| n == b"root"),
            "synthetic root should list the 'root' dir, got {names:?}"
        );
        let root_dir = fs.lookup(fs.root(), b"root").unwrap().unwrap();
        assert_eq!(fs.meta(root_dir).unwrap().kind, NodeKind::Dir);
    }

    #[test]
    fn reads_hello_meta_and_content() {
        let (_d, fs, exp) = open_sample();
        let e = expected_of(&exp, "root/hello.txt");
        let id = resolve(&fs, &[b"root", b"hello.txt"]);
        let m = fs.meta(id).unwrap();
        assert_eq!(m.kind, NodeKind::File);
        assert_eq!(m.size, e.size);
        assert_eq!(m.allocated, Allocation::Allocated);
        // AD1 stores no epoch timestamps; honestly absent, never epoch-0.
        assert!(m.times.modified.is_none());
        assert!(m.times.accessed.is_none());
        assert!(m.times.changed.is_none());
        assert!(m.times.born.is_none());
        assert_eq!(m.uid, None);
        assert_eq!(m.gid, None);
        assert_eq!(m.mode, None);
        assert_eq!(read_all(&fs, id), *e.data.as_ref().unwrap());
    }

    #[test]
    fn reads_large_file_spanning_chunks() {
        let (_d, fs, exp) = open_sample();
        let e = expected_of(&exp, "root/sub/a.bin");
        let id = resolve(&fs, &[b"root", b"sub", b"a.bin"]);
        let m = fs.meta(id).unwrap();
        assert_eq!(m.size, e.size);
        assert!(m.size > u64::from(testfix::CHUNK_SIZE), "spans >1 chunk");
        assert_eq!(&read_all(&fs, id), e.data.as_ref().unwrap());
    }

    #[test]
    fn directory_reports_dir_kind() {
        let (_d, fs, _e) = open_sample();
        let id = resolve(&fs, &[b"root", b"sub"]);
        assert_eq!(fs.meta(id).unwrap().kind, NodeKind::Dir);
        assert!(fs.read_dir(id).is_ok());
    }

    #[test]
    fn empty_file_reads_zero_and_no_extents() {
        let (_d, fs, _e) = open_sample();
        let id = resolve(&fs, &[b"root", b"sub", b"empty.dat"]);
        let m = fs.meta(id).unwrap();
        assert_eq!(m.size, 0);
        assert_eq!(m.kind, NodeKind::File);
        let mut buf = [0u8; 8];
        assert_eq!(fs.read_at(id, StreamId::Default, 0, &mut buf).unwrap(), 0);
        assert_eq!(fs.extents(id, StreamId::Default).unwrap().count(), 0);
    }

    #[test]
    fn extents_hello_single_run_and_root() {
        let (_d, fs, exp) = open_sample();
        let e = expected_of(&exp, "root/hello.txt");
        let id = resolve(&fs, &[b"root", b"hello.txt"]);
        let runs: Vec<_> = fs
            .extents(id, StreamId::Default)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].run.len, e.size);
        assert_eq!(runs[0].alloc, RunAlloc::Allocated);
        let root_runs: Vec<_> = fs
            .extents(fs.root(), StreamId::Default)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert!(root_runs.len() <= 1);
    }

    #[test]
    fn read_at_offset_and_past_eof() {
        let (_d, fs, _e) = open_sample();
        let id = resolve(&fs, &[b"root", b"hello.txt"]);
        let mut buf = [0u8; 8];
        // "Hello, AD1!\n" — offset 7 is "AD1!\n".
        let n = fs.read_at(id, StreamId::Default, 7, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"AD1!\n");
        assert_eq!(
            fs.read_at(id, StreamId::Default, 9999, &mut buf).unwrap(),
            0
        );
    }

    #[test]
    fn wrong_file_id_and_stream_are_loud() {
        let (_d, fs, _e) = open_sample();
        let bad = FileId::NtfsRef { entry: 5, seq: 1 };
        assert!(fs.meta(bad).is_err());
        assert!(fs.read_dir(bad).is_err());
        assert!(fs.lookup(bad, b"x").is_err());
        assert!(fs.read_link(bad, 8).is_err());
        // An out-of-range node index is refused.
        assert!(fs.meta(FileId::Opaque(9_999_999)).is_err());
        // A named stream is refused.
        let id = resolve(&fs, &[b"root", b"hello.txt"]);
        assert!(fs
            .read_at(id, StreamId::Named(1), 0, &mut [0u8; 4])
            .is_err());
        assert!(fs.extents(id, StreamId::Named(1)).is_err());
        // read_dir on a file is loud.
        assert!(fs.read_dir(id).is_err());
    }

    #[test]
    fn lookup_missing_is_none() {
        let (_d, fs, _e) = open_sample();
        assert!(fs.lookup(fs.root(), b"NOPE.NOTPRESENT").unwrap().is_none());
    }

    #[test]
    fn empty_forensic_surfaces() {
        let (_d, fs, _e) = open_sample();
        assert_eq!(fs.deleted().unwrap().count(), 0);
        assert_eq!(fs.unallocated().unwrap().count(), 0);
        let id = resolve(&fs, &[b"root", b"hello.txt"]);
        assert!(fs.read_link(id, 4096).unwrap().is_empty());
    }

    #[test]
    fn index_of_rejects_non_opaque() {
        assert!(super::index_of(FileId::Opaque(42)).is_ok());
        assert!(super::index_of(FileId::NtfsRef { entry: 1, seq: 1 }).is_err());
    }

    #[test]
    fn leaf_splits_on_last_separator() {
        assert_eq!(super::leaf("root/sub/a.bin"), b"a.bin");
        assert_eq!(super::leaf("toplevel"), b"toplevel");
        assert_eq!(super::leaf("a/b/c"), b"c");
    }
}
