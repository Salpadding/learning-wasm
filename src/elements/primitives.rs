use super::{Deserialize, Error};
use std::io;


#[cfg(feature = "reduced-stack-buffer")]
const PRIMITIVES_BUFFER_LENGTH: usize = 256;

#[cfg(not(feature = "reduced-stack-buffer"))]
const PRIMITIVES_BUFFER_LENGTH: usize = 1024;


/// 32-bit unsigned integer, encoded in little endian.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Uint32(pub u32);

impl From<u32> for Uint32 {
    fn from(n: u32) -> Uint32 {
        Uint32(n)
    }
}

impl From<Uint32> for u32 {
    fn from(n: Uint32) -> u32 {
        n.0
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

/// Unsigned variable-length integer, limited to 32 bits,
/// represented by at most 5 bytes that may contain padding 0x80 bytes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarUint32(pub u32);

impl From<u32> for VarUint32 {
    fn from(x: u32) -> VarUint32 {
        VarUint32(x)
    }
}

impl From<VarUint32> for u32 {
    fn from(x: VarUint32) -> u32 {
        x.0
    }
}

impl Deserialize for VarUint32 {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<VarUint32, Error>{
		let mut res = 0;
		let mut shift = 0;
		let mut u8buf = [0u8; 1];
		loop {
			if shift > 31 { return Err(Error::InvalidVarUint32); }

			reader.read(&mut u8buf)?;
			let b = u8buf[0] as u32;
			res |= (b & 0x7f).checked_shl(shift).ok_or(Error::InvalidVarUint32)?;
			shift += 7;
			if (b >> 7) == 0 {
				if shift >= 32 && (b as u8).leading_zeros() < 4 {
					return Err(Error::InvalidVarInt32);
				}
				break;
			}
		}
		Ok(VarUint32(res))        
    }
}

impl Deserialize for String {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<String, Error>{
        let len: u32 = VarUint32::deserialize(reader)?.into();

        if len == 0 {
            return Ok(String::new());
        }

        let v = buffered_read!(PRIMITIVES_BUFFER_LENGTH, len as usize, reader);
        // map_err 把 Result<String, FromUtf8Error> 转成了 Result<String, Error>
        String::from_utf8(v).map_err(|_| Error::NonUtf8String)
    }
}

/// 7-bit signed integer, encoded in LEB128 (always 1 byte length)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarInt7(pub i8);

impl From<VarInt7> for i8 {
    fn from(other: VarInt7) -> i8 {
        other.0
    }
}

impl From<VarInt7> for u8 {
    fn from(other: VarInt7) -> u8 {
        other.0 as u8
    }
}

impl From<i8> for VarInt7 {
    fn from(other: i8) -> VarInt7 {
        VarInt7(other)
    }
}


impl Deserialize for VarInt7 {
	type Error = Error;

	/// Deserialize type from serial i/o
	fn deserialize<R: io::Read>(reader: &mut R) -> Result<VarInt7, Error> {
		let mut u8buf = [0u8; 1];
		reader.read(&mut u8buf)?;

		// check if number is not continued!
		if u8buf[0] & 0b1000_0000 != 0 {
			return Err(Error::InvalidVarInt7(u8buf[0]));
		}

		// expand sign
		if u8buf[0] & 0b0100_0000 == 0b0100_0000 { u8buf[0] |= 0b1000_0000 }

		Ok(VarInt7(u8buf[0] as i8))        
    }
}

#[derive(Debug, Clone)]
pub struct CountedList<T: Deserialize>(pub Vec<T>);

impl<T: Deserialize> CountedList<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
} 

impl<T: Deserialize> Deserialize for CountedList<T> where T::Error : From<Error> {
    type Error = T::Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let len: u32 = VarUint32::deserialize(reader)?.into();
        let mut res: Vec<T> = Vec::new();
        for _ in 0..len {
            let t = T::deserialize(reader)?;
            res.push(
                t
            );
        }
        Ok(CountedList(res))
    }     
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Uint8(pub u8);

impl From<Uint8> for u8 {
    fn from(other: Uint8) -> u8 {
        other.0
    }
}

impl Deserialize for Uint8 {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Uint8, Error> {
        let mut buf = [0u8; 1];
        reader.read(&mut buf)?;
        Ok(Uint8(buf[0]))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarUint1(pub bool);

impl From<VarUint1> for bool {
    fn from(other: VarUint1) -> bool {
        other.0
    }
}

impl Deserialize for VarUint1 {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<VarUint1, Error> {
        let mut buf = [0u8; 1];
        reader.read(&mut buf)?;
        match buf[0] {
            0 => Ok(VarUint1(false)),
            1 => Ok(VarUint1(true)),
            _ => Err(
                Error::InvalidVarUint1(buf[0])
            )
        }
    }
}

/// 7-bit unsigned integer, encoded in LEB128 (always 1 byte length).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarUint7(pub u8);

impl From<VarUint7> for u8 {
	fn from(v: VarUint7) -> u8 {
		v.0
	}
}

impl From<u8> for VarUint7 {
	fn from(v: u8) -> Self {
		VarUint7(v)
	}
}

impl Deserialize for VarUint7 {
	type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut u8buf = [0u8; 1];
		reader.read(&mut u8buf)?;
		Ok(VarUint7(u8buf[0]))
	}
}

/// 64-bit signed integer, encoded in LEB128 (can be 1-9 bytes length).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarInt64(i64);

impl From<VarInt64> for i64 {
	fn from(v: VarInt64) -> i64 {
		v.0
	}
}

impl From<i64> for VarInt64 {
	fn from(v: i64) -> VarInt64 {
		VarInt64(v)
	}
}

impl Deserialize for VarInt64 {
	type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut res = 0i64;
		let mut shift = 0;
		let mut u8buf = [0u8; 1];

		loop {
			if shift > 63 { return Err(Error::InvalidVarInt64); }
			reader.read(&mut u8buf)?;
			let b = u8buf[0];

			res |= ((b & 0x7f) as i64).checked_shl(shift).ok_or(Error::InvalidVarInt64)?;

			shift += 7;
			if (b >> 7) == 0 {
				if shift < 64 && b & 0b0100_0000 == 0b0100_0000 {
					res |= (1i64 << shift).wrapping_neg();
				} else if shift >= 64 && b & 0b0100_0000 == 0b0100_0000 {
					if (b | 0b1000_0000) as i8 != -1 {
						return Err(Error::InvalidVarInt64);
					}
				} else if shift >= 64 && b != 0 {
					return Err(Error::InvalidVarInt64);
				}
				break;
			}
		}
		Ok(VarInt64(res))
	}
}

/// 32-bit signed integer, encoded in LEB128 (can be 1-5 bytes length).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarInt32(i32);

impl From<VarInt32> for i32 {
	fn from(v: VarInt32) -> i32 {
		v.0
	}
}

impl From<i32> for VarInt32 {
	fn from(v: i32) -> VarInt32 {
		VarInt32(v)
	}
}

impl Deserialize for VarInt32 {
	type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut res = 0;
		let mut shift = 0;
		let mut u8buf = [0u8; 1];
		loop {
			if shift > 31 { return Err(Error::InvalidVarInt32); }
			reader.read(&mut u8buf)?;
			let b = u8buf[0];

			res |= ((b & 0x7f) as i32).checked_shl(shift).ok_or(Error::InvalidVarInt32)?;

			shift += 7;
			if (b >> 7) == 0 {
				if shift < 32 && b & 0b0100_0000 == 0b0100_0000 {
					res |= (1i32 << shift).wrapping_neg();
				} else if shift >= 32 && b & 0b0100_0000 == 0b0100_0000 {
					if (!(b | 0b1000_0000)).leading_zeros() < 5 {
						return Err(Error::InvalidVarInt32);
					}
				} else if shift >= 32 && b & 0b0100_0000 == 0 {
					if b.leading_zeros() < 5 {
						return Err(Error::InvalidVarInt32);
					}
				}
				break;
			}
		}
		Ok(VarInt32(res))
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


/// 64-bit unsigned integer, encoded in little endian.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Uint64(u64);

impl Deserialize for Uint64 {
	type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut buf = [0u8; 8];
		reader.read(&mut buf)?;
		// todo check range
		Ok(u64::from_le_bytes(buf).into())
	}
}

impl From<u64> for Uint64 {
	fn from(u: u64) -> Self { Uint64(u) }
}

impl From<Uint64> for u64 {
	fn from(var: Uint64) -> u64 {
		var.0
	}
}