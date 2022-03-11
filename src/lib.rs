#[derive(Debug)]
pub enum DecodeError {
    EmptyBuffer,
    UnexpectedEndOfBuffer,
    TooLargeVarint,
    UnknownFieldNumber(usize),
    FieldOverflow(String),
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

    // Protobuf uses up to 64bit types for varints, which need 9 bytes in LEB128
    let mut tag_bytes = heapless::Vec::<u8, 9>::new();

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
        result = result | ((byte as u64) << (7*i));
    }
    Ok(result)
}

pub fn decode_leb128_u32<'a, I>(mut bytes: I) -> Result<u32, DecodeError>
where I: Iterator<Item = &'a u8> {
    let val = decode_leb128(bytes)?;
    // If the parsed value overflows a u32
    if ((val & 0xFF_FF_FF_FF_00_00_00_00) >> 32) != 0 {
        return Err(DecodeError::TooLargeVarint);
    }
    Ok((val & 0xFF_FF_FF_FF) as u32)
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

    let mut s: heapless::String<SIZE> = heapless::String::new();
    for i in 0..bufsize as usize {
        let mut byte = match bytes.next() {
            Some(byte) => s.push(byte.to_owned() as char),
            None => return Err(DecodeError::UnexpectedEndOfBuffer),
        };
    }

    Ok(s)
}
