#[test]
fn test_ints() {
    let mut buffer = Vec::<u8>::new();
    let bytes_written = ::twpb::encoder::sint32(&mut buffer, -300).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(buffer, [0xD7, 4]);
    let result = ::twpb::decoder::sint32(buffer.iter(), "").unwrap();
    assert_eq!(result, -300);

    let mut buffer = Vec::<u8>::new();
    let bytes_written = ::twpb::encoder::sint64(&mut buffer, -300).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(buffer, [0xD7, 4]);
    let result = ::twpb::decoder::sint64(buffer.iter(), "").unwrap();
    assert_eq!(result, -300);
}

#[test]
fn test_ints_max() {
    let mut buffer = Vec::<u8>::new();
    let bytes_written = ::twpb::encoder::sint32(&mut buffer, 0x3F_FF_FF_FF as i32).unwrap();
    assert_eq!(bytes_written, 5);
    assert_eq!(buffer, [0xFE, 0xFF, 0xFF, 0xFF, 7]);
    let result = ::twpb::decoder::sint32(buffer.iter(), "").unwrap();
    assert_eq!(result, 0x3F_FF_FF_FF as i32);
}