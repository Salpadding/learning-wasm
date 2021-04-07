use super::{Deserialize, Error};
use std::io;
use super::primitives::{VarUint32, CountedList, VarUint7};
use super::types::FunctionType;
use super::import_entry::{ImportEntry, TableType, ResizableLimits};
use super::func::Func;
use super::global_entry::GlobalEntry;
use super::print_stream;
use super::segment::{ElementSegment};
use crate::elements::segment::DataSegment;
use super::func::FuncBody;
use super::export_entry::ExportEntry;

#[cfg(feature = "reduced-stack-buffer")]
const ENTRIES_BUFFER_LENGTH: usize = 256;

#[cfg(not(feature = "reduced-stack-buffer"))]
const ENTRIES_BUFFER_LENGTH: usize = 16384;

pub(crate) struct SectionReader {
    cursor: io::Cursor<Vec<u8>>,
    declared_length: usize
}

impl SectionReader {
    pub fn new<R: io::Read>(r: &mut R) -> Result<Self, Error> {
        let len: u32 = VarUint32::deserialize(r)?.into();

        let declared_length = len as usize;
        let v = buffered_read!(ENTRIES_BUFFER_LENGTH, declared_length, r);

        Ok(
            SectionReader {
                cursor: io::Cursor::new(v),
                declared_length: declared_length,
            }
        )
    }

    pub fn close(self) -> Result<(), io::Error> {
        let cursor = self.cursor;
        let buf_length = self.declared_length;

        if cursor.position() as usize != buf_length {
            return Err(io::ErrorKind::InvalidData.into());
        }

        Ok(())
    }

    pub fn payload(self) -> Vec<u8> {
        self.cursor.into_inner()
    }
}

impl io::Read for SectionReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cursor.read(buf)
    }
}

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
    Table(TableSection),
    Memory(MemorySection),
    Global(GlobalSection),
    Export(ExportSection),
    Start(u32),
    Element(ElementSection),
    DataCount(u32),
    Code(CodeSection),
    Data(DataSection),
}

impl Deserialize for Section {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        let id: u8 = match VarUint7::deserialize(reader) {
            Ok(v) => v.into(),
            Err(_) => return Err(Error::UnexpectedEof)
        };

        let s: Section = match id {
            0 => Section::Custom(
                CustomSection::deserialize(reader)?
            ),
            1 => Section::Type(
                TypeSection::deserialize(reader)?
            ),
            2 => Section::Import(
                ImportSection::deserialize(reader)?
            ),
            3 => Section::Function(
                FunctionSection::deserialize(reader)?
            ),
            4 => Section::Table(
                TableSection::deserialize(reader)?
            ),
            5 => Section::Memory(
                MemorySection::deserialize(reader)?
            ),
            6 => Section::Global(
                GlobalSection::deserialize(reader)?
            ),
            7 => {
                Section::Export(ExportSection::deserialize(reader)?)
            },
            8 => {
                let mut section_reader = SectionReader::new(reader)?;
                let start_idx = VarUint32::deserialize(&mut section_reader)?;
                section_reader.close()?;
                Section::Start(start_idx.into())
            },
            9 => Section::Element(
              ElementSection::deserialize(reader)?
            ),
            10 => Section::Code(
                CodeSection::deserialize(reader)?
            ),
            11 => {
                Section::Data(DataSection::deserialize(reader)?)
            },
            12 => {
                let mut section_reader = SectionReader::new(reader)?;
                let count = VarUint32::deserialize(&mut section_reader)?;
                section_reader.close()?;
                Section::DataCount(count.into())
            }
            _ => {
                let r = SectionReader::new(reader)?;
                let payload = r.payload();
                Section::Unparsed {
                    id, payload
                }
            }
        };

        Ok(s)
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
        let mut rd = SectionReader::new(reader)?;
        let types: Vec<FunctionType> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(TypeSection(types))
    }    
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSection(pub Vec<ImportEntry>);

impl Deserialize for ImportSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<ImportSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let imports: Vec<ImportEntry> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(ImportSection(imports))
    }  
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSection(pub Vec<Func>);

impl Deserialize for FunctionSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<FunctionSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let funcs: Vec<Func> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(FunctionSection(funcs))
    }      
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableSection(pub Vec<TableType>);

impl Deserialize for TableSection {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<TableSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let types: Vec<TableType> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(
            TableSection(
                types
            )
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct MemorySection(pub Vec<ResizableLimits>);

impl Deserialize for MemorySection {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<MemorySection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let v: Vec<ResizableLimits> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(
            MemorySection(v)
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GlobalSection(pub Vec<GlobalEntry>);

impl Deserialize for GlobalSection {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<GlobalSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let v: Vec<GlobalEntry> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(
            GlobalSection(v)
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElementSection(Vec<ElementSegment>);

impl Deserialize for ElementSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<ElementSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let v: Vec<ElementSegment> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(
            ElementSection(v)
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DataSection(Vec<DataSegment>);

impl Deserialize for DataSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<DataSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let v: Vec<DataSegment> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(
            DataSection(v)
        )
    }
}

/// Section with function bodies of the module.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CodeSection(Vec<FuncBody>);

impl Deserialize for CodeSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<CodeSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let v: Vec<FuncBody> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(
            CodeSection(v)
        )
    }
}

/// List of exports definition.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExportSection(Vec<ExportEntry>);

impl Deserialize for ExportSection {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<ExportSection, Error> {
        let mut rd = SectionReader::new(reader)?;
        let v: Vec<ExportEntry> = CountedList::deserialize(&mut rd)?.into_inner();
        rd.close()?;
        Ok(
            ExportSection(v)
        )
    }
}

#[cfg(test)]
mod test{

    #[test]
    fn test() {

    }
}