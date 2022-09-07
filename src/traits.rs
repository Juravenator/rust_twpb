use defmt::Format;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Format)]
pub enum WriterError {
    BufferOverflow,
}

pub trait Writer {
    fn write(&mut self, byte: u8) -> Result<(), WriterError>;

    fn write_all(&mut self, bytes: &[u8]) -> Result<usize, WriterError> {
        for byte in bytes {
            if let Err(err) = self.write(*byte) {
                return Err(err);
            }
        }
        Ok(bytes.len())
    }
}

pub trait MessageEncoder {
    fn twpb_encode(&self, buffer: &mut impl Writer) -> Result<usize, WriterError>;
}
pub trait MessageDecoder: Sized {
    fn twpb_decode(buf: &[u8]) -> Result<Self, crate::decoder::DecodeError> {
        Self::twpb_decode_iter(buf.iter().map(|x| *x))
    }

    fn twpb_decode_iter<I>(bytes: I) -> Result<Self, crate::decoder::DecodeError>
    where I: Iterator<Item = u8>;
}

impl Writer for &mut [u8] {
    #[inline]
    fn write(&mut self, byte: u8) -> Result<(), WriterError> {
        if self.is_empty() {
            return Err(WriterError::BufferOverflow);
        }

        // We need to make edits that directly working with
        // `self` wouldn't allow. Temporarily swap with an empty array.
        let orig = core::mem::replace(self, &mut []);

        // Remove the first element from the original array.
        let (a, orig) = orig.split_first_mut().unwrap();

        // Put it back.
        let _ = core::mem::replace(self, orig);

        // Write to the address of the (now-removed) array element.
        *a = byte;

        Ok(())
    }
}