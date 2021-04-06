const WASM_MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
use super::{Deserialize, Error};
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    magic: u32,
    version: u32,
    sections: Vec<u32>
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
        reader.read(&mut buf)?;

        if buf != WASM_MAGIC_NUMBER {
            return Err(
                Error::InvalidMagic
            );
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

