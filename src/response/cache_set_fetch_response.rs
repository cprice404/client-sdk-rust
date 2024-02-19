use std::collections::HashSet;
use std::convert::TryFrom;
use crate::MomentoError;

pub enum SetFetch {
    Hit { value: SetFetchValue },
    Miss,
}

pub struct SetFetchValue {
    pub(crate) raw_item: Vec<Vec<u8>>,
}

impl SetFetchValue {
    pub fn value(self: Self) -> HashSet<Vec<u8>> {
        self.raw_item.into_iter().collect()
    }
}
//
// #[derive(Debug)]
// #[non_exhaustive]
// pub struct MomentoSetFetchResponse {
//     pub value: Option<HashSet<Vec<u8>>>,
// }

impl TryFrom<SetFetchValue> for HashSet<Vec<u8>> {
    type Error = MomentoError;

    fn try_from(value: SetFetchValue) -> Result<Self, Self::Error> {
        Ok(value.value())
    }
}


impl TryFrom<SetFetchValue> for HashSet<String> {
    type Error = MomentoError;

    fn try_from(value: SetFetchValue) -> Result<Self, Self::Error> {
        Ok(value.raw_item.into_iter().map(|v| String::from_utf8(v).unwrap()).collect())
    }
}
