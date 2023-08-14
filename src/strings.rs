use std::path::Path;
use std::str;

use crate::entities::StringRef;
use crate::errors::{CorpusError, CorpusResult};

#[derive(Debug, Default, PartialEq)]
pub struct Strings(Vec<u8>);

impl Strings {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn append(&mut self, slice: &[u8]) {
        self.0.extend_from_slice(slice);
    }
    fn gs(&self, start: usize, end: usize) -> CorpusResult<String> {
        let arr = self.gb(start, end)?;
        let s = str::from_utf8(arr).map_err(|_| CorpusError::InvalidStringError(start, end))?;
        Ok(String::from(s))
    }
    fn gb(&self, start: usize, end: usize) -> CorpusResult<&[u8]> {
        self.0
            .get(start..=end)
            .ok_or(CorpusError::StringNotFoundError(start as u64, end as u64))
    }
    pub fn get_string<S>(&self, string_ref: &StringRef) -> CorpusResult<String>
    where
        S: AsRef<[u8]>,
    {
        self.gs(string_ref.start()?, string_ref.end()?)
    }
    pub(crate) fn get_bytes<S>(&self, string_ref: &StringRef) -> CorpusResult<&[u8]>
    where
        S: AsRef<[u8]>,
    {
        self.gb(string_ref.start()?, string_ref.end()?)
    }
    pub fn from_file<P>(f: P) -> CorpusResult<Self>
    where
        P: AsRef<Path>,
    {
        std::fs::read(f)
            .map(|a| Self(a))
            .map_err(|e| CorpusError::BackingStorageError(e))
    }
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        Self(Vec::from(bytes))
    }
    #[cfg(test)]
    pub fn _test_contents(&self) -> &[u8] {
        self.0.as_slice()
    }
    #[cfg(test)]
    pub fn _test_gs(&self, start: usize, end: usize) -> CorpusResult<String> {
        self.gs(start, end)
    }
    #[cfg(test)]
    pub fn _test_from_str(s: &str) -> Self {
        Self(Vec::from(s.as_bytes()))
    }
    #[cfg(test)]
    pub fn _test_from_vec(s: Vec<u8>) -> Self {
        Self(s)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct StringsUpdate {
    old: (u64, u64),
    new: (u64, u64),
}

impl StringsUpdate {}
// todo: update string_ref::View.{start,end}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strings_append() {
        let mut s = Strings::default();
        let t: &[u8] = "hello".as_bytes();
        s.append(t);
        assert_eq!(s._test_contents(), t);
    }
    #[test]
    fn strings_get_string() -> CorpusResult<()> {
        let s = Strings::_test_from_str("hellothere");
        assert_eq!(s._test_gs(5, 9)?, String::from("there"));
        Ok(())
    }
    #[test]
    fn strings_get_string_oob() {
        let s = Strings::_test_from_str("hellothere");
        match s._test_gs(6, 10) {
            Ok(_) => panic!("oob test failed"),
            Err(CorpusError::StringNotFoundError(st, en)) if st == 6 && en == 10 => (),
            Err(e) => panic!("oob test wrong error {e:?}"),
        }
    }
    #[test]
    fn strings_get_string_non_utf8() {
        let s = Strings::_test_from_vec(vec![0xc0; 10]);
        match s._test_gs(5, 9) {
            Ok(s) => panic!("non_utf8 test failed, got {s:?}"),
            Err(CorpusError::InvalidStringError(st, en)) if st == 5 && en == 9 => (),
            Err(e) => panic!("non_utf8 test wrong error {e:?}"),
        }
    }
}
