const WASM_MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
use super::{Deserialize, Error};
use super::primitives::Uint32;
use super::sections::Section;
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub magic: u32,
    pub version: u32,
    pub sections: Vec<Section>,
}

impl Default for Module {
    fn default() -> Module {
        Module {
            magic: u32::from_le_bytes(WASM_MAGIC_NUMBER),
            version:1,
            sections: Vec::new()
        }
    }
}

impl Deserialize for Module {
    	/// Serialization error produced by deserialization routine.
	type Error = Error;
	/// Deserialize type from serial i/o
	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Module, Error> {
        let mut buf = [0u8; 4];

        // 因为 Error 实现了 From<std::io::Error>，所以可以直接使用 ? 语法糖
        reader.read(&mut buf)?;

        if buf != WASM_MAGIC_NUMBER {
            return Err(
                Error::InvalidMagic
            );
        }

        let version: u32 = Uint32::deserialize(reader)?.into();
        if version != 1 {
            return Err(Error::UnsupportedVersion(version));
        }

        let mut sections: Vec<Section> = Vec::new();

        loop {
            match Section::deserialize(reader) {
                Err(Error::UnexpectedEof) => break,
                Err(e) => {
                    return Err(e);
                },
                Ok(s) => sections.push(s),
            }
        }

        let mut m = Module::default();
        m.sections = sections;
        Ok(m)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::fs;
    use super::super::print_stream;
    use crate::io::BufReader;

    #[test]
    pub fn test() {
        let _m = Module::default();
    }

    #[test]
    pub fn test_parse() {
        let mut f = fs::File::open("/Users/sal/Documents/Github/maze-protocol/layer2/main.wasm").unwrap();
        let mut buf = BufReader::new(&mut f);
        let m = Module::deserialize(&mut buf).unwrap();
    }
}

