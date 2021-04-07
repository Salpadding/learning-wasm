use super::ops::InitExpr;
use super::{Deserialize, Error};
use std::io;
use crate::elements::primitives::{VarUint32, CountedList};

#[cfg(feature = "reduced-stack-buffer")]
const VALUES_BUFFER_LENGTH: usize = 256;

#[cfg(not(feature = "reduced-stack-buffer"))]
const VALUES_BUFFER_LENGTH: usize = 16384;

/// Entry in the element section.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementSegment {
    pub index: u32,
    pub offset: Option<InitExpr>,
    pub members: Vec<u32>,
}

impl Deserialize for ElementSegment {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let index: u32 = VarUint32::deserialize(reader)?.into();
        let offset = InitExpr::deserialize(reader)?;
        let members: Vec<u32> = CountedList::<VarUint32>::deserialize(reader)?
            .into_inner()
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(ElementSegment {
            index,
            offset: Some(offset),
            members,
        })
    }
}

/// Data segment definition.
#[derive(Clone, Debug, PartialEq)]
pub struct DataSegment {
    pub index: u32,
    pub offset: Option<InitExpr>,
    pub value: Vec<u8>,
}

impl Deserialize for DataSegment {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let index = VarUint32::deserialize(reader)?;
        let offset = InitExpr::deserialize(reader)?;
        let value_len = u32::from(VarUint32::deserialize(reader)?) as usize;
        let value_buf = buffered_read!(VALUES_BUFFER_LENGTH, value_len, reader);

        Ok(DataSegment {
            index: index.into(),
            offset: Some(offset),
            value: value_buf,
        })
    }
}