pub mod memtable {
    use std::io::Result;
    use std::sync::{Arc, RwLock};
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use SDB::lsm::{WAL,WNode};

    use crate::sstable::sstable;

    const MT_SIZ: u64 = 64 * 1024 * 1024;
    const MT_DIR: &str = "./data/memtable";

    pub struct MTable {
        dbuf: Arc<RwLock<BTreeMap<String, Option<String>>>>,
        size: Arc<RwLock<usize>>,
    }

    impl MTable {
        pub fn new(dir: impl Into<PathBuf>) -> Result<Vec<WNode>> {
            let wal = WAL::replay_two(dir.into());
            wal
        }

        pub fn get(&self, key: &str) -> Result<Option<String>> {
            let dat = self.dbuf.read().unwrap();
            /* dat.get(key).and_then(|s| s.clone()) */
            /* dat.get(key).cloned().flatten() */
            let mat = dat.get(key).cloned();
            match mat {
                Some(v) => {return Ok(v);},
                None => {Ok(None)}
            }
        }

        pub fn read(&self){
            let mat = self.dbuf.read().unwrap();
            for (k, v) in mat.iter().enumerate() {
                println!(" k = {:?} v = {:?}", k, v)
            }
        }

        pub fn put(&self, key: &str, val: &str){
            let sz =self.size.write().unwrap();
            if *sz as u64 >= MT_SIZ { 
                self.flush(MT_DIR).unwrap();
            }
            let wnode = WNode::new(key, val);
            let mut dat = self.dbuf.write().unwrap();
            dat.insert(wnode.key.into(), wnode.val);
            let mut s = self.size.write().unwrap();
            *s += key.len() + val.len();
        }

        pub fn remove(&self, key: &str) {
            let mut buf = self.dbuf.write().unwrap();
            if let Some(val) = buf.remove(key.into()) {
                let mut sz = self.size.write().unwrap();
                *sz -= key.len();
                if let Some(v) = val{
                    *sz -= v.len()
                }
            }
        }

        pub fn flush(&self, dir: impl Into<PathBuf>) -> Result<()>{
            let sz =  self.size.read().unwrap();
            if *sz as u64 >= MT_SIZ {
                let dir =  dir.into();
                let stable  = sstable::sstable::SSTable::new(&dir)?;
                stable.append_to_sstable(&dir);
            }
            Ok(())
        }
    }
}
