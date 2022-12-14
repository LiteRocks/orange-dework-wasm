use super::Decoder;
use super::Error;
use std::convert::TryInto;

pub(crate) fn varuint_encode_size(val: u64) -> usize {
    if val < 0xfd {
        1
    } else if val <= 0xffff {
        3
    } else if val <= 0xFFFF_FFFF {
        5
    } else {
        9
    }
}

///Parse data of bytearray type into original data type
pub struct Source<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Source<'a> {
    ///Create a new source instance
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write("123");
    ///   let mut source = Source::new(sink.bytes());
    ///   let res:&str = source.read().unwrap_or_default();
    ///   assert_eq!(res, "123");
    /// ```
    pub fn new(data: &'a [u8]) -> Self {
        Self { buf: data, pos: 0 }
    }

    pub(crate) fn next_bytes(&mut self, len: usize) -> Result<&'a [u8], Error> {
        if self.buf.len() - self.pos < len {
            Err(Error::UnexpectedEOF)
        } else {
            let bytes = &self.buf[self.pos..self.pos + len];
            self.pos += len;
            Ok(bytes)
        }
    }
    ///read bytearray
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write("123".as_bytes());
    ///   let mut source = Source::new(sink.bytes());
    ///   let res= source.read_bytes().unwrap_or_default();
    ///   assert_eq!(res, "123".as_bytes());
    /// ```
    pub fn read_bytes(&mut self) -> Result<&'a [u8], Error> {
        let n = self.read_varuint()?;
        self.next_bytes(n as usize)
    }

    ///Parse the bytearray data into the original data type. The original data type must implement the decoder interface.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write("123");
    ///   sink.write(123u128);
    ///   let mut source = Source::new(sink.bytes());
    ///   let res :&str= source.read().unwrap();
    ///   let res2 :u128 = source.read().unwrap();
    ///   assert_eq!(res, "123");
    ///   assert_eq!(res2, 123u128);
    /// ```
    pub fn read<T: Decoder<'a>>(&mut self) -> Result<T, Error> {
        T::decode(self)
    }

    pub(crate) fn read_into(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let bytes = self.next_bytes(buf.len())?;
        buf.copy_from_slice(bytes);
        Ok(())
    }
    ///read byte.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write(b'1');
    ///   let mut source = Source::new(sink.bytes());
    ///   let res= source.read_byte().unwrap_or_default();
    ///   assert_eq!(res, b'1');
    /// ```
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        if self.pos >= self.buf.len() {
            Err(Error::UnexpectedEOF)
        } else {
            let b = self.buf[self.pos];
            self.pos += 1;
            Ok(b)
        }
    }

    ///read bool.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    /// let mut sink = Sink::new(0);
    /// sink.write(true);
    /// let mut source = Source::new(sink.bytes());
    /// let res= source.read_bool().unwrap_or_default();
    /// assert_eq!(res, true);
    /// ```
    pub fn read_bool(&mut self) -> Result<bool, Error> {
        match self.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::IrregularData),
        }
    }

    #[allow(unused)]
    pub fn skip(&mut self, n: usize) -> Result<(), Error> {
        if self.buf.len() - self.pos < n {
            Err(Error::UnexpectedEOF)
        } else {
            self.pos += n;
            Ok(())
        }
    }

    ///Back specified length of bytes.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write(123 as U128);
    ///   let mut source = Source::new(sink.bytes());
    ///   source.read_byte();//Read a byte of data here
    ///   source.backup(1);//Back one byte
    ///   let res = source.read_u128().unwrap_or_default();
    ///   assert_eq!(res, 123 as U128);
    /// ```
    #[allow(unused)]
    pub fn backup(&mut self, n: usize) {
        assert!(self.pos >= n);
        self.pos -= n;
    }
    ///Read u16 type data.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write(123u16);
    ///   let mut source = Source::new(sink.bytes());
    ///   let res = source.read_u16().unwrap_or_default();
    ///   assert_eq!(res, 123u16);
    /// ```
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.next_bytes(2)?.try_into().unwrap()))
    }
    ///Read u32 type data.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write(123u32);
    ///   let mut source = Source::new(sink.bytes());
    ///   let res = source.read_u32().unwrap_or_default();
    ///   assert_eq!(res, 123u32);
    /// ```
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.next_bytes(4)?.try_into().unwrap()))
    }

    ///Read u64 type data.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write(123u64);
    ///   let mut source = Source::new(sink.bytes());
    ///   let res = source.read_u64().unwrap_or_default();
    ///   assert_eq!(res, 123u64);
    /// ```
    pub fn read_u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.next_bytes(8)?.try_into().unwrap()))
    }

    ///Read u128 type data.
    /// # Example
    /// ```
    /// # use oscore::abi::{Source, Sink};
    ///   let mut sink = Sink::new(0);
    ///   sink.write(123 as U128);
    ///   let mut source = Source::new(sink.bytes());
    ///   let res = source.read_u128().unwrap_or_default();
    ///   assert_eq!(res, 123 as U128);
    /// ```
    pub fn read_u128(&mut self) -> Result<u128, Error> {
        Ok(u128::from_le_bytes(
            self.next_bytes(16)?.try_into().unwrap(),
        ))
    }

    pub fn read_varuint(&mut self) -> Result<u64, Error> {
        match self.read_byte()? {
            0xFD => self.read_u16().map(|v| (3, v as u64)),
            0xFE => self.read_u32().map(|v| (5, v as u64)),
            0xFF => self.read_u64().map(|v| (9, v)),
            val => Ok((1, val as u64)),
        }
        .and_then(|(len, val)| {
            if len == varuint_encode_size(val) {
                Ok(val)
            } else {
                Err(Error::IrregularData)
            }
        })
    }
}
