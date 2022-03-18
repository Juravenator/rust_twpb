use core::str::FromStr;

#[derive(Debug)]
pub enum DecodeError {
    EmptyBuffer,
    UnexpectedEndOfBuffer,
    TooLargeVarint,
    StringParseError,
    UnknownFieldNumber(usize),
    FieldOverflow(String),
    WrongWireType(u8, String),
}

// https://developers.google.com/protocol-buffers/docs/encoding#structure
pub mod WireTypes {
    // int32, int64, uint32, uint64, sint32, sint64, bool, enum
    pub const Varint: u8 = 0;
    // fixed64, sfixed64, double
    pub const B64: u8 = 1;
    // string, bytes, embedded messages, packed repeated fields
    pub const LengthDelimited: u8 = 2;
    // groups (deprecated)
    pub const StartGroup: u8 = 3;
    // groups (deprecated)
    pub const EndGroup: u8 = 4;
    // fixed32, sfixed32, float
    pub const B32: u8 = 5;
}


pub struct LimitedIterator<'a, 'b, I> where I: Iterator<Item = &'b u8> {
    source_iterator: &'a mut I,
    range: u32,
    current_index: u32,
}

impl<'a, 'b, I> LimitedIterator<'a, 'b, I> where I: Iterator<Item = &'b u8> {
    pub fn new(source_iterator: &'a mut I, range: u32) -> Self {
        LimitedIterator{
            source_iterator: source_iterator,
            range: range,
            current_index: 0,
        }
    }
}

impl<'a, 'b, I> Iterator for LimitedIterator<'a, 'b, I> where I: Iterator<Item = &'b u8> {
    type Item = &'b u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.range {
            self.current_index += 1;
            self.source_iterator.next()
        } else {
            None
        }
    }
}

pub fn decode_leb128<'a, I>(mut bytes: I) -> Result<u64, DecodeError>
where I: Iterator<Item = &'a u8> {
    // LEB128 encoded numbers are split up in 7-bit chunks
    // the 1st bit (MSB) denotes wether or not it is the last chunk (0) or not (1).
    let mut last_encountered_msb = true;

    // Protobuf uses up to 64bit types for unsigned varints, which need 9 bytes in LEB128
    // However, signed varints use 11...
    let mut tag_bytes = heapless::Vec::<u8, 11>::new();

    // Remove those MSBs and collect our chunks.
    // We keep the 7-bit chunks in an 8-bit byte, this is not a problem,
    // during re-assembly we XOR anyways
    while last_encountered_msb {
        println!("getting varint byte");
        // As long as last MSB == 1, we need to read in more bytes
        if let Some(mut byte) = bytes.next() {
            println!("varint byte {:X}", byte);
            last_encountered_msb = byte & 0x80 != 0;
            println!("MSB {:?}", last_encountered_msb);
            // push() returns to sender if the vec capacity has been exceeded
            if let Err(_) = tag_bytes.push(byte & 0x7F) {
                return Err(DecodeError::TooLargeVarint{});
            }
        // If the byte stream is empty, but we were already busy decoding
        } else if tag_bytes.len() != 0 {
            return Err(DecodeError::UnexpectedEndOfBuffer{});
        // If we were passed an empty byte stream, no work to do, no u64 for you
        } else {
            return Err(DecodeError::EmptyBuffer);
        }
    }

    // Re-assemble into a u64 (which is the max varint size in protobuf).
    // It is also in little endian,
    // so we need to swap the order of the 7-bit chunks.
    let mut result: u64 = 0;
    for (i, byte) in tag_bytes.into_iter().enumerate() {
        if i < 10 {
            result = result | ((byte as u64) << (7*i));
        } else {
            // Hack to not have to use u128 instead of u64.
            // Signed integers always encode into 10 bytes, and end with 0x01, 
            // which after decoding is __ONE__ bit too much for u64.
            // Since it only serves as a stop byte, throw it away.
            // If this _isn't_ 0x01 however, hello overflow.
            if !(i == 10 && byte == 0x01) {
                return Err(DecodeError::TooLargeVarint{});
            }
        }
    }
    Ok(result)
}

pub fn decode_leb128_u32<'a, I>(mut bytes: I) -> Result<u32, DecodeError>
where I: Iterator<Item = &'a u8> {
    let val = decode_leb128(bytes)?;
    // If the parsed value overflows a u32
    if ((val & 0xFF_FF_FF_FF_00_00_00_00) >> 32) != 0 {
        Err(DecodeError::TooLargeVarint)
    } else {
        Ok((val & 0xFF_FF_FF_FF) as u32)
    }
}

pub fn decode_leb128_i64<'a, I>(mut bytes: I) -> Result<i64, DecodeError>
where I: Iterator<Item = &'a u8> {
    let val = decode_leb128(bytes)?;
    Ok(val as i64)
}

pub fn decode_leb128_i32<'a, I>(mut bytes: I) -> Result<i32, DecodeError>
where I: Iterator<Item = &'a u8> {
    let val = decode_leb128_i64(bytes)?;
    match i32::try_from(val) {
        Ok(val) => Ok(val),
        Err(_) => Err(DecodeError::TooLargeVarint),
    }
}


pub fn decode_tag<'a, I>(mut bytes: I) -> Result<(u32, u8), DecodeError>
where I: Iterator<Item = &'a u8> {
    let val = decode_leb128_u32(bytes)?;
    // Wire type is specified using the 3 LSBs.
    // Mask all but those 3 bits and convert to u8.
    let wire_type = (val & 0b0111) as u8;
    // Field type is specified using the (32-3)=29 bits next to that.
    // Mask 3LSB bits and everything overflowing a u32.
    let field_number = ((val & 0xFF_FF_FF_F8) >> 3) as u32;
    return Ok((field_number, wire_type));
}

pub fn decode_string<'a, const SIZE: usize, I>(mut bytes: I, field_name: &str) -> Result<heapless::String<SIZE>, DecodeError>
where I: Iterator<Item = &'a u8> {
    println!("decoding string of max size {}", SIZE);

    let bufsize = decode_leb128_u32(&mut bytes)?;

    println!("string in protobuf is size {}", bufsize);
    if bufsize > (SIZE as u32) {
        return Err(DecodeError::FieldOverflow(field_name.to_string()))
    }

    let mut strbuf = heapless::Vec::<u8, SIZE>::new();
    for i in 0..bufsize as usize {
        let mut byte = match bytes.next() {
            Some(byte) => strbuf.push(*byte),
            None => return Err(DecodeError::UnexpectedEndOfBuffer),
        };
    }
    let s = core::str::from_utf8(&strbuf).or(Err(DecodeError::StringParseError))?;
    heapless::String::from_str(s).or(Err(DecodeError::StringParseError))
}

pub fn decode_int32<'a, I>(mut bytes: I, field_name: &str) -> Result<i32, DecodeError>
where I: Iterator<Item = &'a u8> {
    decode_leb128_i32(&mut bytes)
}

pub fn decode_int64<'a, I>(mut bytes: I, field_name: &str) -> Result<i64, DecodeError>
where I: Iterator<Item = &'a u8> {
    decode_leb128_i64(&mut bytes)
}

pub fn decode_uint32<'a, I>(mut bytes: I, field_name: &str) -> Result<u32, DecodeError>
where I: Iterator<Item = &'a u8> {
    decode_leb128_u32(&mut bytes)
}

pub fn decode_uint64<'a, I>(mut bytes: I, field_name: &str) -> Result<u64, DecodeError>
where I: Iterator<Item = &'a u8> {
    decode_leb128(&mut bytes)
}

pub fn decode_sint32<'a, I>(mut bytes: I, field_name: &str) -> Result<i32, DecodeError>
where I: Iterator<Item = &'a u8> {
    let value = decode_leb128_u32(&mut bytes)?;
    // sint32/64 values are identical to their int32/64 counterparts, except that they
    // use ZigZag encoding to prevent negative numbers immediately taking up 10 bytes in leb128.
    // They do this by mapping low->high signed numbers to low->high unsigned numbers.
    // E.g 0=0, -1=1, 1=2, -2=3, 2=4, -3=5, ...

    // Notice in the example above that all positive signed numbers
    // are mapped to even unsigned numbers.
    // So in this zigzag encoding, the LSB is acting as a sign bit.
    // Remove it to obtain the absolute value.
    let abs = (value >> 1) as i32;

    // Now get the sign bit.
    // Mask out everything except the LSB.
    let sign = (value & 1) as i32;
    // Then perform two's complement if the sign bit is set.
    // To perform two's complement, one inverts and adds +1.
    // We can cover both cases with an XOR.
    // Examples: 3 maps to -2 and 4 maps to 2
    // 0011 (3) => abs=..001, sign=..01 => ..001 XOR ..1111 (-1) = ..1110 (-2)
    // 0100 (4) => abs=..010, sign=..00 => ..010 XOR ..0000 (-0) = ..0010 (2)
    Ok(abs ^ -sign)
}

pub fn decode_sint64<'a, I>(mut bytes: I, field_name: &str) -> Result<i64, DecodeError>
where I: Iterator<Item = &'a u8> {
    // same as decode_sint32, but everything is 64
    let value = decode_leb128(&mut bytes)?;
    let abs = (value >> 1) as i64;
    let sign = (value & 1) as i64;
    Ok(abs ^ -sign)
}

pub fn decode_unknown<'a, I>(mut bytes: I, wire_type: u8) -> Result<(), DecodeError>
where I: Iterator<Item = &'a u8> {
    match wire_type {
        WireTypes::Varint => {
            decode_leb128_u32(&mut bytes)?;
        }
        WireTypes::B32 => {
            for _ in 0..32/8 {
                bytes.next();
            }
        },
        WireTypes::B64 => {
            for _ in 0..64/8 {
                bytes.next();
            }
        },
        WireTypes::LengthDelimited => {
            let bufsize = decode_leb128_u32(&mut bytes)?;
            for _ in 0..bufsize {
                bytes.next();
            }
        },
        WireTypes::StartGroup | WireTypes::EndGroup => panic!("Groups are not supported"),
        _ => panic!("unsupported wire type '{}'", wire_type),
    };
    Ok(())
}