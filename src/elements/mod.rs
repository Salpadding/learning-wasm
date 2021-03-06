use std::io;
use core::fmt;

// 先从 Reader 读到 buffer，再从 buffer 读到 vec
// 这里 buffer_size 必须是字面量或者常量
// 这里要把宏定义放在 pub mod module 前面
macro_rules! buffered_read {
    ($buffer_size: expr, $length: expr, $reader: expr) => {
        {
            let mut v: Vec<u8> = Vec::new();
            let mut total_read: usize = 0;
            let mut buf = [0u8; $buffer_size];
            while total_read < $length {
                let next_to_read = if $length - total_read > $buffer_size  { $buffer_size } else { $length - total_read };
                $reader.read(&mut buf[0..next_to_read])?;
                v.extend_from_slice(&buf[0..next_to_read]);
                total_read += next_to_read;
            }
            v
        }
    }
}

pub mod module;
pub mod primitives;
pub mod sections;
pub mod types;
pub mod import_entry;
pub mod func;
pub mod ops;
pub mod global_entry;
pub mod segment;
pub mod export_entry;

pub fn print_stream<R: io::Read>(r: &mut R, max_len: usize) -> io::Result<()> {
    const BUF_SIZE: usize = 256;
    let mut buf = [0u8; BUF_SIZE];
    let mut already_read: usize = 0;
    while already_read < max_len {
        let max = if max_len - already_read > BUF_SIZE  { BUF_SIZE } else { max_len - already_read };
        let slice = &mut buf[0..max_len];
        r.read(slice)?;

        for i in slice.iter() {
            print!("{:02x}", i);
        }
        
        already_read += max;
    }

    Ok(())
}

/// Deserialization from serial i/o.
pub trait Deserialize : Sized {
	/// Serialization error produced by deserialization routine.
	type Error: From<io::Error>;
	/// Deserialize type from serial i/o
	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error>;
}

/// Deserialization/serialization error
#[derive(Debug, Clone)]
pub enum Error {
	/// Unexpected end of input.
	UnexpectedEof,
	/// Invalid magic.
	InvalidMagic,
	/// Unsupported version.
	UnsupportedVersion(u32),
	/// Inconsistence between declared and actual length.
	InconsistentLength {
		/// Expected length of the definition.
		expected: usize,
		/// Actual length of the definition.
		actual: usize
	},
	/// Other static error.
	Other(&'static str),
	/// Other allocated error.
	HeapOther(String),
	/// Invalid/unknown value type declaration.
	UnknownValueType(i8),
	/// Invalid/unknown table element type declaration.
	UnknownTableElementType(i8),
	/// Non-utf8 string.
	NonUtf8String,
	/// Unknown external kind code.
	UnknownExternalKind(u8),
	/// Unknown internal kind code.
	UnknownInternalKind(u8),
	/// Unknown opcode encountered.
	UnknownOpcode(u8),
	#[cfg(feature="simd")]
	/// Unknown SIMD opcode encountered.
	UnknownSimdOpcode(u32),
	/// Invalid VarUint1 value.
	InvalidVarUint1(u8),
	/// Invalid VarInt32 value.
	InvalidVarInt32,
	/// Invalid VarInt64 value.
	InvalidVarInt64,
	/// Invalid VarUint32 value.
	InvalidVarUint32,
	/// Invalid VarUint64 value.
	InvalidVarUint64,
	/// Inconsistent metadata.
	InconsistentMetadata,
	/// Invalid section id.
	InvalidSectionId(u8),
	/// Sections are out of order.
	SectionsOutOfOrder,
	/// Duplicated sections.
	DuplicatedSections(u8),
	/// Invalid memory reference (should be 0).
	InvalidMemoryReference(u8),
	/// Invalid table reference (should be 0).
	InvalidTableReference(u8),
	/// Invalid value used for flags in limits type.
	InvalidLimitsFlags(u8),
	/// Unknown function form (should be 0x60).
	UnknownFunctionForm(u8),
	/// Invalid varint7 (should be in -64..63 range).
	InvalidVarInt7(u8),
	/// Number of function body entries and signatures does not match.
	InconsistentCode,
	/// Only flags 0, 1, and 2 are accepted on segments.
	InvalidSegmentFlags(u32),
	/// Sum of counts of locals is greater than 2^32.
	TooManyLocals,
	/// Duplicated name subsections.
	DuplicatedNameSubsections(u8),
	/// Unknown name subsection type.
	UnknownNameSubsectionType(u8),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::UnexpectedEof => write!(f, "Unexpected end of input"),
			Error::InvalidMagic => write!(f, "Invalid magic number at start of file"),
			Error::UnsupportedVersion(v) => write!(f, "Unsupported wasm version {}", v),
			Error::InconsistentLength { expected, actual } => {
				write!(f, "Expected length {}, found {}", expected, actual)
			}
			Error::Other(msg) => write!(f, "{}", msg),
			Error::HeapOther(ref msg) => write!(f, "{}", msg),
			Error::UnknownValueType(ty) => write!(f, "Invalid or unknown value type {}", ty),
			Error::UnknownTableElementType(ty) => write!(f, "Unknown table element type {}", ty),
			Error::NonUtf8String => write!(f, "Non-UTF-8 string"),
			Error::UnknownExternalKind(kind) => write!(f, "Unknown external kind {}", kind),
			Error::UnknownInternalKind(kind) => write!(f, "Unknown internal kind {}", kind),
			Error::UnknownOpcode(opcode) => write!(f, "Unknown opcode {}", opcode),
			#[cfg(feature="simd")]
			Error::UnknownSimdOpcode(opcode) => write!(f, "Unknown SIMD opcode {}", opcode),
			Error::InvalidVarUint1(val) => write!(f, "Not an unsigned 1-bit integer: {}", val),
			Error::InvalidVarInt7(val) => write!(f, "Not a signed 7-bit integer: {}", val),
			Error::InvalidVarInt32 => write!(f, "Not a signed 32-bit integer"),
			Error::InvalidVarUint32 => write!(f, "Not an unsigned 32-bit integer"),
			Error::InvalidVarInt64 => write!(f, "Not a signed 64-bit integer"),
			Error::InvalidVarUint64 => write!(f, "Not an unsigned 64-bit integer"),
			Error::InconsistentMetadata =>  write!(f, "Inconsistent metadata"),
			Error::InvalidSectionId(ref id) =>  write!(f, "Invalid section id: {}", id),
			Error::SectionsOutOfOrder =>  write!(f, "Sections out of order"),
			Error::DuplicatedSections(ref id) =>  write!(f, "Duplicated sections ({})", id),
			Error::InvalidMemoryReference(ref mem_ref) =>  write!(f, "Invalid memory reference ({})", mem_ref),
			Error::InvalidTableReference(ref table_ref) =>  write!(f, "Invalid table reference ({})", table_ref),
			Error::InvalidLimitsFlags(ref flags) =>  write!(f, "Invalid limits flags ({})", flags),
			Error::UnknownFunctionForm(ref form) =>  write!(f, "Unknown function form ({})", form),
			Error::InconsistentCode =>  write!(f, "Number of function body entries and signatures does not match"),
			Error::InvalidSegmentFlags(n) =>  write!(f, "Invalid segment flags: {}", n),
			Error::TooManyLocals => write!(f, "Too many locals"),
			Error::DuplicatedNameSubsections(n) =>  write!(f, "Duplicated name subsections: {}", n),
			Error::UnknownNameSubsectionType(n) => write!(f, "Unknown subsection type: {}", n),
		}
	}
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::UnexpectedEof => "Unexpected end of input",
			Error::InvalidMagic => "Invalid magic number at start of file",
			Error::UnsupportedVersion(_) => "Unsupported wasm version",
			Error::InconsistentLength { .. } => "Inconsistent length",
			Error::Other(msg) => msg,
			Error::HeapOther(ref msg) => &msg[..],
			Error::UnknownValueType(_) => "Invalid or unknown value type",
			Error::UnknownTableElementType(_) => "Unknown table element type",
			Error::NonUtf8String => "Non-UTF-8 string",
			Error::UnknownExternalKind(_) => "Unknown external kind",
			Error::UnknownInternalKind(_) => "Unknown internal kind",
			Error::UnknownOpcode(_) => "Unknown opcode",
			#[cfg(feature="simd")]
			Error::UnknownSimdOpcode(_) => "Unknown SIMD opcode",
			Error::InvalidVarUint1(_) => "Not an unsigned 1-bit integer",
			Error::InvalidVarInt32 => "Not a signed 32-bit integer",
			Error::InvalidVarInt7(_) => "Not a signed 7-bit integer",
			Error::InvalidVarUint32 => "Not an unsigned 32-bit integer",
			Error::InvalidVarInt64 => "Not a signed 64-bit integer",
			Error::InvalidVarUint64 => "Not an unsigned 64-bit integer",
			Error::InconsistentMetadata => "Inconsistent metadata",
			Error::InvalidSectionId(_) =>  "Invalid section id",
			Error::SectionsOutOfOrder =>  "Sections out of order",
			Error::DuplicatedSections(_) =>  "Duplicated section",
			Error::InvalidMemoryReference(_) =>  "Invalid memory reference",
			Error::InvalidTableReference(_) =>  "Invalid table reference",
			Error::InvalidLimitsFlags(_) => "Invalid limits flags",
			Error::UnknownFunctionForm(_) =>  "Unknown function form",
			Error::InconsistentCode =>  "Number of function body entries and signatures does not match",
			Error::InvalidSegmentFlags(_) =>  "Invalid segment flags",
			Error::TooManyLocals => "Too many locals",
			Error::DuplicatedNameSubsections(_) =>  "Duplicated name subsections",
			Error::UnknownNameSubsectionType(_) => "Unknown name subsections type",
		}
	}
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Error {
        Error::HeapOther(format!("I/O Error: {:?}", other))
    }
}