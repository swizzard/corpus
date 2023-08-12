use crate::entities::{CorpusEntity, Id};
use crate::errors::{CorpusError, CorpusResult};
use minicbor::{Decode, Encode};
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct CorpusState<T> {
    state: Arc<RwLock<T>>,
}
impl<T> CorpusState<T> {
    pub(crate) fn _new(state: T) -> CorpusResult<Self> {
        Ok(Self {
            state: Arc::new(RwLock::new(state)),
        })
    }
    pub(crate) fn lock(&self) -> &RwLock<T> {
        self.state.borrow()
    }
}

pub(crate) trait CorpusRead {
    fn read_obj(&self, obj_id: Id) -> CorpusResult<CorpusEntity>;
    fn read_objs(&self, obj_ids: impl AsRef<[Id]>) -> CorpusResult<Vec<CorpusEntity>>;
}

pub(crate) trait CorpusWrite {
    fn write_objs(&self, objs: impl AsRef<[CorpusEntity]>) -> CorpusResult<()>;
}

#[repr(transparent)]
#[derive(Debug, Decode, Encode, Clone)]
#[cbor(transparent)]
pub struct Page(#[n(0)] pub BTreeMap<u64, CorpusEntity>);

impl Page {
    pub fn to_bytes(&self) -> CorpusResult<Vec<u8>> {
        let mut v = Vec::with_capacity(self.0.len());
        minicbor::encode::<&Page, &mut Vec<u8>>(self, v.as_mut())
            .map_err(|_| CorpusError::EncodingError("Page encoding error".to_string()));
        Ok(v)
    }
}

#[macro_export]
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

#[allow(unused_variables)]
pub fn pf(object_id: u64, object_size: usize) -> u8 {
    *object_id.to_be_bytes().get(0).unwrap() << 1
}
