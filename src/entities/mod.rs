use crate::errors::{CorpusError, CorpusResult};
use author::Author;
use collection::Collection;
use document::Document;
use minicbor::{Decode, Encode};
use string_ref::StringRef;
use token::Token;

pub(crate) mod author;
pub(crate) mod collection;
pub(crate) mod document;
pub(crate) mod string_ref;
pub(crate) mod token;

#[derive(Copy, Clone, Debug, Decode, Encode)]
pub enum CorpusEntity {
    #[n(0)]
    Author(#[n(0)] Author),
    #[n(1)]
    Collection(#[n(0)] Collection),
    #[n(2)]
    Document(#[n(0)] Document),
    #[n(3)]
    Token(#[n(0)] Token),
    #[n(4)]
    StringRef(#[n(0)] StringRef),
}

impl CorpusEntity {
    pub fn obj_type(&self) -> ObjType {
        match self {
            Self::Author(_) => ObjType::Author,
            Self::Collection(_) => ObjType::Collection,
            Self::Document(_) => ObjType::Document,
            Self::Token(_) => ObjType::Token,
            Self::StringRef(_) => ObjType::StringRef,
        }
    }
    pub fn id(&self) -> u128 {
        match self {
            Self::Author(a) => a.id(),
            Self::Collection(c) => c.id(),
            Self::Document(d) => d.id(),
            Self::Token(t) => t.id(),
            Self::StringRef(s) => s.id(),
        }
    }
    fn len(&self) -> usize {
        match self {
            Self::Author(_) => 80,
            Self::Collection(_) => 96,
            Self::Document(_) => 96,
            Self::StringRef(_) => 32,
            Self::Token(_) => 128,
        }
    }
    pub(crate) fn encode(&self) -> CorpusResult<&[u8]> {
        let mut b = Vec::with_capacity(self.len());
        match self {
            Self::Author(ref a) => minicbor::encode::<&Author, &mut Vec<u8>>(a, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self))),
            Self::Collection(ref c) => minicbor::encode::<&Collection, &mut Vec<u8>>(c, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self))),
            Self::Document(ref d) => minicbor::encode::<&Document, &mut Vec<u8>>(d, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self))),
            Self::StringRef(ref s) => minicbor::encode::<&StringRef, &mut Vec<u8>>(s, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self))),
            Self::Token(ref t) => minicbor::encode::<&Token, &mut Vec<u8>>(t, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self))),
        };
        Ok(&b)
    }

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
    pub fn obj_id(&self) -> (u64, u64) {
        let id = self.id();
        let t = self.obj_type() as u64;
        let bytes = id.to_be_bytes();
        let (h, l) = bytes.split_at(8);
        let h = u64::from_be_bytes(h.try_into().unwrap());
        let l = u64::from_be_bytes(l.try_into().unwrap());
        let l = l & 0x0111_1111_1111_1111 | t;
        (h, l)
    }
    pub(crate) fn page_id(&self) -> u64 {
        self.obj_id().0
    }
}

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

pub(crate) trait HasObjId: HasId + HasType {}

fn u128_id(bytes: &[u8; 16]) -> u128 {
    u128::from_be_bytes(*bytes)
}
