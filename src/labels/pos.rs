use super::Labels;
use enum_iterator::{all, Sequence};

#[repr(u128)]
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Sequence)]
pub enum PosLbls {
    ZERO = 0x0000_0000_0000_0000,

    PosCC = 0x1000_0000_0000_0000,
    PosCD = 0x2000_0000_0000_0000,
    PosDT = 0x3000_0000_0000_0000,
    PosEX = 0x4000_0000_0000_0000,
    PosFW = 0x5000_0000_0000_0000,
    PosIN = 0x6000_0000_0000_0000,
    PosJJ = 0x7000_0000_0000_0000,
    PosJJR = 0x8000_0000_0000_0000,
    PosJS = 0x9000_0000_0000_0000,
    PosMD = 0xA000_0000_0000_0000,
    PosNN = 0xB000_0000_0000_0000,
    PosNNP = 0xC000_0000_0000_0000,
    PosNNPS = 0xD000_0000_0000_0000,
    PosNNS = 0xE000_0000_0000_0000,
    PosPDT = 0xF000_0000_0000_0000,

    PosPOS = 0xF100_0000_0000_0000,
    PosPRP = 0xF200_0000_0000_0000,
    PosRB = 0xF300_0000_0000_0000,
    PosRBR = 0xF400_0000_0000_0000,
    PosRBS = 0xF500_0000_0000_0000,
    PosRP = 0xF600_0000_0000_0000,
    PosSYM = 0xF700_0000_0000_0000,
    PosTO = 0xF800_0000_0000_0000,
    PosUH = 0xF900_0000_0000_0000,
    PosVB = 0xFA00_0000_0000_0000,
    PosVBD = 0xFB00_0000_0000_0000,
    PosVBG = 0xFC00_0000_0000_0000,
    PosVBN = 0xFD00_0000_0000_0000,
    PosVBP = 0xFE00_0000_0000_0000,
    PosVBZ = 0xFF00_0000_0000_0000,

    PosWDT = 0xFF10_0000_0000_0000,
    PosWP = 0xFF20_0000_0000_0000,
    PosWRB = 0xFF30_0000_0000_0000,
}

pub struct PosLabels {}

impl Labels for PosLabels {
    type Lbls = PosLbls;

    fn deserialize(&self, val: u128) -> Result<Vec<PosLbls>, String> {
        Ok(all::<PosLbls>()
            .map(|p| {
                let u = p as u128;
                if val & u == u {
                    Some(p)
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<PosLbls>>())
    }
    fn serialize(&self, attrs: Vec<PosLbls>) -> Result<u128, String> {
        Ok(attrs
            .iter()
            .fold(PosLbls::ZERO as u128, |acc: u128, p: &PosLbls| {
                acc | (*p as u128)
            }))
    }
}
