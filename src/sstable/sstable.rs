pub mod sstable {
    use std::{
        fs::{File, OpenOptions},
        io::{self, BufRead, BufReader, Result},
        path::{Path, PathBuf},
        sync::{Arc, Mutex},
    };

    use SDB::lsm::WNode;

    pub struct SSTable {
        dir: PathBuf,
        wrt: Mutex<File>,
        seg: u64,
        off: u64,
    }

    impl SSTable {
        pub fn new(dir: impl Into<PathBuf>) -> io::Result<Arc<Self>> {
            let dir = dir.into();
            let seg = Self::sstable_latest_segment(&dir)?;
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .read(true)
                .open(Self::ssegment_path(&dir, seg))?;
            let off = file.metadata()?.len();
            Ok(Arc::new(Self {
                dir,
                wrt: Mutex::new(file),
                seg,
                off,
            }))
        }

        pub fn ssegment_path(dir: &Path, seg: u64) -> PathBuf {
            dir.join(format!("sstable-{:06}.log", seg))
        }

        pub fn sstable_latest_segment(dir: &Path) -> Result<u64> {
            let mut max = 0;
            let fss = std::fs::read_dir(dir);
            for e in fss? {
                let name = e?.file_name().into_string().unwrap_or_default();
                if let Some(s) = name
                    .strip_prefix("sstable")
                    .and_then(|s| s.strip_suffix(".log"))
                {
                    if let Ok(n) = s.parse::<u64>() {
                        max = max.max(n)
                    }
                }
            }
            Ok(max)
        }

        pub fn append_to_sstable(&self, dir: impl Into<PathBuf>) {
            let mut svec = vec![];
            let mut fseg: Vec<u64> = std::fs::read_dir(&dir.into())
                .unwrap()
                .filter_map(|e| e.ok())
                .filter_map(|de| {
                    let name = de.file_name().into_string().unwrap_or_default();
                    name.strip_prefix("sstable")
                        .and_then(|s| s.strip_suffix(".log"))
                        .and_then(|s| s.parse::<u64>().ok())
                })
                .collect();

            fseg.sort_unstable();
            for seg in fseg {
                let pat = format!("sstable-{:06}.log", seg);
                let fz = File::open(pat).unwrap();
                let bz = BufReader::new(fz);
                let _ = bz
                    .lines()
                    .map(|line| {
                        let line = line.unwrap();
                        let node: WNode = serde_json::from_str(&line)
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                            .unwrap();
                        svec.push(node);
                    })
                    .collect::<Vec<_>>();
            }
        }
    }
}
