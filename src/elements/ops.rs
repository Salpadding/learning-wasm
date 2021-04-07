use super::types::BlockType;
use super::{Deserialize, Error};
use super::primitives::{VarUint32, CountedList, Uint8, VarInt32, VarInt64, Uint32, Uint64};
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct BrTableData {
	pub table: Box<[u32]>,
	pub default: u32,
}

/// List of instructions (usually inside a block section).
#[derive(Debug, Clone, PartialEq)]
pub struct Instructions(Vec<Instruction>);

impl Deserialize for Instructions {
	type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		let mut instructions = Vec::new();
		let mut block_count = 1usize;

		loop {
			let instruction = Instruction::deserialize(reader)?;
			if instruction.is_terminal() {
				block_count -= 1;
			} else if instruction.is_block() {
				block_count = block_count.checked_add(1).ok_or(Error::Other("too many instructions"))?;
			}

			instructions.push(instruction);
			if block_count == 0 {
				break;
			}
		}

		Ok(Instructions(instructions))
	}
}

/// Initialization expression.
#[derive(Debug, Clone, PartialEq)]
pub struct InitExpr(pub Vec<Instruction>);


impl Deserialize for InitExpr {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
        let mut ins: Vec<Instruction> = Vec::new();

        loop {
            let i = Instruction::deserialize(reader)?;
            let is_terminal = i.is_terminal();

            if is_terminal {
                break;
            }
        }

        Ok(InitExpr(ins))
    }
}

impl Instruction {
	/// Is this instruction starts the new block (which should end with terminal instruction).
	pub fn is_block(&self) -> bool {
		match self {
			&Instruction::Block(_) | &Instruction::Loop(_) | &Instruction::If(_) => true,
			_ => false,
		}
	}

	/// Is this instruction determines the termination of instruction sequence?
	///
	/// `true` for `Instruction::End`
	pub fn is_terminal(&self) -> bool {
		match self {
			&Instruction::End => true,
			_ => false,
		}
	}
}

/// Instruction.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Instruction {
	Unreachable,
	Nop,
	Block(BlockType),
	Loop(BlockType),
	If(BlockType),
	Else,
	End,
	Br(u32),
	BrIf(u32),
	BrTable(Box<BrTableData>),
	Return,

	Call(u32),
	CallIndirect(u32, u8),

	Drop,
	Select,

	GetLocal(u32),
	SetLocal(u32),
	TeeLocal(u32),
	GetGlobal(u32),
	SetGlobal(u32),

	// All store/load instructions operate with 'memory immediates'
	// which represented here as (flag, offset) tuple
	I32Load(u32, u32),
	I64Load(u32, u32),
	F32Load(u32, u32),
	F64Load(u32, u32),
	I32Load8S(u32, u32),
	I32Load8U(u32, u32),
	I32Load16S(u32, u32),
	I32Load16U(u32, u32),
	I64Load8S(u32, u32),
	I64Load8U(u32, u32),
	I64Load16S(u32, u32),
	I64Load16U(u32, u32),
	I64Load32S(u32, u32),
	I64Load32U(u32, u32),
	I32Store(u32, u32),
	I64Store(u32, u32),
	F32Store(u32, u32),
	F64Store(u32, u32),
	I32Store8(u32, u32),
	I32Store16(u32, u32),
	I64Store8(u32, u32),
	I64Store16(u32, u32),
	I64Store32(u32, u32),

	CurrentMemory(u8),
	GrowMemory(u8),

	I32Const(i32),
	I64Const(i64),
	F32Const(u32),
	F64Const(u64),

	I32Eqz,
	I32Eq,
	I32Ne,
	I32LtS,
	I32LtU,
	I32GtS,
	I32GtU,
	I32LeS,
	I32LeU,
	I32GeS,
	I32GeU,

	I64Eqz,
	I64Eq,
	I64Ne,
	I64LtS,
	I64LtU,
	I64GtS,
	I64GtU,
	I64LeS,
	I64LeU,
	I64GeS,
	I64GeU,

	F32Eq,
	F32Ne,
	F32Lt,
	F32Gt,
	F32Le,
	F32Ge,

	F64Eq,
	F64Ne,
	F64Lt,
	F64Gt,
	F64Le,
	F64Ge,

	I32Clz,
	I32Ctz,
	I32Popcnt,
	I32Add,
	I32Sub,
	I32Mul,
	I32DivS,
	I32DivU,
	I32RemS,
	I32RemU,
	I32And,
	I32Or,
	I32Xor,
	I32Shl,
	I32ShrS,
	I32ShrU,
	I32Rotl,
	I32Rotr,

	I64Clz,
	I64Ctz,
	I64Popcnt,
	I64Add,
	I64Sub,
	I64Mul,
	I64DivS,
	I64DivU,
	I64RemS,
	I64RemU,
	I64And,
	I64Or,
	I64Xor,
	I64Shl,
	I64ShrS,
	I64ShrU,
	I64Rotl,
	I64Rotr,
	F32Abs,
	F32Neg,
	F32Ceil,
	F32Floor,
	F32Trunc,
	F32Nearest,
	F32Sqrt,
	F32Add,
	F32Sub,
	F32Mul,
	F32Div,
	F32Min,
	F32Max,
	F32Copysign,
	F64Abs,
	F64Neg,
	F64Ceil,
	F64Floor,
	F64Trunc,
	F64Nearest,
	F64Sqrt,
	F64Add,
	F64Sub,
	F64Mul,
	F64Div,
	F64Min,
	F64Max,
	F64Copysign,

	I32WrapI64,
	I32TruncSF32,
	I32TruncUF32,
	I32TruncSF64,
	I32TruncUF64,
	I64ExtendSI32,
	I64ExtendUI32,
	I64TruncSF32,
	I64TruncUF32,
	I64TruncSF64,
	I64TruncUF64,
	F32ConvertSI32,
	F32ConvertUI32,
	F32ConvertSI64,
	F32ConvertUI64,
	F32DemoteF64,
	F64ConvertSI32,
	F64ConvertUI32,
	F64ConvertSI64,
	F64ConvertUI64,
	F64PromoteF32,

	I32ReinterpretF32,
	I64ReinterpretF64,
	F32ReinterpretI32,
	F64ReinterpretI64,
}

pub mod opcodes {
	pub const UNREACHABLE: u8 = 0x00;
	pub const NOP: u8 = 0x01;
	pub const BLOCK: u8 = 0x02;
	pub const LOOP: u8 = 0x03;
	pub const IF: u8 = 0x04;
	pub const ELSE: u8 = 0x05;
	pub const END: u8 = 0x0b;
	pub const BR: u8 = 0x0c;
	pub const BRIF: u8 = 0x0d;
	pub const BRTABLE: u8 = 0x0e;
	pub const RETURN: u8 = 0x0f;
	pub const CALL: u8 = 0x10;
	pub const CALLINDIRECT: u8 = 0x11;
	pub const DROP: u8 = 0x1a;
	pub const SELECT: u8 = 0x1b;
	pub const GETLOCAL: u8 = 0x20;
	pub const SETLOCAL: u8 = 0x21;
	pub const TEELOCAL: u8 = 0x22;
	pub const GETGLOBAL: u8 = 0x23;
	pub const SETGLOBAL: u8 = 0x24;
	pub const I32LOAD: u8 = 0x28;
	pub const I64LOAD: u8 = 0x29;
	pub const F32LOAD: u8 = 0x2a;
	pub const F64LOAD: u8 = 0x2b;
	pub const I32LOAD8S: u8 = 0x2c;
	pub const I32LOAD8U: u8 = 0x2d;
	pub const I32LOAD16S: u8 = 0x2e;
	pub const I32LOAD16U: u8 = 0x2f;
	pub const I64LOAD8S: u8 = 0x30;
	pub const I64LOAD8U: u8 = 0x31;
	pub const I64LOAD16S: u8 = 0x32;
	pub const I64LOAD16U: u8 = 0x33;
	pub const I64LOAD32S: u8 = 0x34;
	pub const I64LOAD32U: u8 = 0x35;
	pub const I32STORE: u8 = 0x36;
	pub const I64STORE: u8 = 0x37;
	pub const F32STORE: u8 = 0x38;
	pub const F64STORE: u8 = 0x39;
	pub const I32STORE8: u8 = 0x3a;
	pub const I32STORE16: u8 = 0x3b;
	pub const I64STORE8: u8 = 0x3c;
	pub const I64STORE16: u8 = 0x3d;
	pub const I64STORE32: u8 = 0x3e;
	pub const CURRENTMEMORY: u8 = 0x3f;
	pub const GROWMEMORY: u8 = 0x40;
	pub const I32CONST: u8 = 0x41;
	pub const I64CONST: u8 = 0x42;
	pub const F32CONST: u8 = 0x43;
	pub const F64CONST: u8 = 0x44;
	pub const I32EQZ: u8 = 0x45;
	pub const I32EQ: u8 = 0x46;
	pub const I32NE: u8 = 0x47;
	pub const I32LTS: u8 = 0x48;
	pub const I32LTU: u8 = 0x49;
	pub const I32GTS: u8 = 0x4a;
	pub const I32GTU: u8 = 0x4b;
	pub const I32LES: u8 = 0x4c;
	pub const I32LEU: u8 = 0x4d;
	pub const I32GES: u8 = 0x4e;
	pub const I32GEU: u8 = 0x4f;
	pub const I64EQZ: u8 = 0x50;
	pub const I64EQ: u8 = 0x51;
	pub const I64NE: u8 = 0x52;
	pub const I64LTS: u8 = 0x53;
	pub const I64LTU: u8 = 0x54;
	pub const I64GTS: u8 = 0x55;
	pub const I64GTU: u8 = 0x56;
	pub const I64LES: u8 = 0x57;
	pub const I64LEU: u8 = 0x58;
	pub const I64GES: u8 = 0x59;
	pub const I64GEU: u8 = 0x5a;

	pub const F32EQ: u8 = 0x5b;
	pub const F32NE: u8 = 0x5c;
	pub const F32LT: u8 = 0x5d;
	pub const F32GT: u8 = 0x5e;
	pub const F32LE: u8 = 0x5f;
	pub const F32GE: u8 = 0x60;

	pub const F64EQ: u8 = 0x61;
	pub const F64NE: u8 = 0x62;
	pub const F64LT: u8 = 0x63;
	pub const F64GT: u8 = 0x64;
	pub const F64LE: u8 = 0x65;
	pub const F64GE: u8 = 0x66;

	pub const I32CLZ: u8 = 0x67;
	pub const I32CTZ: u8 = 0x68;
	pub const I32POPCNT: u8 = 0x69;
	pub const I32ADD: u8 = 0x6a;
	pub const I32SUB: u8 = 0x6b;
	pub const I32MUL: u8 = 0x6c;
	pub const I32DIVS: u8 = 0x6d;
	pub const I32DIVU: u8 = 0x6e;
	pub const I32REMS: u8 = 0x6f;
	pub const I32REMU: u8 = 0x70;
	pub const I32AND: u8 = 0x71;
	pub const I32OR: u8 = 0x72;
	pub const I32XOR: u8 = 0x73;
	pub const I32SHL: u8 = 0x74;
	pub const I32SHRS: u8 = 0x75;
	pub const I32SHRU: u8 = 0x76;
	pub const I32ROTL: u8 = 0x77;
	pub const I32ROTR: u8 = 0x78;

	pub const I64CLZ: u8 = 0x79;
	pub const I64CTZ: u8 = 0x7a;
	pub const I64POPCNT: u8 = 0x7b;
	pub const I64ADD: u8 = 0x7c;
	pub const I64SUB: u8 = 0x7d;
	pub const I64MUL: u8 = 0x7e;
	pub const I64DIVS: u8 = 0x7f;
	pub const I64DIVU: u8 = 0x80;
	pub const I64REMS: u8 = 0x81;
	pub const I64REMU: u8 = 0x82;
	pub const I64AND: u8 = 0x83;
	pub const I64OR: u8 = 0x84;
	pub const I64XOR: u8 = 0x85;
	pub const I64SHL: u8 = 0x86;
	pub const I64SHRS: u8 = 0x87;
	pub const I64SHRU: u8 = 0x88;
	pub const I64ROTL: u8 = 0x89;
	pub const I64ROTR: u8 = 0x8a;
	pub const F32ABS: u8 = 0x8b;
	pub const F32NEG: u8 = 0x8c;
	pub const F32CEIL: u8 = 0x8d;
	pub const F32FLOOR: u8 = 0x8e;
	pub const F32TRUNC: u8 = 0x8f;
	pub const F32NEAREST: u8 = 0x90;
	pub const F32SQRT: u8 = 0x91;
	pub const F32ADD: u8 = 0x92;
	pub const F32SUB: u8 = 0x93;
	pub const F32MUL: u8 = 0x94;
	pub const F32DIV: u8 = 0x95;
	pub const F32MIN: u8 = 0x96;
	pub const F32MAX: u8 = 0x97;
	pub const F32COPYSIGN: u8 = 0x98;
	pub const F64ABS: u8 = 0x99;
	pub const F64NEG: u8 = 0x9a;
	pub const F64CEIL: u8 = 0x9b;
	pub const F64FLOOR: u8 = 0x9c;
	pub const F64TRUNC: u8 = 0x9d;
	pub const F64NEAREST: u8 = 0x9e;
	pub const F64SQRT: u8 = 0x9f;
	pub const F64ADD: u8 = 0xa0;
	pub const F64SUB: u8 = 0xa1;
	pub const F64MUL: u8 = 0xa2;
	pub const F64DIV: u8 = 0xa3;
	pub const F64MIN: u8 = 0xa4;
	pub const F64MAX: u8 = 0xa5;
	pub const F64COPYSIGN: u8 = 0xa6;

	pub const I32WRAPI64: u8 = 0xa7;
	pub const I32TRUNCSF32: u8 = 0xa8;
	pub const I32TRUNCUF32: u8 = 0xa9;
	pub const I32TRUNCSF64: u8 = 0xaa;
	pub const I32TRUNCUF64: u8 = 0xab;
	pub const I64EXTENDSI32: u8 = 0xac;
	pub const I64EXTENDUI32: u8 = 0xad;
	pub const I64TRUNCSF32: u8 = 0xae;
	pub const I64TRUNCUF32: u8 = 0xaf;
	pub const I64TRUNCSF64: u8 = 0xb0;
	pub const I64TRUNCUF64: u8 = 0xb1;
	pub const F32CONVERTSI32: u8 = 0xb2;
	pub const F32CONVERTUI32: u8 = 0xb3;
	pub const F32CONVERTSI64: u8 = 0xb4;
	pub const F32CONVERTUI64: u8 = 0xb5;
	pub const F32DEMOTEF64: u8 = 0xb6;
	pub const F64CONVERTSI32: u8 = 0xb7;
	pub const F64CONVERTUI32: u8 = 0xb8;
	pub const F64CONVERTSI64: u8 = 0xb9;
	pub const F64CONVERTUI64: u8 = 0xba;
	pub const F64PROMOTEF32: u8 = 0xbb;

	pub const I32REINTERPRETF32: u8 = 0xbc;
	pub const I64REINTERPRETF64: u8 = 0xbd;
	pub const F32REINTERPRETI32: u8 = 0xbe;
	pub const F64REINTERPRETI64: u8 = 0xbf;
}


impl Deserialize for Instruction {
	type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
		use self::Instruction::*;
		use self::opcodes::*;

		let val: u8 = Uint8::deserialize(reader)?.into();

		Ok(
			match val {
				UNREACHABLE => Unreachable,
				NOP => Nop,
				BLOCK => Block(BlockType::deserialize(reader)?),
				LOOP => Loop(BlockType::deserialize(reader)?),
				IF => If(BlockType::deserialize(reader)?),
				ELSE => Else,
				END => End,

				BR => Br(VarUint32::deserialize(reader)?.into()),
				BRIF => BrIf(VarUint32::deserialize(reader)?.into()),
				BRTABLE => {
					let t1: Vec<u32> = CountedList::<VarUint32>::deserialize(reader)?
						.into_inner()
						.into_iter()
						.map(Into::into)
						.collect();

					BrTable(Box::new(BrTableData {
						table: t1.into_boxed_slice(),
						default: VarUint32::deserialize(reader)?.into(),
					}))
				},
				RETURN => Return,
				CALL => Call(VarUint32::deserialize(reader)?.into()),
				CALLINDIRECT => {
					let signature: u32 = VarUint32::deserialize(reader)?.into();
					let table_ref: u8 = Uint8::deserialize(reader)?.into();
					if table_ref != 0 { return Err(Error::InvalidTableReference(table_ref)); }

					CallIndirect(
						signature,
						table_ref,
					)
				},
				DROP => Drop,
				SELECT => Select,

				GETLOCAL => GetLocal(VarUint32::deserialize(reader)?.into()),
				SETLOCAL => SetLocal(VarUint32::deserialize(reader)?.into()),
				TEELOCAL => TeeLocal(VarUint32::deserialize(reader)?.into()),
				GETGLOBAL => GetGlobal(VarUint32::deserialize(reader)?.into()),
				SETGLOBAL => SetGlobal(VarUint32::deserialize(reader)?.into()),

				I32LOAD => I32Load(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64LOAD => I64Load(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				F32LOAD => F32Load(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				F64LOAD => F64Load(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I32LOAD8S => I32Load8S(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I32LOAD8U => I32Load8U(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I32LOAD16S => I32Load16S(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I32LOAD16U => I32Load16U(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64LOAD8S => I64Load8S(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64LOAD8U => I64Load8U(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64LOAD16S => I64Load16S(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64LOAD16U => I64Load16U(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64LOAD32S => I64Load32S(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64LOAD32U => I64Load32U(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I32STORE => I32Store(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64STORE => I64Store(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				F32STORE => F32Store(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				F64STORE => F64Store(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I32STORE8 => I32Store8(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I32STORE16 => I32Store16(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64STORE8 => I64Store8(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64STORE16 => I64Store16(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),

				I64STORE32 => I64Store32(
					VarUint32::deserialize(reader)?.into(),
					VarUint32::deserialize(reader)?.into()),


				CURRENTMEMORY => {
					let mem_ref: u8 = Uint8::deserialize(reader)?.into();
					if mem_ref != 0 { return Err(Error::InvalidMemoryReference(mem_ref)); }
					CurrentMemory(mem_ref)
				},
				GROWMEMORY => {
					let mem_ref: u8 = Uint8::deserialize(reader)?.into();
					if mem_ref != 0 { return Err(Error::InvalidMemoryReference(mem_ref)); }
					GrowMemory(mem_ref)
				}

				I32CONST => I32Const(VarInt32::deserialize(reader)?.into()),
				I64CONST => I64Const(VarInt64::deserialize(reader)?.into()),
				F32CONST => F32Const(Uint32::deserialize(reader)?.into()),
				F64CONST => F64Const(Uint64::deserialize(reader)?.into()),
				I32EQZ => I32Eqz,
				I32EQ => I32Eq,
				I32NE => I32Ne,
				I32LTS => I32LtS,
				I32LTU => I32LtU,
				I32GTS => I32GtS,
				I32GTU => I32GtU,
				I32LES => I32LeS,
				I32LEU => I32LeU,
				I32GES => I32GeS,
				I32GEU => I32GeU,

				I64EQZ => I64Eqz,
				I64EQ => I64Eq,
				I64NE => I64Ne,
				I64LTS => I64LtS,
				I64LTU => I64LtU,
				I64GTS => I64GtS,
				I64GTU => I64GtU,
				I64LES => I64LeS,
				I64LEU => I64LeU,
				I64GES => I64GeS,
				I64GEU => I64GeU,

				F32EQ => F32Eq,
				F32NE => F32Ne,
				F32LT => F32Lt,
				F32GT => F32Gt,
				F32LE => F32Le,
				F32GE => F32Ge,

				F64EQ => F64Eq,
				F64NE => F64Ne,
				F64LT => F64Lt,
				F64GT => F64Gt,
				F64LE => F64Le,
				F64GE => F64Ge,

				I32CLZ => I32Clz,
				I32CTZ => I32Ctz,
				I32POPCNT => I32Popcnt,
				I32ADD => I32Add,
				I32SUB => I32Sub,
				I32MUL => I32Mul,
				I32DIVS => I32DivS,
				I32DIVU => I32DivU,
				I32REMS => I32RemS,
				I32REMU => I32RemU,
				I32AND => I32And,
				I32OR => I32Or,
				I32XOR => I32Xor,
				I32SHL => I32Shl,
				I32SHRS => I32ShrS,
				I32SHRU => I32ShrU,
				I32ROTL => I32Rotl,
				I32ROTR => I32Rotr,

				I64CLZ => I64Clz,
				I64CTZ => I64Ctz,
				I64POPCNT => I64Popcnt,
				I64ADD => I64Add,
				I64SUB => I64Sub,
				I64MUL => I64Mul,
				I64DIVS => I64DivS,
				I64DIVU => I64DivU,
				I64REMS => I64RemS,
				I64REMU => I64RemU,
				I64AND => I64And,
				I64OR => I64Or,
				I64XOR => I64Xor,
				I64SHL => I64Shl,
				I64SHRS => I64ShrS,
				I64SHRU => I64ShrU,
				I64ROTL => I64Rotl,
				I64ROTR => I64Rotr,
				F32ABS => F32Abs,
				F32NEG => F32Neg,
				F32CEIL => F32Ceil,
				F32FLOOR => F32Floor,
				F32TRUNC => F32Trunc,
				F32NEAREST => F32Nearest,
				F32SQRT => F32Sqrt,
				F32ADD => F32Add,
				F32SUB => F32Sub,
				F32MUL => F32Mul,
				F32DIV => F32Div,
				F32MIN => F32Min,
				F32MAX => F32Max,
				F32COPYSIGN => F32Copysign,
				F64ABS => F64Abs,
				F64NEG => F64Neg,
				F64CEIL => F64Ceil,
				F64FLOOR => F64Floor,
				F64TRUNC => F64Trunc,
				F64NEAREST => F64Nearest,
				F64SQRT => F64Sqrt,
				F64ADD => F64Add,
				F64SUB => F64Sub,
				F64MUL => F64Mul,
				F64DIV => F64Div,
				F64MIN => F64Min,
				F64MAX => F64Max,
				F64COPYSIGN => F64Copysign,

				I32WRAPI64 => I32WrapI64,
				I32TRUNCSF32 => I32TruncSF32,
				I32TRUNCUF32 => I32TruncUF32,
				I32TRUNCSF64 => I32TruncSF64,
				I32TRUNCUF64 => I32TruncUF64,
				I64EXTENDSI32 => I64ExtendSI32,
				I64EXTENDUI32 => I64ExtendUI32,
				I64TRUNCSF32 => I64TruncSF32,
				I64TRUNCUF32 => I64TruncUF32,
				I64TRUNCSF64 => I64TruncSF64,
				I64TRUNCUF64 => I64TruncUF64,
				F32CONVERTSI32 => F32ConvertSI32,
				F32CONVERTUI32 => F32ConvertUI32,
				F32CONVERTSI64 => F32ConvertSI64,
				F32CONVERTUI64 => F32ConvertUI64,
				F32DEMOTEF64 => F32DemoteF64,
				F64CONVERTSI32 => F64ConvertSI32,
				F64CONVERTUI32 => F64ConvertUI32,
				F64CONVERTSI64 => F64ConvertSI64,
				F64CONVERTUI64 => F64ConvertUI64,
				F64PROMOTEF32 => F64PromoteF32,

				I32REINTERPRETF32 => I32ReinterpretF32,
				I64REINTERPRETF64 => I64ReinterpretF64,
				F32REINTERPRETI32 => F32ReinterpretI32,
				F64REINTERPRETI64 => F64ReinterpretI64,

		

				_ => { return Err(Error::UnknownOpcode(val)); }
			}
		)
	}
}

