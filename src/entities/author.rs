use crate::entities::{u128_id, HasId, HasObjId, HasType, Id, ObjType, StringRef};
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
