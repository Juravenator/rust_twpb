mod types;

use types::RepeatedTypes;

#[test]
// A packed repeated field containing zero elements does not
// appear in the encoded message.
fn test_repeated_encode_zero_elements(){
    let source = RepeatedTypes {
        int32: heapless::Vec::new(),
        int32_notpacked: heapless::Vec::new(),
        int64: heapless::Vec::new(),
        uint32: heapless::Vec::new(),
        uint64: heapless::Vec::new(),
        sint32: heapless::Vec::new(),
        sint64: heapless::Vec::new(),
        fixed32: heapless::Vec::new(),
        fixed64: heapless::Vec::new(),
        sfixed32: heapless::Vec::new(),
        sfixed64: heapless::Vec::new(),
        double: heapless::Vec::new(),
        float: heapless::Vec::new(),
        boolean: heapless::Vec::new(),
        string: heapless::Vec::new(),
        bytes: heapless::Vec::new(),
    };
    let mut dummydata = Vec::<u8>::new();
    let bytes_written = source.twpb_encode(&mut dummydata).unwrap();
    assert_eq!(bytes_written, 0);
    assert_eq!(dummydata, []);
}

#[test]
// Yes other tests cover this, but let's add one field with actual data
// to ensure we're still actually encoding something.
fn test_repeated_encode_one_element(){
    let source = RepeatedTypes {
        int32: heapless::Vec::new(),
        int32_notpacked: heapless::Vec::new(),
        int64: heapless::Vec::new(),
        uint32: heapless::Vec::new(),
        uint64: heapless::Vec::new(),
        sint32: heapless::Vec::new(),
        sint64: heapless::Vec::new(),
        fixed32: heapless::Vec::new(),
        fixed64: heapless::Vec::new(),
        sfixed32: heapless::Vec::new(),
        sfixed64: heapless::Vec::new(),
        double: heapless::Vec::new(),
        float: heapless::Vec::new(),
        boolean: heapless::Vec::from_slice(&[true, false]).unwrap(),
        string: heapless::Vec::new(),
        bytes: heapless::Vec::new(),
    };
    let mut dummydata = Vec::<u8>::new();
    let bytes_written = source.twpb_encode(&mut dummydata).unwrap();
    assert_eq!(bytes_written, 2);
    assert_eq!(dummydata, [0x68, 0x01, 0x68, 0x00]);
}