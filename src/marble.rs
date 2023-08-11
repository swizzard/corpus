use crate::bl;
use crate::errors::{CorpusError, CorpusResult};
use marble;
use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

macro_rules! env_default {
    ($label:literal, $default:literal, $t:ty) => {
        match env::var(format!("MARBLE_{}", $label)).ok() {
            None => $default,
            Some(v) => v.parse::<$t>().map_err(|_| {
                CorpusError::ConfigurationError(format!("Invalid MARBLE_{}", $label))
            })?,
        }
    };
}

#[derive(Debug)]
struct _CorpusState {
    author_id: u64,
    collection_id: u64,
    document_id: u64,
    token_id: u64,
    db: marble::Marble,
}

#[derive(Debug)]
pub struct CorpusState {
    state: Arc<RwLock<_CorpusState>>,
}
impl CorpusState {
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
        let cs = _CorpusState {
            author_id: 0,
            collection_id: 0,
            document_id: 0,
            token_id: 0,
            db,
        };
        Ok(CorpusState {
            state: Arc::new(RwLock::new(cs)),
        })
    }
    pub(crate) fn default() -> CorpusResult<Self> {
        Self::new(Self::default_config()?)
    }
    pub fn author_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        Ok(st
            .read()
            .map_err(|_| CorpusError::LockError("Author id lock error".to_string()))?
            .deref()
            .author_id)
    }
    pub fn next_author_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        let mut st = st
            .write()
            .map_err(|_| CorpusError::LockError("Next author id lock error".to_string()))?;
        let mut author_id = st.borrow_mut().author_id;
        author_id = author_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Author".into()))?;
        Ok(author_id.clone())
    }
    pub fn collection_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        Ok(st
            .read()
            .map_err(|_| CorpusError::LockError("Collection id lock error".to_string()))?
            .deref()
            .collection_id)
    }
    pub fn next_collection_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        let mut st = st
            .write()
            .map_err(|_| CorpusError::LockError("Next collection id lock error".to_string()))?;
        let mut collection_id = st.borrow_mut().author_id;
        collection_id = collection_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Collection".into()))?;
        Ok(collection_id.clone())
    }
    pub fn document_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        Ok(st
            .read()
            .map_err(|_| CorpusError::LockError("Document id lock error".to_string()))?
            .deref()
            .document_id)
    }
    pub fn next_document_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        let mut st = st
            .write()
            .map_err(|_| CorpusError::LockError("Next document id lock error".to_string()))?;
        let mut document_id = st.borrow_mut().author_id;
        document_id = document_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Document".into()))?;
        Ok(document_id.clone())
    }
    pub fn token_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        Ok(st
            .read()
            .map_err(|_| CorpusError::LockError("Token id lock error".to_string()))?
            .deref()
            .token_id)
    }
    pub fn next_token_id(&self) -> CorpusResult<u64> {
        let st: &RwLock<_CorpusState> = self.state.borrow();
        let mut st = st
            .write()
            .map_err(|_| CorpusError::LockError("Next token id lock error".to_string()))?;
        let mut token_id = st.borrow_mut().author_id;
        token_id = token_id
            .checked_add(1)
            .ok_or(CorpusError::IdOverflowError("Token".into()))?;
        Ok(token_id.clone())
    }
}

#[allow(unused_variables)]
fn pf(object_id: u64, object_size: usize) -> u8 {
    *object_id.to_be_bytes().get(0).unwrap() << 1
}
