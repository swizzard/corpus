use crate::entities::{u128_id, HasId, HasObjId, HasType, Id, ObjType, StringRef};
use minicbor::{Decode, Encode};

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
