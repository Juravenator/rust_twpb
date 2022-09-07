#[test]
fn test_tags() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::tag(&mut buffer.as_mut(), &30, &5).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(buffer[0..bytes_written], [0xF5, 1]);
    let (field_number, wire_type) = ::twpb::decoder::tag(buffer.into_iter()).unwrap();
    assert_eq!(field_number, 30);
    assert_eq!(wire_type, 5);
}

#[test]
fn test_tags_max() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::tag(&mut buffer.as_mut(), &((u32::MAX << 3) >> 3), &0b0111).unwrap();
    assert_eq!(bytes_written, 5);
    assert_eq!(buffer[0..bytes_written], [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
    let (field_number, wire_type) = ::twpb::decoder::tag(buffer.into_iter()).unwrap();
    assert_eq!(field_number, (u32::MAX << 3) as u32 >> 3);
    assert_eq!(wire_type, 7);
}

#[test]
fn test_tags_overflow() {
    let mut buffer = [0x0; 100];
    let result = ::twpb::encoder::tag(&mut buffer.as_mut(), &u32::MAX, &1).unwrap_err();
    assert_eq!(result, ::twpb::traits::WriterError::BufferOverflow);
    let result = ::twpb::encoder::tag(&mut buffer.as_mut(), &1, &u8::MAX).unwrap_err();
    assert_eq!(result, ::twpb::traits::WriterError::BufferOverflow);
}