use super::types::{TableElementType, ValueType};
use super::{Deserialize, Error};
use super::primitives::{Uint8, VarUint32, VarUint1, VarInt7};
use std::io;

const FLAG_HAS_MAX: u8 = 0x01;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResizableLimits {
	pub initial: u32,
	pub maximum: Option<u32>
}

impl Deserialize for ResizableLimits {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        let flags: u8 = Uint8::deserialize(reader)?.into();
        match flags {
            0x00 | 0x01 => {},
            _ => return Err(Error::InvalidLimitsFlags(flags)),
        }

        let initial: u32 = VarUint32::deserialize(reader)?.into();
        let maximum = if flags & FLAG_HAS_MAX != 0 {
            Some(VarUint32::deserialize(reader)?.into())
        } else {
            None
        };

        Ok(
            ResizableLimits {
                initial, maximum
            }
        )
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TableType {
    pub elem_type: TableElementType,
    pub limits: ResizableLimits
}

impl Deserialize for TableType {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<TableType, Error> {
        let elem_type: TableElementType = TableElementType::deserialize(reader)?;
        let limits: ResizableLimits = ResizableLimits::deserialize(reader)?;

        Ok(
            TableType {
                elem_type, limits
            }
        )
    }

}   

#[derive(Debug, Clone, PartialEq)]
pub enum External {
    Function(u32),
    Table(TableType),
    Memory(ResizableLimits),
    Global(GlobalType)
}

impl Deserialize for External {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        let kind: u8 = VarInt7::deserialize(reader)?.into();

        match kind {
            0x00 => Ok(External::Function(VarUint32::deserialize(reader)?.into())),
            0x01 => Ok(External::Table(TableType::deserialize(reader)?)),
            0x02 => Ok(External::Memory(ResizableLimits::deserialize(reader)?)),
            0x03 => Ok(External::Global(GlobalType::deserialize(reader)?)),
            _ => Err(Error::UnknownExternalKind(kind)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportEntry {
    pub module_str: String,
    pub field_str: String,
    pub external: External
}

impl Deserialize for ImportEntry {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<ImportEntry, Error> {
        let module_str = String::deserialize(reader)?;
        let field_str = String::deserialize(reader)?;
        let external = External::deserialize(reader)?;

        Ok(
            ImportEntry { module_str, field_str, external}
        )
    }
}

/// Global definition struct
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GlobalType {
	pub content_type: ValueType,
	pub is_mutable: bool,
}


impl Deserialize for GlobalType {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<GlobalType, Error> {
        let content_type = ValueType::deserialize(reader)?;
        let is_mutable: bool = VarUint1::deserialize(reader)?.into();
        Ok (
            GlobalType {
                content_type, is_mutable
            }
        )
    }
}


