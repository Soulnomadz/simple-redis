mod encode;
mod decode;

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    NullArray(RespNullArray),
    Null(RespNull),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct SimpleString(String);

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct SimpleError(String);

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct BulkString(Vec<u8>);

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct RespNullBulkString;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespArray(Vec<RespFrame>);

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct RespNullArray;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct RespNull;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespMap(BTreeMap<String, RespFrame>);

impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}

impl Default for RespMap {
    fn default() -> Self {
        RespMap::new()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespSet(Vec<RespFrame>);

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}


#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
}

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl From<SimpleString> for RespFrame {
//     fn from(s: SimpleString) -> Self {
//         RespFrame::SimpleString(s)
//     }
// }

// impl From<SimpleError> for RespFrame {
//     fn from(s: SimpleError) -> Self {
//         RespFrame::Error(s)
//     }
// }

// impl From<i64> for RespFrame {
//     fn from(s: i64) -> Self {
//         RespFrame::Integer(s)
//     }
// }


// impl From<BulkString> for RespFrame {
//     fn from(s: BulkString) -> Self {
//         RespFrame::BulkString(s)
//     }
// }

// impl From<RespNullBulkString> for RespFrame {
//     fn from(s: RespNullBulkString) -> Self {
//         RespFrame::NullBulkString(s)
//     }
// }

// impl From<RespArray> for RespFrame {
//     fn from(s: RespArray) -> Self {
//         RespFrame::Array(s)
//     }
// }

// impl From<RespNullArray> for RespFrame {
//     fn from(s: RespNullArray) -> Self {
//         RespFrame::NullArray(s)
//     }
// }

// impl From<RespNull> for RespFrame {
//     fn from(s: RespNull) -> Self {
//         RespFrame::Null(s)
//     }
// }

// impl From<bool> for RespFrame {
//     fn from(s: bool) -> Self {
//         RespFrame::Boolean(s)
//     }
// }

// impl From<f64> for RespFrame {
//     fn from(s: f64) -> Self {
//         RespFrame::Double(s)
//     }
// }

// impl From<RespMap> for RespFrame {
//     fn from(s: RespMap) -> Self {
//         RespFrame::Map(s)
//     }
// }

// impl From<RespSet> for RespFrame {
//     fn from(s: RespSet) -> Self {
//         RespFrame::Set(s)
//     }
// }

