use crate::entities::{
    parse_date, strings::Strings, u128_id, HasId, HasObjId, HasType, HydratedEntity, Id, ObjType,
    StringRef,
};
use crate::errors::CorpusResult;
use chrono::{DateTime, Utc};
use minicbor::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Decode, Encode)]
pub struct Document {
    #[n(0)]
    id: Id, // 16
    #[n(1)]
    author_id: Id, // 16
    #[n(2)]
    collection_id: Id, // 16
    #[n(3)]
    date: u64, // 16
    #[n(4)]
    title: StringRef, // 32
}

impl Document {
    pub fn hydrate(&self, strings: &Strings) -> CorpusResult<HydratedEntity> {
        let author_id = u128_id(&self.author_id);
        let collection_id = u128_id(&self.collection_id);
        let id = u128_id(&self.id);
        let date = parse_date(&self.date)?;
        Ok(HydratedEntity::Document(HydratedDocument {
            id,
            author_id,
            collection_id,
            date,
            title: self.title.hydrate(strings)?,
        }))
    }
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

impl HasObjId for Document {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedDocument {
    id: u128,
    author_id: u128,
    collection_id: u128,
    date: DateTime<Utc>,
    title: String,
}
