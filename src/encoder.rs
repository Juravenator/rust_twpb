use crate::{traits::Writer, WriterError};

#[allow(unused_mut)] // for signature consistency
fn write_u8(bytes: &mut impl Writer, input: u8) -> Result<usize, WriterError> {
    write(bytes, &[input])
}

fn write(bytes: &mut impl Writer, input: &[u8]) -> Result<usize, WriterError> {
    let mut len = 0;
    for byte in input {
        bytes.write(*byte)?;
        len += 1;
    }
    Ok(len)
}

pub fn leb128(bytes: &mut impl Writer, input: &u64) -> Result<usize, WriterError> {
    let mut bytes_written = 0;
    let mut input = *input; // make a cloned copy of readonly input

    // LEB128 encoded numbers are split up in 7-bit chunks
    // the 1st bit (MSB) denotes wether or not it is the last chunk (0) or not (1).
    // as long as our value to encode still needs the MSB, we are not at the last chunk
    while input >= 0x80 {
        bytes_written += write_u8(bytes, ((input & 0x7F) | 0x80) as u8)?;
        input >>= 7;
    }

    // Write our last chunk
    bytes_written += write_u8(bytes, (input & 0xFF) as u8)?;

    Ok(bytes_written)
}

pub fn leb128_u32(bytes: &mut impl Writer, input: &u32) -> Result<usize, WriterError> {
    leb128(bytes, &(*input as u64))
}

pub fn leb128_i64(bytes: &mut impl Writer, input: &i64) -> Result<usize, WriterError> {
    leb128(bytes, &(*input as u64))
}

pub fn leb128_i32(bytes: &mut impl Writer, input: &i32) -> Result<usize, WriterError> {
    leb128(bytes, &(*input as u64))
}

pub fn tag(bytes: &mut impl Writer, field_number: &u32, wire_type: &u8) -> Result<usize, WriterError> {
    // Wire type is specified using the 3 LSBs.
    // Field type is specified using the (32-3)=29 bits next to that.
    if (field_number & 0xE0_00_00_00) != 0 {
        return Err(WriterError::BufferOverflow);
    }
    if (wire_type & 0xF8) != 0 {
        return Err(WriterError::BufferOverflow);
    }
    leb128_u32(bytes, &(((*field_number << 3) & 0xFF_FF_FF_F8) | (*wire_type & 0b0111) as u32))
}

pub fn string<const SIZE: usize>(bytes: &mut impl Writer, input: &heapless::String<SIZE>) -> Result<usize, WriterError> {
    let b = input.as_bytes();
    let mut bytes_written = 0;

    // Write the size bits
    bytes_written += leb128_u32(bytes, &(b.len() as u32))?;

    bytes_written += write(bytes, b)?;
    Ok(bytes_written)
}

pub fn int32(bytes: &mut impl Writer, input :&i32) -> Result<usize, WriterError> {
    leb128_i32(bytes, input)
}

pub fn int64(bytes: &mut impl Writer, input :&i64) -> Result<usize, WriterError> {
    leb128_i64(bytes, input)
}

pub fn uint32(bytes: &mut impl Writer, input :&u32) -> Result<usize, WriterError> {
    leb128_u32(bytes, input)
}

pub fn uint64(bytes: &mut impl Writer, input :&u64) -> Result<usize, WriterError> {
    leb128(bytes, input)
}

pub fn sint32(bytes: &mut impl Writer, input :&i32) -> Result<usize, WriterError> {
    // sint32/64 values are identical to their int32/64 counterparts, except that they
    // use ZigZag encoding to prevent negative numbers immediately taking up 10 bytes in leb128.
    // They do this by mapping low->high signed numbers to low->high unsigned numbers.
    // E.g 0=0, -1=1, 1=2, -2=3, 2=4, -3=5, ...

    // We use a couple of tricks.

    // First off, notice in the example above that all positive signed numbers
    // are mapped to even unsigned numbers. So we'll get the 'sign bit'.
    let sign = (*input < 0) as i32;
    // Secondly, because the way zigzag works, even numbers match to their *2 value in
    // unsigned encoding (1=2, 2=4, 3=6, ...).
    // So for even numbers, things would be as easy as a bit shift left.
    // For uneven numbers, we observe the same followed by
    // an inversion + -1. (-1=1, -2=2*2-1=3, -3=3*2-1=5, -4=7)
    // We can cover both cases with an XOR.
    // We bit shift left to achieve *2,
    // then apply an XOR on an inversed sign bit to either perform -1 or -0.
    // Examples: -2 maps to 3 and 2 maps to 4
    // ..1110 (-2) => sign=1 => ..11100 XOR ..1111 (-1) = ..0011 (3)
    // ..0010 (2)  => sign=0 => ..00100 XOR ..0000 (-0) = ..0100 (4)
    let converted = (*input << 1) ^ -sign;

    leb128_i32(bytes, &converted)
}

pub fn sint64(bytes: &mut impl Writer, input: &i64) -> Result<usize, WriterError> {
    // Same as sint32, but with 64 bit numbers
    let sign = (*input < 0) as i64;
    let converted = (*input << 1) ^ -sign;

    leb128_i64(bytes, &converted)
}

pub fn fixed32(bytes: &mut impl Writer, input: &u32) -> Result<usize, WriterError> {
    write(bytes, &input.to_le_bytes()[..])
}

pub fn fixed64(bytes: &mut impl Writer, input: &u64) -> Result<usize, WriterError> {
    write(bytes, &input.to_le_bytes()[..])
}

pub fn sfixed32(bytes: &mut impl Writer, input: &i32) -> Result<usize, WriterError> {
    write(bytes, &input.to_le_bytes()[..])
}

pub fn sfixed64(bytes: &mut impl Writer, input: &i64) -> Result<usize, WriterError> {
    write(bytes, &input.to_le_bytes()[..])
}

pub fn float(bytes: &mut impl Writer, input: &f32) -> Result<usize, WriterError> {
    write(bytes, &input.to_le_bytes()[..])
}

pub fn double(bytes: &mut impl Writer, input: &f64) -> Result<usize, WriterError> {
    write(bytes, &input.to_le_bytes()[..])
}

pub fn bool(bytes: &mut impl Writer, input: &bool) -> Result<usize, WriterError> {
    write_u8(bytes, *input as u8)
}

pub fn bytes<const SIZE: usize>(bytes: &mut impl Writer, input: &heapless::Vec<u8, SIZE>) -> Result<usize, WriterError> {
    let mut bytes_written = 0;
    // Write the size bits
    bytes_written += leb128_u32(bytes, &(input.len() as u32))?;

    bytes_written += write(bytes, input.as_ref())?;

    Ok(bytes_written)
}