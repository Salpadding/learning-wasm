use std::io;

// io::Reader 在遇到 eof 时可能会返回 Ok(0)，而不是 Err(EOF)
pub struct BufReader<'a, T: io::Read> {
    reader: &'a mut T,
}

impl<'a, T: io::Read> BufReader<'a, T> {
    pub fn new(r: &'a mut T) -> BufReader<'a, T> {
        BufReader {
            reader: r
        }
    }
}

impl<T: io::Read> io::Read for BufReader<'_, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let l = self.reader.read(buf)?;
        if l == 0 {
            return Err(
                io::ErrorKind::UnexpectedEof.into()
            );
        }
        Ok(l)
    }
}