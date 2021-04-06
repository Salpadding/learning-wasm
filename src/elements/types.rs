use super::{Deserialize, Error};
use super::primitives::{VarInt7, CountedList};

use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum ValueType {
	/// 32-bit signed integer
	I32,
	/// 64-bit signed integer
	I64,
	/// 32-bit float
	F32,
	/// 64-bit float
	F64,
}

impl Deserialize for ValueType {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<ValueType, Error> {
        let val: i8 = VarInt7::deserialize(reader)?.into();

        match val {
            -1 => Ok(ValueType::I32),
            -2 => Ok(ValueType::I64),
            -3 => Ok(ValueType::F32),
            -4 => Ok(ValueType::F64),
            _ => Err(Error::UnknownValueType(val)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct FunctionType {
    pub form: u8,
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

impl Default for FunctionType {
    fn default() -> FunctionType {
        FunctionType {
            form: 0,
            params: Vec::new(),
            results: Vec::new()
        }
    }
}

impl Deserialize for FunctionType {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<FunctionType, Error> {
        let form: u8 = VarInt7::deserialize(reader)?.into();

        if form != 0x60 {
            return Err(Error::UnknownFunctionForm(form));
        }

        let params: Vec<ValueType> = CountedList::deserialize(reader)?.into_inner();
        let results: Vec<ValueType> = CountedList::deserialize(reader)?.into_inner();

        if results.len() > 1 {
            return Err(
                Error::Other("Enable the multi_value feature to deserialize more than one function result")
            );
        }

        Ok(
            FunctionType {
                form: form,
                params: params,
                results: results,
            }
        )
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TableElementType {
	/// A reference to a function with any signature.
	AnyFunc,
}

impl Deserialize for TableElementType {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<TableElementType, Error> {
        let val: i8 = VarInt7::deserialize(reader)?.into();

        match val {
            -0x10 => Ok(TableElementType::AnyFunc),
            _ => Err(Error::UnknownTableElementType(val)),
        }
    }   
}