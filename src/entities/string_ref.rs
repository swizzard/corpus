use crate::entities::{HasId, HasObjId, HasType, ObjType};
use crate::errors::{CorpusError, CorpusResult};
use minicbor::{Decode, Encode};

#[derive(Copy, Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub struct StringRef {
    #[n(0)]
    pub start: u64,
    #[n(1)]
    length: u64,
}

impl StringRef {
    pub fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }
    pub fn start(&self) -> CorpusResult<usize> {
        self.start
            .try_into()
            .map_err(|_| CorpusError::StringNotFoundError(self.start, self.length))
    }
    pub fn end(&self) -> CorpusResult<usize> {
        let start: usize = self
            .start
            .try_into()
            .map_err(|_| CorpusError::StringNotFoundError(self.start, self.length))?;
        let length: usize = self
            .length
            .try_into()
            .map_err(|_| CorpusError::StringNotFoundError(self.start, self.length))?;
        start
            .checked_add(length)
            .ok_or(CorpusError::StringNotFoundError(self.start, self.length))
    }
    pub(crate) fn hydrate(
        &self,
        strings: &crate::entities::strings::Strings,
    ) -> CorpusResult<String> {
        strings.get_string(&self)
    }
    pub(crate) fn dehydrate(
        &self,
        string: String,
        strings: &super::strings::Strings,
    ) -> CorpusResult<Self> {
        todo!()
    }
}

impl HasId for StringRef {
    fn id(&self) -> u128 {
        ((self.start as u128) << 64) & self.length as u128
    }
}

impl HasType for StringRef {
    fn obj_type(&self) -> ObjType {
        ObjType::StringRef
    }
}

impl HasObjId for StringRef {}

impl PartialOrd for StringRef {
    fn partial_cmp(&self, other: &StringRef) -> Option<std::cmp::Ordering> {
        if self.start < other.start {
            Some(std::cmp::Ordering::Less)
        } else if self.start > other.start {
            Some(std::cmp::Ordering::Greater)
        } else {
            self.length.partial_cmp(&other.length)
        }
    }
}
impl Ord for StringRef {
    fn cmp(&self, other: &StringRef) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
