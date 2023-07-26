use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::bl::*;
use crate::strings::{CorpusError, CorpusResult, Strings};

fn uuid_path(val: &u128) -> Option<PathBuf> {
    val.to_be_bytes()
        .chunks(4)
        .take(3)
        .map(|ch| {
            Path::new(
                u32::from_be_bytes(ch.try_into().unwrap())
                    .to_string()
                    .as_str(),
            )
            .to_path_buf()
        })
        .reduce(|mut acc, el| {
            acc.push(el);
            acc
        })
}

fn data_file_path(path: PathBuf) -> PathBuf {
    path.as_path().join(Path::new("data").to_path_buf())
}
fn strings_file_path(path: PathBuf) -> PathBuf {
    path.as_path().join(Path::new("strings").to_path_buf())
}

fn load_strings(dir_pth: PathBuf) -> CorpusResult<Strings> {
    let p = strings_file_path(dir_pth);
    Strings::from_file(p)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_uuid_path_max() {
        let v = u128::MAX;
        let expected_path =
            Path::new(format!("{m}/{m}/{m}", m = 0xFFFFFFFF).as_str()).to_path_buf();
        assert_eq!(uuid_path(&v).unwrap(), expected_path);
    }
    #[test]
    fn test_uuid_path_other() {
        let v: u128 = u128::MAX - 0x000000F0;
        let expected_path =
            Path::new(format!("{m}/{m}/{m}", m = 0xFFFFFFFF, o = 0xFFFFFF0F).as_str())
                .to_path_buf();
        assert_eq!(uuid_path(&v).unwrap(), expected_path);
    }
}
