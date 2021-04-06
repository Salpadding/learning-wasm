use super::{Deserialize, Error};
use std::io;

/// 32-bit unsigned integer, encoded in little endian.
#[derive(Debug)]
pub struct Uint32(u32);

impl From<u32> for Uint32 {
    fn from(n: u32) -> Uint32 {
        Uint32(n)
    }
}

impl Deserialize for Uint32 {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Uint32, Error> {
        let mut buf = [0u8; 4];
        reader.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf).into())
    }
}

#[cfg(test)]
mod test{
    use crate::tests::ByteStream;
    use super::Uint32;
    use super::{Deserialize, Error};

    #[test]
    fn test() {
        let buf = [1u8, 0u8, 0u8, 0u8];
        let mut stream = ByteStream(&buf);

        let u = Uint32::deserialize(&mut stream);
        println!("{:?}", u);
    }
}
