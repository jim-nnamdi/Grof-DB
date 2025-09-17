use std::{fs::{File, OpenOptions}, io::{self, BufRead, BufReader, Error, Result, Write}, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct WNode {
    cot: u64,
    key: String,
    val: Option<String>
}

pub struct WAL {
    dir: PathBuf,
    wrt: Mutex<File>,
    seg: u64,
    off: u64
}

impl WAL {
    pub fn open(dir:impl Into<PathBuf>) -> Result<Arc<Self>> {
        let dir = dir.into();
        std::fs::create_dir_all(&dir)?;
        let seg = Self::latest_segment(&dir)?;
        let fss = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(Self::segment_path(&dir, seg))?;
        let offs = fss.metadata()?.len();
        Ok(Arc::new(Self { dir, wrt: Mutex::new(fss), seg, off: offs }))
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
        let line = format!("{}", json);
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

    pub fn replay(&self) -> Result<Vec<WNode>> {
        let fss = File::open(&self.dir)?;
        let buf = BufReader::new(fss);
        buf.lines().map(|line| {
            let line = line?;
            let entry = serde_json::from_str(&line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(entry)
        }).collect()
    }
}

fn main() {}