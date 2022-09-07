#[test]
fn test_leb128_decode() {
    // Example value 300
    // https://developers.google.com/protocol-buffers/docs/encoding#varints
    let val = ::twpb::decoder::leb128_u32([0xAC, 0x02].into_iter()).unwrap();
    assert_eq!(val, 300);

    let val = ::twpb::decoder::leb128_i32([0xBB, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01].into_iter()).unwrap();
    assert_eq!(val, -69);

    let val = ::twpb::decoder::leb128_i64([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01].into_iter()).unwrap();
    assert_eq!(val, i64::MIN);
}

#[test]
fn test_leb128_decode_overflows() {
    // encoded i64::MIN
    let result = ::twpb::decoder::leb128_i32([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01].into_iter()).unwrap_err();
    assert_eq!(result, ::twpb::decoder::DecodeError::TooLargeVarint);
    let result = ::twpb::decoder::leb128_u32([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01].into_iter()).unwrap_err();
    assert_eq!(result, ::twpb::decoder::DecodeError::TooLargeVarint);
}

#[test]
fn test_leb128_encode() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::leb128(&mut buffer.as_mut(), &300).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(buffer[0..bytes_written], [0xAC, 0x02]);
}

#[test]
fn test_leb128_encode_overflows() {
    let mut buffer = [0x0; 100];
    ::twpb::encoder::leb128(&mut buffer.as_mut(), &u64::MAX).unwrap();
    let val = ::twpb::decoder::leb128(buffer.into_iter()).unwrap();
    assert_eq!(val, u64::MAX);

    let mut buffer = [0x0; 100];
    ::twpb::encoder::leb128_i64(&mut buffer.as_mut(), &i64::MAX).unwrap();
    let val = ::twpb::decoder::leb128_i64(buffer.into_iter()).unwrap();
    assert_eq!(val, i64::MAX);

    let mut buffer = [0x0; 100];
    ::twpb::encoder::leb128_i32(&mut buffer.as_mut(), &i32::MIN).unwrap();
    let val = ::twpb::decoder::leb128_i32(buffer.into_iter()).unwrap();
    assert_eq!(val, i32::MIN);

    let mut buffer = [0x0; 100];
    ::twpb::encoder::leb128(&mut buffer.as_mut(), &300).unwrap();
    let val = ::twpb::decoder::leb128(buffer.into_iter()).unwrap();
    assert_eq!(val, 300);
}