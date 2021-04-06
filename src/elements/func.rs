use std::io;
use super::{Deserialize, Error};
use super::primitives::VarUint32;
use super::types::{ValueType};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Func(pub u32);

impl Deserialize for Func {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Func, Error> {
        let u: u32 = VarUint32::deserialize(reader)?.into();
        Ok(Func(u))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Local {
    pub count: u32,
    pub value_type: ValueType,
}

impl Deserialize for Local {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Local, Error> {
        let count: u32 = VarUint32::deserialize(reader)?.into();
        let value_type = ValueType::deserialize(reader)?;
        Ok (
            Local { count, value_type}
        )
    }
}


