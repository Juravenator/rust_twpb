#[test]
fn test_ints() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::sint32(&mut buffer.as_mut(), &-300).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(buffer[0..bytes_written], [0xD7, 4]);
    let result = ::twpb::decoder::sint32(buffer.into_iter(), "").unwrap();
    assert_eq!(result, -300);

    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::sint64(&mut buffer.as_mut(), &-300).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(buffer[0..bytes_written], [0xD7, 4]);
    let result = ::twpb::decoder::sint64(buffer.into_iter(), "").unwrap();
    assert_eq!(result, -300);
}

#[test]
fn test_ints_max() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::sint32(&mut buffer.as_mut(), &(0x3F_FF_FF_FF as i32)).unwrap();
    assert_eq!(bytes_written, 5);
    assert_eq!(buffer[0..bytes_written], [0xFE, 0xFF, 0xFF, 0xFF, 7]);
    let result = ::twpb::decoder::sint32(buffer.into_iter(), "").unwrap();
    assert_eq!(result, 0x3F_FF_FF_FF as i32);
}

#[test]
// LEB128 encodes the value as 7 bit chunks, using the msb as indicator if it is the last encoded byte.
// Verify that a 64 bit number is correctly encoded in the tricky case of the very last bit being set.
fn test_ints_encode_correctly_if_msb_set() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::uint64(&mut buffer.as_mut(), &(0x80_00_00_00_00_00_00_00)).unwrap();
    assert_eq!(bytes_written, 10);
    assert_eq!(buffer[0..bytes_written], [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
    let result = ::twpb::decoder::uint64(buffer.into_iter(), "").unwrap();
    assert_eq!(result, 0x80_00_00_00_00_00_00_00);

    // same but for signed integers (zigzag encoding)
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::sint64(&mut buffer.as_mut(), &(-0x80_00_00_00_00_00_00_00)).unwrap();
    assert_eq!(bytes_written, 10);
    assert_eq!(buffer[0..bytes_written], [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]);
    let result = ::twpb::decoder::sint64(buffer.into_iter(), "").unwrap();
    assert_eq!(result, -0x80_00_00_00_00_00_00_00);
}

#[test]
fn test_ints_overflow() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::sint64(&mut buffer.as_mut(), &(-9223372036854775808 as i64)).unwrap();
    assert_eq!(bytes_written, 10);
    let result = ::twpb::decoder::sint32(buffer.into_iter(), "").unwrap_err();
    assert_eq!(result, ::twpb::decoder::DecodeError::TooLargeVarint);
}