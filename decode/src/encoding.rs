use std::io::Write;

pub trait Encodable {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error>;
}

pub enum EncodingError {
    NotEnoughData(String),
}

mod bytes {
    use super::*;

    impl Encodable for u8 {
        fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
            writer.write_all(&[*self])?;
            Ok(())
        }
    }
}

mod string {
    use super::*;

    impl Encodable for String {
        fn encode<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
            writer.write(self.as_bytes())?;
            Ok(())
        }
    }
}
