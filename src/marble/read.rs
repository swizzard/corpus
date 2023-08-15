use crate::entities;
use crate::entities::strings::Strings;
use crate::entities::{id_to_u128, CorpusEntity, Id};
use crate::env_default;
use crate::errors::{CorpusError, CorpusResult};
use crate::marble::{pf, CorpusRead, CorpusState, Page};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct ReadState {
    db: marble::Marble,
    cache: HashMap<u128, CorpusEntity>,
    pages: BTreeSet<u64>,
    strings_cache: HashMap<u64, Strings>,
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
            pages: BTreeSet::new(),
            strings_cache: HashMap::new(),
        };
        CorpusState::_new(cs)
    }
    pub(crate) fn default() -> CorpusResult<Self> {
        Self::new(Self::default_config()?)
    }
    fn _read_lock(&self, msg: String) -> CorpusResult<std::sync::RwLockReadGuard<'_, ReadState>> {
        self.lock().read().map_err(|_| CorpusError::LockError(msg))
    }
    fn _write_lock(&self, msg: String) -> CorpusResult<std::sync::RwLockWriteGuard<'_, ReadState>> {
        self.lock().write().map_err(|_| CorpusError::LockError(msg))
    }
    fn load_page(&self, page_id: u64) -> CorpusResult<Page> {
        if let Some(raw) = self
            ._read_lock("loading page".to_string())?
            .borrow()
            .db
            .read(page_id)
            .map_err(|e| CorpusError::BackingStorageError(e))?
        {
            let decoded = minicbor::decode::<Page>(&raw)
                .map_err(|_| CorpusError::DecodingError("loading page".to_string()))?;
            Ok(decoded)
        } else {
            Err(CorpusError::PageNotFoundError(page_id))
        }
    }
    fn cache_page(&self, page_id: u64, page: &Page) -> CorpusResult<()> {
        let mut t = self._write_lock("Caching page".to_string())?;
        let this = t.borrow_mut();
        for obj in page.0.values().into_iter() {
            this.cache.insert(obj.id(), *obj);
        }
        this.pages.insert(page_id);
        Ok(())
    }
    fn load_strings(&self, strings_page_id: u64) -> CorpusResult<Strings> {
        if let Some(raw) = self
            ._read_lock("loading strings".to_string())?
            .borrow()
            .db
            .read(strings_page_id)
            .map_err(|e| CorpusError::BackingStorageError(e))?
        {
            Ok(Strings::from_bytes(&raw))
        } else {
            Err(CorpusError::PageNotFoundError(strings_page_id))
        }
    }
    fn cache_strings(&self, strings_page_id: u64, strings: Strings) -> CorpusResult<()> {
        self._write_lock("Caching strings".to_string())?
            .borrow_mut()
            .strings_cache
            .insert(strings_page_id, strings);
        Ok(())
    }
    fn entity_from_cache(&self, id: Id) -> CorpusResult<Option<CorpusEntity>> {
        Ok(self
            ._read_lock("Accessing read cache".to_string())?
            .borrow()
            .cache
            .get(&id_to_u128(id))
            .cloned())
    }
    fn page_cached(&self, page_id: u64) -> CorpusResult<bool> {
        Ok(self
            ._read_lock("Checking page cache".to_string())?
            .borrow()
            .pages
            .get(&page_id)
            .is_some())
    }
}

impl CorpusRead for CorpusState<ReadState> {
    fn read_obj(&self, obj_id: Id) -> CorpusResult<CorpusEntity> {
        if let Some(entity) = self.entity_from_cache(obj_id)? {
            Ok(entity)
        } else {
            let (h, _) = entities::split_id(obj_id)?;
            let page = self.load_page(h)?;
            self.cache_page(h, &page)?;
            Ok(self.entity_from_cache(obj_id)?.unwrap())
        }
    }
    fn read_objs(&self, obj_ids: impl AsRef<[Id]>) -> CorpusResult<Vec<CorpusEntity>> {
        let obj_ids = obj_ids.as_ref();
        let mut cached: Vec<Id> = Vec::new();
        let mut to_find: HashMap<u64, Vec<u64>> = HashMap::new();
        for id in obj_ids {
            let (h, l) = entities::split_id(*id)?;
            if self.page_cached(h)? {
                cached.push(*id);
            } else {
                to_find
                    .entry(h)
                    .and_modify(|ids| ids.push(l))
                    .or_insert(vec![l]);
            }
        }
        let mut out: Vec<CorpusEntity> = Vec::with_capacity(obj_ids.len());
        for id in cached {
            if let Some(entity) = self.entity_from_cache(id)? {
                out.push(entity);
            } else {
                let (h, l) = entities::split_id(id)?;
                to_find
                    .entry(h)
                    .and_modify(|ids| ids.push(l))
                    .or_insert(vec![l]);
            }
        }
        for (page_id, oids) in to_find.into_iter() {
            let p = self.load_page(page_id)?;
            self.cache_page(page_id, &p)?;
            for id in oids {
                if let Some(entity) = p.0.get(&id) {
                    out.push(*entity);
                } else {
                    return Err(CorpusError::EntityNotFoundError((page_id, id)));
                }
            }
        }
        Ok(out)
    }
}
