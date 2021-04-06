use std::io::{Read, Result, ErrorKind, Error};

pub struct ByteStream<'a> (pub &'a [u8]);

/// io::Read 实现，用于单元测试
impl Read for ByteStream<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.0.len() == 0 {
            let e: Error = ErrorKind::UnexpectedEof.into();
            return Err(e);
        }
        let min = if buf.len() > self.0.len() {self.0.len()} else { buf.len() };
        buf.copy_from_slice(&self.0[0..min]);
        self.0 = &self.0[min..];
        Ok(min)
    }    
}
