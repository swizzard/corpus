use crate::entities::CorpusEntity;
use crate::env_default;
use crate::errors::{CorpusError, CorpusResult};
use crate::marble::{pf, CorpusState, CorpusWrite, Page};
use marble;
use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeMap;
use std::env;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::RwLock;

#[derive(Debug)]
struct WriteState {
    author_id: u64,
    collection_id: u64,
    document_id: u64,
    token_id: u64,
    db: marble::Marble,
}

impl CorpusState<WriteState> {
    pub(crate) fn default_config() -> CorpusResult<marble::Config> {
        let pth = env::var("MARBLE_PATH").ok().unwrap_or("corpus".to_string());
        let path = PathBuf::from(pth);
        let zstd_compression_level = match env::var("MARBLE_ZSTD_COMPRESSION_LEVEL").ok() {
            None => None,
            Some(v) => Some(v.parse::<i32>().map_err(|_| {
                CorpusError::ConfigurationError("Invalid MARBLE_ZSTD_COMPRESSION_LEVEL".to_string())
            })?),
        };
        let fsync_each_batch = true;
        let target_file_size = env_default!("TARGET_FILE_SIZE", 512_000_000usize, usize);
        let file_compaction_percent = env_default!("FILE_COMPACTION_SIZE", 20u8, u8);
        let max_object_size = env_default!("MAX_OBJECT_SIZE", 1_024_000usize, usize);
        let small_file_cleanup_threshold =
            env_default!("SMALL_FILE_CLEANUP_THRESHOLD", 128usize, usize);
        let min_compaction_files = env_default!("MIN_COMPACTION_FILES", 128usize, usize);
        Ok(marble::Config {
            path,
            zstd_compression_level,
            fsync_each_batch,
            target_file_size,
            file_compaction_percent,
            max_object_size,
            small_file_cleanup_threshold,
            min_compaction_files,
            partition_function: pf,
        })
    }
    pub(crate) fn new(config: marble::Config) -> CorpusResult<Self> {
        let db = config
            .open()
            .map_err(|e| CorpusError::BackingStorageError(e))?;
        let cs = WriteState {
            author_id: 0,
            collection_id: 0,
            document_id: 0,
            token_id: 0,
            db,
        };
        CorpusState::_new(cs)
    }
    pub(crate) fn default() -> CorpusResult<Self> {
        Self::new(Self::default_config()?)
    }

    fn author_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<WriteState> = self.lock();
        Ok(self
            ._read_lock("Author id lock error".to_string())?
            .deref()
            .author_id)
    }
    fn next_author_id(&self) -> CorpusResult<u64> {
        let mut st = self._write_lock("Next author id lock error".to_string())?;
        let mut author_id = st.borrow_mut().author_id;
        author_id = author_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Author".into()))?;
        Ok(author_id.clone())
    }
    fn collection_id(&self) -> CorpusResult<u64> {
        Ok(self
            ._read_lock("Collection id lock error".to_string())?
            .deref()
            .collection_id)
    }
    fn next_collection_id(&self) -> CorpusResult<u64> {
        let mut st = self._write_lock("Next collection id lock error".to_string())?;
        let mut collection_id = st.borrow_mut().author_id;
        collection_id = collection_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Collection".into()))?;
        Ok(collection_id.clone())
    }
    fn document_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<WriteState> = self.lock();
        Ok(self
            ._read_lock("Document id lock error".to_string())?
            .deref()
            .document_id)
    }
    fn next_document_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<WriteState> = self.lock();
        let mut st = st
            .write()
            .map_err(|_| CorpusError::LockError("Next document id lock error".to_string()))?;
        let mut document_id = st.borrow_mut().author_id;
        document_id = document_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Document".into()))?;
        Ok(document_id.clone())
    }
    fn token_id(&self) -> CorpusResult<u64> {
        Ok(self
            ._read_lock("Token id lock error".to_string())?
            .deref()
            .token_id)
    }
    fn next_token_id(&self) -> CorpusResult<u64> {
        let mut st = self._write_lock("Next token id lock error".to_string())?;
        let mut token_id = st.borrow_mut().author_id;
        token_id = token_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Token".into()))?;
        Ok(token_id.clone())
    }
    fn _read_lock(&self, msg: String) -> CorpusResult<std::sync::RwLockReadGuard<'_, WriteState>> {
        self.lock().read().map_err(|_| CorpusError::LockError(msg))
    }
    fn _write_lock(
        &self,
        msg: String,
    ) -> CorpusResult<std::sync::RwLockWriteGuard<'_, WriteState>> {
        self.lock().write().map_err(|_| CorpusError::LockError(msg))
    }
}

impl CorpusWrite for CorpusState<WriteState> {
    fn write_objs(&self, objs: impl AsRef<[CorpusEntity]>) -> CorpusResult<()> {
        let objs = objs.as_ref();
        let mut updates: BTreeMap<u64, Vec<(u64, CorpusEntity)>> = BTreeMap::new();
        for obj in objs {
            let (page_id, obj_id) = obj.obj_id();
            updates
                .entry(page_id)
                .and_modify(|entries| entries.push((obj_id, *obj)))
                .or_insert(vec![(obj_id, *obj)]);
        }
        let batch = {
            let mut batch: Vec<(u64, Option<Vec<u8>>)> = Vec::with_capacity(objs.len());
            // lock will be dropped at end of block so getting the write lock later is ok
            let s = self._read_lock("Read lock error retrieving pages for updates".to_string())?;
            let st = s.borrow();
            for (page_id, entries) in updates.into_iter() {
                let page = st.db.read(page_id)?;
                if let Some(raw) = page {
                    let mut page = minicbor::decode::<Page>(raw.deref()).map_err(|_| {
                        CorpusError::DecodingError(format!("Decoding page {page_id}"))
                    })?;
                    for (id, obj) in entries.iter() {
                        page.0.insert(*id, *obj);
                    }
                    batch.push((page_id, Some(page.to_bytes()?)));
                } else {
                    let entries = entries.into_iter().collect::<BTreeMap<u64, CorpusEntity>>();
                    batch.push((page_id, Some(Page(entries).to_bytes()?)));
                };
            }
            batch
        };
        let mut st = self._write_lock("Write lock error".to_string())?;
        st.borrow_mut()
            .db
            .write_batch(batch)
            .map_err(|e| CorpusError::BackingStorageError(e))?;
        Ok(())
    }
}
