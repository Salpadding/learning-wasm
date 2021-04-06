use super::{Deserialize, Error};
use std::io;
use super::primitives::{VarUint32, CountedList, VarUint7};
use super::types::FunctionType;
use super::import_entry::ImportEntry;
use super::func::Func;

#[cfg(feature = "reduced-stack-buffer")]
const ENTRIES_BUFFER_LENGTH: usize = 256;

#[cfg(not(feature = "reduced-stack-buffer"))]
const ENTRIES_BUFFER_LENGTH: usize = 16384;

#[derive(Debug, Clone, PartialEq)]
pub enum Section {
    Unparsed {
        id: u8,
        payload: Vec<u8>
    },
    Custom(CustomSection),
    Type(TypeSection),
    Import(ImportSection),
    Function(FunctionSection),
}

impl Deserialize for Section {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Section, Error> {
        let id = match VarUint7::deserialize(reader) {
            Ok(v) => v.0,
            Err(_) => {
                return Err(Error::UnexpectedEof);
            }
        };

        let i: u8 = id.into();

        let s = match i {
            0 => Section::Custom(CustomSection::deserialize(reader)?),
            1 => Section::Type(TypeSection::deserialize(reader)?),
            _ => Err(Error::InvalidSectionId(i)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomSection {
    pub name: String,
    pub payload: Vec<u8>
}

impl Deserialize for CustomSection {
	type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<CustomSection, Error> {
        let section_length: u32 = VarUint32::deserialize(reader)?.into();
        let buf: Vec<u8> = buffered_read!(ENTRIES_BUFFER_LENGTH, section_length as usize, reader);
        // 将 buf 转为 Cursor
        let mut cursor = io::Cursor::new(&buf[..]);
        let name = String::deserialize(&mut cursor)?;
        let payload = &buf[(cursor.position() as usize)..];
        Ok(
            CustomSection {
                name: name,
                payload: payload.to_vec()
            }
        )
    }
}

// TypeSection
#[derive(Debug, Clone, PartialEq)]
pub struct TypeSection(pub Vec<FunctionType>);

impl Deserialize for TypeSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<TypeSection, Error> {
        let types: Vec<FunctionType> = CountedList::deserialize(reader)?.into_inner();
        Ok(TypeSection(types))
    }    
}

pub type ImportSection = Vec<ImportEntry>;

impl Deserialize for ImportSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<ImportSection, Error> {
        let imports: Vec<ImportEntry> = CountedList::deserialize(reader)?.into_inner();
        Ok(imports)
    }  
}

pub type FunctionSection = Vec<Func>;

impl Deserialize for FunctionSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<FunctionSection, Error> {
        let funcs: Vec<Func> = CountedList::deserialize(reader)?.into_inner();
        Ok(funcs)
    }      
}

#[cfg(test)]
mod test{

    #[test]
    fn test() {

    }
}