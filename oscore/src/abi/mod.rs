mod codec;
mod sink;
mod source;

pub use self::sink::Sink;
pub use self::source::Source;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    IrregularData,
    InvalidUtf8,
    TypeInconsistency,
    LengthInconsistency,
}

pub trait Encoder {
    fn encode(&self, sink: &mut Sink);
}

#[doc(hidden)]
pub trait Decoder<'a>: Sized {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error>;
}
