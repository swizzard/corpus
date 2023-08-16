use crate::entities::{
    parse_date, strings::Strings, u128_id, HasId, HasObjId, HasType, HydratedEntity, Id, ObjType,
    StringRef,
};
use crate::errors::CorpusResult;
use chrono::{DateTime, Utc};
use minicbor::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Decode, Encode)]
pub struct Collection {
    #[n(0)]
    id: Id, // 16
    #[n(1)]
    date: u64, // 16
    #[n(2)]
    title: StringRef, // 32
    #[n(3)]
    notes: StringRef, // 32
}

impl Collection {
    pub(crate) fn hydrate(&self, strings: &Strings) -> CorpusResult<HydratedEntity> {
        let date = parse_date(&self.date)?;
        Ok(HydratedEntity::Collection(HydratedCollection {
            id: u128_id(&self.id),
            date,
            title: self.title.hydrate(strings)?,
            notes: self.notes.hydrate(strings)?,
        }))
    }
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedCollection {
    id: u128,
    date: DateTime<Utc>,
    title: String,
    notes: String,
}
