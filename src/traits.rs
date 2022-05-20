pub trait MessageEncoder {
    fn twpb_encode(&self, buffer: impl bytes::BufMut) -> Result<usize, crate::encoder::EncodeError>;
}
pub trait MessageDecoder: Sized {
    fn twpb_decode(buf: &[u8]) -> Result<Self, crate::decoder::DecodeError> {
        Self::twpb_decode_iter(buf.iter())
    }

    fn twpb_decode_iter<'a, I>(bytes: I) -> Result<Self, crate::decoder::DecodeError>
    where I: Iterator<Item = &'a u8>;
}