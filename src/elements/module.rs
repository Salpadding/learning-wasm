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


        Ok(Module::default())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    pub fn test() {
        let _m = Module::default();
    }
}

