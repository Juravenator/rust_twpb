mod types;

use types::{SimpleTypes, RepeatedTypes};
use twpb::{MessageEncoder, MessageDecoder};

#[test]
fn test_types(){
    // Test if binary data from another library (in this case Python)
    // is correctly parsed by us
    let dummydata = include_bytes!("files/bin/python.types.simple.bin");

    let parsed = SimpleTypes::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();
    let expected = SimpleTypes {
        int32: -69,
        int64: -9223372036854775808,
        uint32: 42,
        uint64: 1,
        sint32: -69,
        sint64: 69,
        fixed32: u32::MAX,
        fixed64: 42,
        sfixed32: i32::MAX,
        sfixed64: i64::MIN + 1,
        double: 1.0,
        float: 3.1415926535,
        boolean: true,
        string: heapless::String::from("🐉"),
        bytes: heapless::Vec::from_slice(&['A' as u8, 'S' as u8, 'D' as u8, 'F' as u8]).unwrap(),
    };
    assert_eq!(parsed, expected);
    // Now that we verified decoding works, encode and decode some data and check if it matches
    // Note that we can't encode and check for binary match with external data. Order is not guaranteed.
    let mut dummydata = [0x0; 1000];

    let bytes_written = expected.twpb_encode(&mut dummydata.as_mut()).unwrap();
    let expected_bytes = [
        0x08, 0xBB, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x01, 0x10, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x01, 0x18, 0x2A, 0x20, 0x01, 0x28, 0x89, 0x01, 0x30,
        0x8A, 0x01, 0x3D, 0xFF, 0xFF, 0xFF, 0xFF, 0x41, 0x2A, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4D, 0xFF, 0xFF, 0xFF,
        0x7F, 0x51, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
        0x59, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, 0x65,
        0xDB, 0x0F, 0x49, 0x40, 0x68, 0x01, 0x72, 0x04, 0xF0, 0x9F,
        0x90, 0x89, 0x7A, 0x04, 0x41, 0x53, 0x44, 0x46,
    ];
    assert_eq!(bytes_written, expected_bytes.len());
    assert_eq!(dummydata[0..bytes_written], expected_bytes);
    let parsed = SimpleTypes::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_types_repeated_decode(){
    let dummydata = include_bytes!("files/bin/python.types.repeated.bin");

    let parsed = RepeatedTypes::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();
    let expected = RepeatedTypes {
        int32: heapless::Vec::from_slice(&[4, -300]).unwrap(),
        int32_notpacked: heapless::Vec::from_slice(&[4, -300]).unwrap(),
        int64: heapless::Vec::from_slice(&[69, -69]).unwrap(),
        uint32: heapless::Vec::from_slice(&[42, 420]).unwrap(),
        uint64: heapless::Vec::from_slice(&[42, 420]).unwrap(),
        sint32: heapless::Vec::from_slice(&[-69, 69]).unwrap(),
        sint64: heapless::Vec::from_slice(&[69, -69]).unwrap(),
        fixed32: heapless::Vec::from_slice(&[u32::MAX, 1]).unwrap(),
        fixed64: heapless::Vec::from_slice(&[42, u64::MAX]).unwrap(),
        sfixed32: heapless::Vec::from_slice(&[i32::MAX, -69]).unwrap(),
        sfixed64: heapless::Vec::from_slice(&[42, -42]).unwrap(),
        double: heapless::Vec::from_slice(&[1.0, 3.1415926535]).unwrap(),
        float: heapless::Vec::from_slice(&[3.1415926535, 1.0]).unwrap(),
        boolean: heapless::Vec::from_slice(&[true, false]).unwrap(),
        string: heapless::Vec::from_slice(&[heapless::String::from("🐉"), heapless::String::from("अरे")]).unwrap(),
        bytes: heapless::Vec::from_slice(&[
            heapless::Vec::from_slice(&['A' as u8, 'S' as u8, 'D' as u8, 'F' as u8]).unwrap(),
            heapless::Vec::from_slice(&['A' as u8, 'B' as u8, 'C' as u8, 'D' as u8]).unwrap()
        ]).unwrap(),
    };
    assert_eq!(parsed, expected);

    // Now that we verified decoding works, encode and decode some data and check if it matches
    // Note that we can't encode and check for binary match with external data. Order is not guaranteed.
    let mut dummydata = [0x0; 1000];
    let bytes_written = expected.twpb_encode(&mut dummydata.as_mut()).unwrap();
    let expected_bytes = [
        0x08, 0x04, 0x08, 0xD4, 0xFD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0x01, 0x10, 0x45, 0x10, 0xBB, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x18, 0x2A, 0x18, 0xA4,
        0x03, 0x20, 0x2A, 0x20, 0xA4, 0x03, 0x28, 0x89, 0x01, 0x28,
        0x8A, 0x01, 0x30, 0x8A, 0x01, 0x30, 0x89, 0x01, 0x3D, 0xFF,
        0xFF, 0xFF, 0xFF, 0x3D, 0x01, 0x00, 0x00, 0x00, 0x41, 0x2A,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x4D, 0xFF, 0xFF, 0xFF,
        0x7F, 0x4D, 0xBB, 0xFF, 0xFF, 0xFF, 0x51, 0x2A, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0xD6, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x59, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xF0, 0x3F, 0x59, 0x44, 0x17, 0x41, 0x54, 0xFB, 0x21,
        0x09, 0x40, 0x65, 0xDB, 0x0F, 0x49, 0x40, 0x65, 0x00, 0x00,
        0x80, 0x3F, 0x68, 0x01, 0x68, 0x00, 0x72, 0x04, 0xF0, 0x9F,
        0x90, 0x89, 0x72, 0x09, 0xE0, 0xA4, 0x85, 0xE0, 0xA4, 0xB0,
        0xE0, 0xA5, 0x87, 0x7A, 0x04, 0x41, 0x53, 0x44, 0x46, 0x7A,
        0x04, 0x41, 0x42, 0x43, 0x44, 0x80, 0x01, 0x04, 0x80, 0x01,
        0xD4, 0xFD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01,
    ];
    assert_eq!(bytes_written, expected_bytes.len());
    assert_eq!(dummydata[0..bytes_written], expected_bytes);
    let parsed = RepeatedTypes::twpb_decode(&dummydata).unwrap();
    assert_eq!(parsed, expected);
}
