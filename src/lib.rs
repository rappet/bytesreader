extern crate bytes;
extern crate memchr;

use bytes::BytesMut;
use std::io::Read;

pub struct BytesBufReader<R>
where
    R: Read,
{
    inner: R,
    buffer: BytesMut,
}

impl<R> BytesBufReader<R>
where
    R: Read,
{
    pub fn new(inner: R) -> BytesBufReader<R> {
        BytesBufReader {
            inner,
            buffer: BytesMut::new(),
        }
    }

    pub fn read_until(&mut self, byte: u8) -> std::io::Result<BytesMut> {
        if let Some(position) = memchr::memchr(byte, self.buffer.as_ref()) {
            Ok(self.buffer.split_to(position + 1))
        } else {
            let old_length = self.buffer.len();
            self.buffer = self.buffer.clone();

            loop {
                let bytes_read = self.extend_from_stream(1024 * 63)?;
                //let bytes_read = self.extend_from_stream(128)?;

                // End of stream, no new data
                if self.buffer.is_empty() {
                    return Err(std::io::ErrorKind::UnexpectedEof.into());
                }

                // End of stream, new data but no delimiter found
                if bytes_read == 0 {
                    return Ok(self.buffer.split());
                }

                if let Some(position_relative) =
                    memchr::memchr(byte, &self.buffer.as_ref()[old_length..])
                {
                    let position = position_relative + old_length;
                    return Ok(self.buffer.split_to(position + 1));
                }
            }
        }
    }

    fn extend_from_stream(&mut self, len: usize) -> std::io::Result<usize> {
        let old_size = self.buffer.len();
        self.buffer.resize(old_size + len, 0);
        let bytes_read = self.inner.read(&mut self.buffer.as_mut()[old_size..])?;
        self.buffer.truncate(old_size + bytes_read);
        Ok(bytes_read)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
