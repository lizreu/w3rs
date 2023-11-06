use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub fn is_valid_id_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// **Q**uad **C**haracter **C**ode
pub struct Qcc {
    id: u32,
}

impl Qcc {
    pub fn new(id: i32) -> Option<Self> {
        let is_valid = id.to_be_bytes().iter().all(|s| is_valid_id_byte(*s));

        if is_valid {
            Some(Self { id: id as u32 })
        } else {
            None
        }
    }

    /// First component of the id.
    pub fn a(&self) -> u8 {
        (self.id >> 24) as u8
    }

    /// Second component of the id.
    pub fn b(&self) -> u8 {
        (self.id >> 16) as u8
    }

    /// Third component of the id.
    pub fn c(&self) -> u8 {
        (self.id >> 8) as u8
    }

    /// Fourth component of the id.
    pub fn d(&self) -> u8 {
        self.id as u8
    }

    /// Set the first component of the id.
    pub fn set_a(&mut self, value: u8) {
        self.id = (self.id & 0x00FFFFFF) | ((value as u32) << 24);
    }

    /// Set the second component of the id.
    pub fn set_b(&mut self, value: u8) {
        self.id = (self.id & 0xFF00FFFF) | ((value as u32) << 16);
    }

    /// Set the third component of the id.
    pub fn set_c(&mut self, value: u8) {
        self.id = (self.id & 0xFFFF00FF) | ((value as u32) << 8);
    }

    /// Set the fourth component of the id.
    pub fn set_d(&mut self, value: u8) {
        self.id = (self.id & 0xFFFFFF00) | (value as u32);
    }

    pub fn to_array(&self) -> [u8; 4] {
        self.id.to_be_bytes()
    }

    pub fn from_array(array: [u8; 4]) -> Option<Self> {
        let is_valid = array.iter().all(|s| is_valid_id_byte(*s));

        if is_valid {
            Some(Self {
                id: u32::from_be_bytes(array),
            })
        } else {
            None
        }
    }

    pub fn from_slice(s: &[u8]) -> Option<Self> {
        if s.len() != 4 {
            return None;
        }

        Self::from_array([s[0], s[1], s[2], s[3]])
    }
}

impl From<Qcc> for i32 {
    fn from(id: Qcc) -> Self {
        id.id as i32
    }
}

impl From<Qcc> for u32 {
    fn from(id: Qcc) -> Self {
        id.id
    }
}

impl std::fmt::Display for Qcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.a() as char,
            self.b() as char,
            self.c() as char,
            self.d() as char
        )
    }
}

impl std::fmt::Debug for Qcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectId({})", self)
    }
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum QccParseError {
    #[error("invalid length")]
    InvalidLength,
    #[error("invalid character")]
    InvalidCharacter,
}

impl FromStr for Qcc {
    type Err = QccParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(QccParseError::InvalidLength);
        }

        let mut id = 0;

        for (i, c) in s.bytes().enumerate() {
            if !is_valid_id_byte(c) {
                return Err(QccParseError::InvalidCharacter);
            }

            id |= (c as u32) << (24 - (i * 8));
        }

        Ok(Self { id })
    }
}

#[cfg(test)]
mod test {
    use w3_derive::cc;

    use super::*;

    #[test]
    fn round_trip() {
        let id = Qcc::new(cc!("hfoo")).unwrap();
        assert!(format!("{id}") == "hfoo");
        assert!(("hfoo").parse::<Qcc>().unwrap() == id);
    }
}
