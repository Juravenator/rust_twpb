use heapless::String;

#[test]
fn test_strings() {
    let mut buffer = Vec::<u8>::new();
    let bytes_written = ::twpb::encoder::string(&mut buffer, "stay hungry, stay foolish").unwrap();
    assert_eq!(bytes_written, 25);
    let string: String<25> = ::twpb::decoder::string(buffer.iter(), "").unwrap();
    assert_eq!(string, "stay hungry, stay foolish");
}

#[test]
fn test_utf8_strings() {
    let mut buffer = Vec::<u8>::new();
    let bytes_written = ::twpb::encoder::string(&mut buffer, "🐉🐉🐉🐉").unwrap();
    assert_eq!(bytes_written, 16);
    let string: String<20> = ::twpb::decoder::string(buffer.iter(), "").unwrap();
    assert_eq!(string, "🐉🐉🐉🐉");
}