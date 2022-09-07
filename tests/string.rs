use heapless::String;

#[test]
fn test_strings() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::string(&mut buffer.as_mut(), &heapless::String::<25>::from("stay hungry, stay foolish")).unwrap();
    assert_eq!(bytes_written, 26);
    let string: String<25> = ::twpb::decoder::string(&mut buffer.into_iter(), "").unwrap();
    assert_eq!(string, "stay hungry, stay foolish");
}

#[test]
fn test_utf8_strings() {
    let mut buffer = [0x0; 100];
    let bytes_written = ::twpb::encoder::string(&mut buffer.as_mut(), &heapless::String::<16>::from("🐉🐉🐉🐉")).unwrap();
    assert_eq!(bytes_written, 17);
    let string: String<20> = ::twpb::decoder::string(&mut buffer.into_iter(), "").unwrap();
    assert_eq!(string, "🐉🐉🐉🐉");
}