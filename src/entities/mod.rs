use crate::errors::{CorpusError, CorpusResult};
use chrono::{DateTime, TimeZone, Utc};
use minicbor::{Decode, Encode};
use num;
use num_derive::FromPrimitive;
use serde_derive::{Deserialize, Serialize};

pub(crate) mod author;
pub(crate) mod collection;
pub(crate) mod document;
pub(crate) mod string_ref;
pub(crate) mod strings;
pub(crate) mod token;

pub use author::Author;
pub use collection::Collection;
pub use document::Document;
pub use string_ref::StringRef;
pub use token::Token;

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
    pub(crate) fn encode(&self) -> CorpusResult<Vec<u8>> {
        let mut b = Vec::with_capacity(self.len());
        match self {
            Self::Author(ref a) => minicbor::encode::<&Author, &mut Vec<u8>>(a, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self)))?,
            Self::Collection(ref c) => minicbor::encode::<&Collection, &mut Vec<u8>>(c, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self)))?,
            Self::Document(ref d) => minicbor::encode::<&Document, &mut Vec<u8>>(d, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self)))?,
            Self::StringRef(ref s) => {
                minicbor::encode::<&StringRef, &mut Vec<u8>>(s, b.as_mut())
                    .map_err(|_| CorpusError::EncodingError(format!("{:?}", self)))?
            }
            Self::Token(ref t) => minicbor::encode::<&Token, &mut Vec<u8>>(t, b.as_mut())
                .map_err(|_| CorpusError::EncodingError(format!("{:?}", self)))?,
        };
        Ok(b)
    }
    pub fn obj_id(&self) -> (u64, u64) {
        obj_id(self.id(), self.obj_type())
    }
    pub(crate) fn page_id(&self) -> u64 {
        self.obj_id().0
    }
    pub(crate) fn hydrate(
        &self,
        strings: &crate::entities::strings::Strings,
    ) -> CorpusResult<HydratedEntity> {
        match self {
            Self::Author(ref a) => a.hydrate(strings),
            Self::Collection(ref c) => c.hydrate(strings),
            Self::Document(ref d) => d.hydrate(strings),
            Self::Token(ref t) => t.hydrate(strings),
            Self::StringRef(_) => Err(CorpusError::DecodingError(
                "Cannot hydrate string_ref".to_string(),
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HydratedEntity {
    Author(author::HydratedAuthor),
    Collection(collection::HydratedCollection),
    Document(document::HydratedDocument),
    Token(token::HydratedToken),
}

impl HydratedEntity {
    pub fn obj_id(&self) -> (u64, u64) {
        match self {
            Self::Author(ref a) => a.obj_id(),
            Self::Collection(ref c) => c.obj_id(),
            Self::Document(ref d) => d.obj_id(),
            Self::Token(ref t) => t.obj_id(),
        }
    }
}

#[repr(u64)]
#[derive(Debug, Eq, FromPrimitive, PartialEq, Ord, PartialOrd)]
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
    fn obj_id(&self) -> (u64, u64) {
        obj_id(self.id(), self.obj_type())
    }
}

pub fn u128_id(bytes: &[u8; 16]) -> u128 {
    u128::from_be_bytes(*bytes)
}

/// first u64 is marble id
/// 1st byte of 2nd u64 is obj type
/// remaining 15 bytes are lowest 15 bytes of obj id
pub fn obj_id(id: u128, t: ObjType) -> (u64, u64) {
    let t = dbg!(t as u64);
    let bytes = id.to_be_bytes();
    let (h, l) = bytes.split_at(8);
    let h = u64::from_be_bytes(h.try_into().unwrap());
    let l = u64::from_be_bytes(l.try_into().unwrap());
    let l = dbg!(l | t);
    (h, l)
}

pub fn id_to_u128(id: Id) -> u128 {
    u128::from_be_bytes(id)
}

pub fn parse_obj_id(oid: u64) -> CorpusResult<ObjType> {
    let t = num::FromPrimitive::from_u64(oid & 0xF000_0000_0000_0000)
        .ok_or(CorpusError::InvalidEntityTypeError)?;
    Ok(t)
}

pub fn split_id(id: Id) -> CorpusResult<(u64, u64)> {
    let (h, l) = id.split_at(8);
    let h = h
        .try_into()
        .map_err(|_| CorpusError::InvalidDataError(format!("{h:?}")))?;
    let h = u64::from_be_bytes(h);
    let l = l
        .try_into()
        .map_err(|_| CorpusError::InvalidDataError(format!("{l:?}")))?;
    let l = u64::from_be_bytes(l);
    Ok((h, l))
}

pub(crate) fn parse_date(date: &u64) -> CorpusResult<DateTime<Utc>> {
    Utc.timestamp_opt(*date as i64, 0)
        .earliest()
        .ok_or(CorpusError::DecodingError(
            "Invalid collection date".to_string(),
        ))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_obj_id() {
        let token_id = 0x0000_0000_0000_0001u128;
        let obj_type = ObjType::Token;
        let (oh, ol) = obj_id(token_id, obj_type);
        assert_eq!(oh, 0x0000_0000_0000_0000);
        assert_eq!(ol, 0x3000_0000_0000_0001);
    }
}
