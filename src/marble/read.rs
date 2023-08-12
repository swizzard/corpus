use crate::entities::{CorpusEntity, Id};
use crate::env_default;
use crate::errors::{CorpusError, CorpusResult};
use crate::marble::{pf, CorpusRead, CorpusState, Page};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct ReadState {
    db: marble::Marble,
    cache: HashMap<Id, CorpusEntity>,
}

impl CorpusState<ReadState> {
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
        let cs = ReadState {
            db,
            cache: HashMap::new(),
        };
        CorpusState::_new(cs)
    }
    pub(crate) fn default() -> CorpusResult<Self> {
        Self::new(Self::default_config()?)
    }
    fn _read_lock(&self, msg: String) -> CorpusResult<std::sync::RwLockReadGuard<'_, ReadState>> {
        self.lock().read().map_err(|_| CorpusError::LockError(msg))
    }
    fn load_page(&self, page_id: u64) -> CorpusResult<Option<Page>> {
        if let Some(raw) = self
            ._read_lock("loading page".to_string())?
            .borrow()
            .db
            .read(page_id)
            .map_err(|e| CorpusError::BackingStorageError(e))?
        {
            let decoded = minicbor::decode::<Page>(&raw)
                .map_err(|_| CorpusError::DecodingError("loading page".to_string()))?;
            Ok(Some(decoded))
        } else {
            Ok(None)
        }
    }
    // fn cache_page(&self, page: Page) {
    //
    //
}

impl CorpusRead for CorpusState<ReadState> {
    fn read_obj(&self, obj_id: Id) -> CorpusResult<CorpusEntity> {
        todo!()
    }
    fn read_objs(&self, obj_ids: impl AsRef<[Id]>) -> CorpusResult<Vec<CorpusEntity>> {
        todo!()
    }
}
