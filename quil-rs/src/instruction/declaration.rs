use std::{fmt, str::FromStr};

use nom_locate::LocatedSpan;

#[cfg(test)]
use proptest_derive::Arbitrary;

use crate::{
    parser::{common::parse_memory_reference, lex, ParseError},
    program::{disallow_leftover, SyntaxError},
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ScalarType {
    Bit,
    Integer,
    Octet,
    Real,
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ScalarType::*;
        write!(
            f,
            "{}",
            match self {
                Bit => "BIT",
                Integer => "INTEGER",
                Octet => "OCTET",
                Real => "REAL",
            }
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Vector {
    pub data_type: ScalarType,
    pub length: u64,
}

impl Vector {
    pub fn new(data_type: ScalarType, length: u64) -> Self {
        Self { data_type, length }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.data_type, self.length)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offset {
    pub offset: u64,
    pub data_type: ScalarType,
}

impl Offset {
    pub fn new(offset: u64, data_type: ScalarType) -> Self {
        Self { offset, data_type }
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.offset, self.data_type)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declaration {
    pub name: String,
    pub size: Vector,
    pub sharing: Option<String>,
    pub offsets: Vec<Offset>,
}

impl Declaration {
    pub fn new(name: String, size: Vector, sharing: Option<String>, offsets: Vec<Offset>) -> Self {
        Self {
            name,
            size,
            sharing,
            offsets,
        }
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DECLARE {} {}", self.name, self.size)?;
        if let Some(shared) = &self.sharing {
            write!(f, "SHARING {shared}")?;
            if !self.offsets.is_empty() {
                write!(f, "OFFSET")?;
                for offset in self.offsets.iter() {
                    write!(f, " {offset}")?
                }
            }
        }
        Ok(())
    }
}

// TODO: Tests for fmt::Display
// #[cfg(test)]
// mod test_declaration {
//     use super::{Declaration, Vector, Offset, ScalarType};
//     use rstest::rstest;
//
//     #[rstest]
//     #[case(
//         "ro",
//         Vector{ data_type: ScalarType::Bit, length: 1 }
//     )]
// }

#[derive(Clone, Debug, Hash, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MemoryReference {
    pub name: String,
    pub index: u64,
}

impl MemoryReference {
    pub fn new(name: String, index: u64) -> Self {
        Self { name, index }
    }
}

impl Eq for MemoryReference {}

impl fmt::Display for MemoryReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.name, self.index)
    }
}

impl FromStr for MemoryReference {
    type Err = SyntaxError<Self>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = LocatedSpan::new(s);
        let tokens = lex(input)?;
        disallow_leftover(
            parse_memory_reference(&tokens).map_err(ParseError::from_nom_internal_err),
        )
    }
}
