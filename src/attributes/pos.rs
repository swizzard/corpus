use crate::attributes::Attributes;
use enum_iterator::{all, Sequence};

#[repr(u128)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Sequence)]
pub enum PosAttrs {
    ZERO = 0x0000_0000_0000_0000,

    PosCC = 0x8000_0000_0000_0000,
    PosCD = 0x4000_0000_0000_0000,
    PosDT = 0x2000_0000_0000_0000,
    PosEX = 0x1000_0000_0000_0000,

    PosFW = 0x0800_0000_0000_0000,
    PosIN = 0x0400_0000_0000_0000,
    PosJJ = 0x0200_0000_0000_0000,
    PosJJR = 0x0100_0000_0000_0000,

    PosJS = 0x0080_0000_0000_0000,
    PosMD = 0x0040_0000_0000_0000,
    PosNN = 0x0020_0000_0000_0000,
    PosNNP = 0x0010_0000_0000_0000,

    PosNNPS = 0x0008_0000_0000_0000,
    PosNNS = 0x0004_0000_0000_0000,
    PosPDT = 0x0002_0000_0000_0000,
    PosPOS = 0x0001_0000_0000_0000,

    PosPRP = 0x0000_8000_0000_0000,
    PosRB = 0x0000_4000_0000_0000,
    PosRBR = 0x0000_2000_0000_0000,
    PosRBS = 0x0000_1000_0000_0000,

    PosRP = 0x0000_0800_0000_0000,
    PosSYM = 0x0000_0400_0000_0000,
    PosTO = 0x0000_0200_0000_0000,
    PosUH = 0x0000_0100_0000_0000,

    PosVB = 0x0000_0080_0000_0000,
    PosVBD = 0x0000_0040_0000_0000,
    PosVBG = 0x0000_0020_0000_0000,
    PosVBN = 0x0000_0010_0000_0000,

    PosVBP = 0x0000_0008_0000_0000,
    PosVBZ = 0x0000_0004_0000_0000,
    PosWDT = 0x0000_0002_0000_0000,
    PosWP = 0x0000_0001_8000_0000,

    PosWRB = 0x0000_0000_8000_0000,
}

const POS: [PosAttrs; 34] = [
    PosAttrs::ZERO,
    PosAttrs::PosCC,
    PosAttrs::PosCD,
    PosAttrs::PosDT,
    PosAttrs::PosEX,
    PosAttrs::PosFW,
    PosAttrs::PosIN,
    PosAttrs::PosJJ,
    PosAttrs::PosJJR,
    PosAttrs::PosJS,
    PosAttrs::PosMD,
    PosAttrs::PosNN,
    PosAttrs::PosNNP,
    PosAttrs::PosNNPS,
    PosAttrs::PosNNS,
    PosAttrs::PosPDT,
    PosAttrs::PosPOS,
    PosAttrs::PosPRP,
    PosAttrs::PosRB,
    PosAttrs::PosRBR,
    PosAttrs::PosRBS,
    PosAttrs::PosRP,
    PosAttrs::PosSYM,
    PosAttrs::PosTO,
    PosAttrs::PosUH,
    PosAttrs::PosVB,
    PosAttrs::PosVBD,
    PosAttrs::PosVBG,
    PosAttrs::PosVBN,
    PosAttrs::PosVBG,
    PosAttrs::PosVBZ,
    PosAttrs::PosWDT,
    PosAttrs::PosWP,
    PosAttrs::PosWRB,
];

pub struct PosAttributes {}

impl Attributes for PosAttributes {
    type Attrs = PosAttrs;

    fn deserialize(&self, val: u128) -> Result<Vec<PosAttrs>, String> {
        Ok(all::<PosAttrs>()
            .map(|p| {
                let u = p as u128;
                if val & u == u {
                    Some(p)
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<PosAttrs>>())
    }
    fn serialize(&self, attrs: Vec<PosAttrs>) -> Result<u128, String> {
        Ok(attrs
            .iter()
            .fold(PosAttrs::ZERO as u128, |acc: u128, p: &PosAttrs| {
                acc | (*p as u128)
            }))
    }
}
