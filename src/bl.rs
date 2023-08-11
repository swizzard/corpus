use crate::errors::{CorpusError, CorpusResult};
use minicbor::{Decode, Encode};

#[repr(u64)]
pub enum ObjType {
    Author = 0x0000_0000_0000_0000,
    Collection = 0x1000_0000_0000_0000,
    Document = 0x2000_0000_0000_0000,
    Token = 0x3000_0000_0000_0000,
    StringRef = 0x4000_0000_0000_0000,
}

pub type Id = [u8; 16];

pub(crate) trait HasId {
    fn id(&self) -> u128;
}

pub(crate) trait HasType {
    fn obj_type(&self) -> ObjType;
}

pub(crate) trait HasObjId: HasId + HasType {
    /// first u64 is marble id
    /// 1st byte of 2nd u64 is obj type
    /// remaining 15 bytes are lowest 15 bytes of obj id
    /// ```
    /// let t = Token {
    ///     id: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    ///     document_id: [1u8;16],
    ///     author_id: [2u8;16],
    ///     line: 1,
    ///     position: 1,
    ///     text: StringRef { start: 0, length: 5 },
    ///     labels: [0u8;16]
    ///     };
    /// let (oh, ol) = Token.obj_id();
    /// assert_eq!(oh, 0b0000_0000_0000_0000);
    /// assert_eq!(ol, 0b011_0000_0000_0001);
    /// ```
    fn obj_id(&self) -> (u64, u64) {
        let id = self.id();
        let t = self.obj_type() as u64;
        let bytes = id.to_be_bytes();
        let (h, l) = bytes.split_at(8);
        let h = u64::from_be_bytes(h.try_into().unwrap());
        let l = u64::from_be_bytes(l.try_into().unwrap());
        let l = l & 0x0111_1111_1111_1111 | t;
        (h, l)
    }
}

#[derive(Debug, Decode, Encode, Eq, PartialEq)]
pub struct StringRef {
    #[n(0)]
    start: u64,
    #[n(1)]
    length: u64,
}

impl StringRef {
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

#[derive(Debug, Decode, Encode)]
pub struct Token {
    #[n(0)]
    id: Id,
    #[n(1)]
    document_id: Id,
    #[n(2)]
    author_id: Id,
    #[n(3)]
    line: u64,
    #[n(4)]
    position: u64,
    #[n(5)]
    text: StringRef,
    #[n(6)]
    labels: [u8; 16],
}

impl HasId for Token {
    fn id(&self) -> u128 {
        u128_id(&self.id)
    }
}

impl HasType for Token {
    fn obj_type(&self) -> ObjType {
        ObjType::Token
    }
}

impl HasObjId for Token {}

#[derive(Debug, Decode, Encode)]
pub struct Document {
    #[n(0)]
    id: Id,
    #[n(1)]
    author_id: Id,
    #[n(2)]
    collection_id: Id,
    #[n(3)]
    date: u64,
    #[n(4)]
    title: StringRef,
}

impl HasId for Document {
    fn id(&self) -> u128 {
        u128_id(&self.id)
    }
}

impl HasType for Document {
    fn obj_type(&self) -> ObjType {
        ObjType::Document
    }
}

#[derive(Debug, Decode, Encode)]
pub struct Collection {
    #[n(0)]
    id: Id,
    #[n(1)]
    date: u64,
    #[n(2)]
    title: StringRef,
    #[n(3)]
    notes: StringRef,
}

impl HasId for Collection {
    fn id(&self) -> u128 {
        u128_id(&self.id)
    }
}

impl HasType for Collection {
    fn obj_type(&self) -> ObjType {
        ObjType::Collection
    }
}

impl HasObjId for Collection {}

#[derive(Debug, Decode, Encode)]
pub struct Author {
    #[n(0)]
    id: Id,
    #[n(1)]
    name: StringRef,
    #[n(2)]
    notes: StringRef,
}

impl HasId for Author {
    fn id(&self) -> u128 {
        u128_id(&self.id)
    }
}

impl HasType for Author {
    fn obj_type(&self) -> ObjType {
        ObjType::Author
    }
}

impl HasObjId for Author {}

fn u128_id(bytes: &[u8; 16]) -> u128 {
    u128::from_be_bytes(*bytes)
}
