use serde_derive::{Deserialize, Serialize};
use std::u128;

use crate::entities::{
    strings::Strings, u128_id, HasId, HasObjId, HasType, HydratedEntity, Id, ObjType, StringRef,
};
use crate::errors::CorpusResult;
use minicbor::{Decode, Encode};

#[derive(Copy, Clone, Debug, Decode, Encode)]
pub struct Author {
    #[n(0)]
    id: Id, // 16
    #[n(1)]
    name: StringRef, // 32
    #[n(2)]
    notes: StringRef, // 32
}

impl Author {
    pub(crate) fn hydrate(&self, strings: &Strings) -> CorpusResult<HydratedEntity> {
        Ok(HydratedEntity::Author(HydratedAuthor {
            id: u128_id(&self.id),
            name: self.name.hydrate(strings)?,
            notes: self.notes.hydrate(strings)?,
        }))
    }
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedAuthor {
    id: u128,
    name: String,
    notes: String,
}
