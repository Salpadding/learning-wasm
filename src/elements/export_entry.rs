use super::{Deserialize, Error};
use super::primitives::{VarUint7, VarUint32};
use std::io;

/// Export entry.
#[derive(Debug, Clone, PartialEq)]
pub struct ExportEntry {
    pub field_str: String,
    pub internal: Internal,
}


impl Deserialize for ExportEntry {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let field_str = String::deserialize(reader)?;
        let internal = Internal::deserialize(reader)?;

        Ok(ExportEntry {
            field_str: field_str,
            internal: internal,
        })
    }
}

/// Internal reference of the exported entry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Internal {
    /// Function reference.
    Function(u32),
    /// Table reference.
    Table(u32),
    /// Memory reference.
    Memory(u32),
    /// Global reference.
    Global(u32),
}

impl Deserialize for Internal {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let kind = VarUint7::deserialize(reader)?;
        match kind.into() {
            0x00 => Ok(Internal::Function(VarUint32::deserialize(reader)?.into())),
            0x01 => Ok(Internal::Table(VarUint32::deserialize(reader)?.into())),
            0x02 => Ok(Internal::Memory(VarUint32::deserialize(reader)?.into())),
            0x03 => Ok(Internal::Global(VarUint32::deserialize(reader)?.into())),
            _ => Err(Error::UnknownInternalKind(kind.into())),
        }
    }
}