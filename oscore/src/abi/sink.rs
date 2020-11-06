use super::Encoder;

///Encoding different types of data into byte array.
pub struct Sink {
    buf: Vec<u8>,
}

impl Sink {
    ///Create a new sink entity, Specify initial capacity.
    ///For indefinite length parameters, the length of the parameter will be serialized first, and then the content of the parameter will be serialized.

    ///
    ///# Example
    ///```no_run
    /// # use oscore::abi::Sink;
    /// let mut sink = Sink::new(0);
    /// sink.write("123");
    /// assert_eq!(sink.bytes(),[3,49,50,51]);
    ///```
    pub fn new(cap: usize) -> Self {
        Sink {
            buf: Vec::with_capacity(cap),
        }
    }

    ///All data types that implement the encode interface can be serialized by calling the write method
    ///# Example
    ///```
    /// # use oscore::abi::Sink;
    ///   let mut sink = Sink::new(0);
    ///   sink.write("123");
    ///   sink.write(123 as U128);
    ///```
    pub fn write<T: Encoder>(&mut self, val: T) {
        val.encode(self)
    }

    pub(crate) fn write_byte(&mut self, b: u8) {
        self.buf.push(b)
    }

    pub(crate) fn write_bool(&mut self, b: bool) {
        if b {
            self.write_byte(1)
        } else {
            self.write_byte(0)
        }
    }

    pub(crate) fn write_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data)
    }

    pub(crate) fn write_u16(&mut self, val: u16) {
        self.write_bytes(&val.to_le_bytes())
    }

    pub(crate) fn write_u32(&mut self, val: u32) {
        self.write_bytes(&val.to_le_bytes())
    }

    pub(crate) fn write_u64(&mut self, val: u64) {
        self.write_bytes(&val.to_le_bytes())
    }

    pub(crate) fn write_varuint(&mut self, val: u64) {
        if val < 0xFD {
            self.write_byte(val as u8);
        } else if val < 0xFFFF {
            self.write_byte(0xFD);
            self.write_u16(val as u16);
        } else if val <= 0xFFFF_FFFF {
            self.write_byte(0xFE);
            self.write_u32(val as u32);
        } else {
            self.write_byte(0xFF);
            self.write_u64(val);
        }
    }

    ///Used to get the serialized result in bytearray format
    /// # Example
    /// ```
    /// #![feature(proc_macro_hygiene)]
    /// use oscore::macros::base58;
    /// use oscore::types::Address;
    /// use oscore::abi::Sink;
    /// const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
    /// let mut sink = Sink::new(0);
    /// sink.write(&ONT_CONTRACT_ADDRESS);
    /// assert_eq!(sink.into(), [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].to_vec())
    /// ```
    pub fn bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn into(self) -> Vec<u8> {
        self.buf
    }
}
