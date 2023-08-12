use crate::entities::{u128_id, HasId, HasObjId, HasType, Id, ObjType, StringRef};
use minicbor::{Decode, Encode};

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
