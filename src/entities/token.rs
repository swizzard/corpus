use crate::entities::{
    strings::Strings, u128_id, HasId, HasObjId, HasType, HydratedEntity, Id, ObjType, StringRef,
};
use crate::errors::CorpusResult;
use minicbor::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Decode, Encode)]
pub struct Token {
    #[n(0)]
    id: Id, // 16
    #[n(1)]
    document_id: Id, // 16
    #[n(2)]
    author_id: Id, // 16
    #[n(3)]
    line: u64, // 16
    #[n(4)]
    position: u64, // 16
    #[n(5)]
    text: StringRef, // 32
    #[n(6)]
    labels: [u8; 16], // 16
}

impl Token {
    pub(crate) fn hydrate(&self, strings: &Strings) -> CorpusResult<HydratedEntity> {
        let id = u128_id(&self.id);
        let document_id = u128_id(&self.document_id);
        let author_id = u128_id(&self.author_id);
        let labels = Vec::from(self.labels.clone());
        Ok(HydratedEntity::Token(HydratedToken {
            id,
            document_id,
            author_id,
            line: self.line,
            position: self.position,
            text: self.text.hydrate(strings)?,
            labels,
        }))
    }
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedToken {
    id: u128,
    document_id: u128,
    author_id: u128,
    line: u64,
    position: u64,
    text: String,
    labels: Vec<u8>,
}
