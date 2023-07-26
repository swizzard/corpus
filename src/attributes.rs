use binary_layout::LayoutAs;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TokenAttributes(u128);

impl LayoutAs<u128> for TokenAttributes {
    fn read(v: u128) -> TokenAttributes {
        TokenAttributes(v)
    }
    fn write(v: TokenAttributes) -> u128 {
        v.0
    }
}

pub(crate) trait Attributes {
    type Attrs;

    fn get(&self, attr: <Self as Attributes>::Attrs) -> bool;
    fn set(&mut self, attr: <Self as Attributes>::Attrs, value: bool);
}

pub enum TokenAttrs {
    ZERO = 0xFFFF_FFFF,
    Pos0 = 1,
    PosCC = 2,
    PosCD = 4,
    PosDT = 8,
    PosEX = 16,
    PosFW = 32,
    //     "6": "IN",
    //     "7": "JJ",
    //     "8": "JJR",
    //     "9": "JJS",
    //     "10": "MD",
    //     "11": "NN",
    //     "12": "NNP",
    //     "13": "NNPS",
    //     "14": "NNS",
    //     "15": "PDT",
    //     "16": "POS",
    //     "17": "PRP",
    //     "18": "RB",
    //     "19": "RBR",
    //     "20": "RBS",
    //     "21": "RP",
    //     "22": "SYM",
    //     "23": "TO",
    //     "24": "UH",
    //     "25": "VB",
    //     "26": "VBD",
    //     "27": "VBG",
    //     "28": "VBN",
    //     "29": "VBP",
    //     "30": "VBZ",
    //     "31": "WDT",
    //     "32": "WP",
    //     "33": "WRB"
}

// }  "id2label": {
//     "0": "O",
//     "1": "CC",
//     "2": "CD",
//     "3": "DT",
//     "4": "EX",
//     "5": "FW",
//     "6": "IN",
//     "7": "JJ",
//     "8": "JJR",
//     "9": "JJS",
//     "10": "MD",
//     "11": "NN",
//     "12": "NNP",
//     "13": "NNPS",
//     "14": "NNS",
//     "15": "PDT",
//     "16": "POS",
//     "17": "PRP",
//     "18": "RB",
//     "19": "RBR",
//     "20": "RBS",
//     "21": "RP",
//     "22": "SYM",
//     "23": "TO",
//     "24": "UH",
//     "25": "VB",
//     "26": "VBD",
//     "27": "VBG",
//     "28": "VBN",
//     "29": "VBP",
//     "30": "VBZ",
//     "31": "WDT",
//     "32": "WP",
//     "33": "WRB"
//   },
