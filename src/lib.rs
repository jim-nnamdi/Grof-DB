
pub mod lsm {
    use std::{ fs::{File, OpenOptions}, 
io::{self, BufRead, BufReader, Result, Write}, 
path::{Path, PathBuf}, sync::{Arc, Mutex}, time::{Duration, Instant}};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
    pub struct WNode {
        pub key: String,
        pub val: Option<String>
    }

    impl WNode {
        pub fn new(key: &str, val: &str) -> Self {
            Self { key: key.into(), val: Some(val.into())}
        }
    }

    pub struct WAL {
        dir: PathBuf,
        wrt: Mutex<File>,
        seg: u64,
        off: u64
    }

    impl WAL {
        pub fn open(dir:impl Into<PathBuf>) -> Result<Arc<Mutex<Self>>> {
            let dir = dir.into();
            std::fs::create_dir_all(&dir)?;
            let seg = Self::latest_segment(&dir)?;
            let fss = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(Self::segment_path(&dir, seg))?;
            let offs = fss.metadata()?.len();
            Ok(Arc::new(Mutex::new(Self { dir, wrt: Mutex::new(fss), seg, off: offs })))
        }

        pub fn segment_path(dir: &Path, seg: u64) -> PathBuf {
            dir.join(format!("wal-{:06}.log", seg))
        }

        pub fn latest_segment(dir: &Path) -> Result<u64> {
            let mut max = 0;
            let fss =  std::fs::read_dir(dir)?;
            for e in fss {
                let name = e?.file_name().into_string().unwrap_or_default();
                if let Some(s) = name.strip_prefix("wal")
                .and_then(|s| s.strip_suffix(".log")){
                    if let Ok(n) = s.parse::<u64>() {
                        max = max.max(n)
                    }
                }
            }
            Ok(max)
        }

        pub fn append(&mut self, e: &WNode) -> Result<()> {
            let json  = serde_json::to_string(e)?;
            let line = format!("{}\n", json);
            self.off = line.len() as u64;
            const SEG_SZ: u64 = 64 * 1024 * 1024;
            if self.off >= SEG_SZ { self.rotate()?; }

            let mut f = self.wrt.lock().unwrap();
            f.write_all(line.as_bytes())?;
            f.flush()?; f.sync_all()?;
            Ok(())
        }

        fn rotate(&mut self) -> Result<()>{
            self.seg += 1; self.off = 0;
            let path = self.dir.join(format!("wal-{:06}.log", self.seg));
            let file = OpenOptions::new().create(true).append(true).read(true).open(&path)?;
            self.wrt = Mutex::new(file);
            Ok(())
        }

        pub fn replay(dir: impl Into<PathBuf>) -> Result<Vec<WNode>> {
            let fss = File::open(dir.into())?;
            let buf = BufReader::new(fss);
            buf.lines().map(|line| {
                let line = line?;
                let entry = serde_json::from_str(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(entry)
            }).collect()
        }

        pub fn replay_two(dir: impl Into<PathBuf>) -> Result<Vec<WNode>> {
            let mut entries = vec![];
            let mut segs: Vec<u64> = std::fs::read_dir(dir.into())?
            .filter_map(|e| e.ok())
            .filter_map(|de| {
                let n =  de.file_name().into_string().unwrap_or_default();
                n.strip_prefix("wal").and_then(|s| s.strip_suffix(".log"))
                .and_then(|s| s.parse::<u64>().ok())
            }).collect();

            segs.sort_unstable();
            for seg in segs {
                let paf = format!("wal-{:06}.log", seg);
                let fss = File::open(paf)?;
                let buf = BufReader::new(fss);
                let mut off = 0u64;
                buf.lines().map(|line| {
                    let line = line.unwrap();
                    let entry: WNode = serde_json::from_str(&line).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)).unwrap();
                    entries.push(entry);
                    off += line.len() as u64 + 1;
                }).collect()
            }
            Ok(entries)
        }
    }

    pub fn bench(function_benchmark: impl FnOnce() -> ()) -> Duration {
        let start = Instant::now();
        function_benchmark();
        start.elapsed()
    }
}
