pub mod memtable {
    use std::io::Result;
    use std::sync::{Arc, RwLock};
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use SDB::lsm::{WAL,WNode};

    use crate::sstable::sstable;

    const MT_SIZ: u64 = 64 * 1024 * 1024;
    pub const MT_DIR: &str = "./data/memtable";

    pub struct MTable {
        dbuf: Arc<RwLock<BTreeMap<String, Option<String>>>>,
        size: Arc<RwLock<usize>>,
    }

    impl MTable {
        pub fn new(dir: impl Into<PathBuf>) -> Result<Self> {
            let wal = WAL::replay_two(dir.into())?;
            let mut dat = BTreeMap::new();
            let mut sz = 0;
            for i in wal.iter() {
                dat.insert(i.key.clone(), i.val.clone());
                if let Some(v) = &i.val {
                    sz += i.key.len() + v.len();
                } else { sz += i.key.len()}
            }
            let buf = Arc::new(RwLock::new(dat));
            Ok(Self {dbuf: buf, size: Arc::new(RwLock::new(sz))})
        }

        pub fn get(&self, key: &str) -> Result<Option<String>> {
            let dat = self.dbuf.read().unwrap();
            /* dat.get(key).and_then(|s| s.clone()) */
            /* dat.get(key).cloned().flatten() */
            let mat = dat.get(key).cloned();
            dbg!(key);
            match mat {
                Some(v) => {return Ok(v);},
                None => {Ok(None)}
            }
        }

        pub fn read(&self){
            let mat = self.dbuf.read().unwrap();
            for (k, v) in mat.iter() {
                println!(" k = {:?} v = {:?}", k, v)
            }
        }

        pub fn put(&self, key: &str, val: &str) -> Result<()>{
            let mut sz =self.size.write().unwrap();
            if *sz as u64 >= MT_SIZ { 
                self.flush(MT_DIR).unwrap();
            }
            let wnode = WNode::new(key, val);
            let mut dat = self.dbuf.write().unwrap();
            dat.insert(wnode.key.into(), wnode.val);
            *sz += key.len() + val.len();
            dbg!(*sz);
            Ok(())
        }

        pub fn remove(&self, key: &str) {
            let mut buf = self.dbuf.write().unwrap();
            if let Some(val) = buf.remove(key) {
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
