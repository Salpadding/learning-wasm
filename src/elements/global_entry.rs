use super::types::{ValueType};
use super::ops::{Instruction, InitExpr};
use super::{Deserialize, Error};
use super::import_entry::{GlobalType};
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalEntry {
    pub global_type: GlobalType,
    pub init_expr: InitExpr,
}


impl Deserialize for GlobalEntry {
    type Error = Error;

	fn deserialize<R: io::Read>(reader: &mut R) -> Result<GlobalEntry, Error> {
        let global_type = GlobalType::deserialize(reader)?;
        let init_expr = InitExpr::deserialize(reader)?;
        Ok (
            GlobalEntry { global_type, init_expr }
        )
    }
}


