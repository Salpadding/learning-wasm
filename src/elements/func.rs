use std::io;
use super::{Deserialize, Error};
use super::primitives::{VarUint32, CountedList};
use super::types::{ValueType};
use super::sections::SectionReader;
use super::ops::{Instructions};

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

/// Function body definition.
#[derive(Debug, Clone, PartialEq)]
pub struct FuncBody {
    pub locals: Vec<Local>,
    pub instructions: Instructions,
}

impl Deserialize for FuncBody {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let mut body_reader = SectionReader::new(reader)?;
        let locals: Vec<Local> = CountedList::<Local>::deserialize(&mut body_reader)?.into_inner();

        // The specification obliges us to count the total number of local variables while
        // decoding the binary format.
        locals
            .iter()
            .try_fold(0u32, |acc, &Local { count, .. }| acc.checked_add(count))
            .ok_or_else(|| Error::TooManyLocals)?;

        let instructions = Instructions::deserialize(&mut body_reader)?;
        body_reader.close()?;
        Ok(FuncBody { locals: locals, instructions: instructions })
    }
}



