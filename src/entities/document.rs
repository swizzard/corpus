use crate::entities::{u128_id, HasId, HasType, Id, ObjType, StringRef};
use minicbor::{Decode, Encode};

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
