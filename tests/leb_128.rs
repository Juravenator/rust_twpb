#[test]
fn test_leb128_decode() {
    // Example value 300
    // https://developers.google.com/protocol-buffers/docs/encoding#varints
    let val = ::twpb::decoder::leb128_u32([0xAC, 0x02].iter()).unwrap();
    assert_eq!(val, 300);

    let val = ::twpb::decoder::leb128_i32([0xBB, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01].iter()).unwrap();
    assert_eq!(val, -69);

    let val = ::twpb::decoder::leb128_i64([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01].iter()).unwrap();
    assert_eq!(val, i64::MIN);
}

#[test]
fn test_leb128_decode_overflows() {
    // encoded i64::MIN
    ::twpb::decoder::leb128_i32([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01].iter()).unwrap_err();
    ::twpb::decoder::leb128_u32([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01].iter()).unwrap_err();
}

#[test]
fn test_leb128_encode() {
    let mut buffer = Vec::<u8>::new();
    let bytes_written = ::twpb::encoder::leb128(&mut buffer, 300).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(buffer, [0xAC, 0x02]);
}

#[test]
fn test_leb128_encode_overflows() {
    let mut buffer = Vec::<u8>::new();
    ::twpb::encoder::leb128(&mut buffer, u64::MAX).unwrap();
    let val = ::twpb::decoder::leb128(buffer.iter()).unwrap();
    assert_eq!(val, u64::MAX);

    let mut buffer = Vec::<u8>::new();
    ::twpb::encoder::leb128_i64(&mut buffer, i64::MAX).unwrap();
    let val = ::twpb::decoder::leb128_i64(buffer.iter()).unwrap();
    assert_eq!(val, i64::MAX);

    let mut buffer = Vec::<u8>::new();
    ::twpb::encoder::leb128_i32(&mut buffer, i32::MIN).unwrap();
    let val = ::twpb::decoder::leb128_i32(buffer.iter()).unwrap();
    assert_eq!(val, i32::MIN);

    let mut buffer = Vec::<u8>::new();
    ::twpb::encoder::leb128(&mut buffer, 300).unwrap();
    let val = ::twpb::decoder::leb128(buffer.iter()).unwrap();
    assert_eq!(val, 300);
}