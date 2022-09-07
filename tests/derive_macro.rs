mod types;

use types::{Simple, Embedded, embedded};
use twpb::{MessageEncoder, MessageDecoder};

#[test]
fn test_simple(){
    let dummydata = include_bytes!("files/bin/python.simple.bin");
    let expected_len = dummydata.len();

    let parsed = Simple::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();
    let expected = Simple {
        serial: heapless::String::from("serial"),
        firmware_version: heapless::String::from("firmware"),
        vendor: heapless::String::from("vendor"),
        product: heapless::String::from("product"),
    };
    assert_eq!(parsed, expected);

    let mut dummydata = [0x0; 1000];
    let bytes_written = expected.twpb_encode(&mut dummydata.as_mut()).unwrap();
    assert_eq!(bytes_written, expected_len);
    let parsed = Simple::twpb_decode_iter(dummydata[0..bytes_written].iter().map(|x| *x)).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_oneof_simple(){
    let dummydata = include_bytes!("files/bin/python.oneof.simple.bin");
    let expected_len = dummydata.len();

    let parsed = Embedded::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();
    let expected = Embedded {
        content: Some(embedded::Content::Test(heapless::String::from("teststr"))),
        something_else: heapless::String::from(""),
    };
    assert_eq!(parsed, expected);

    let mut dummydata = [0x0; 100];
    let bytes_written = expected.twpb_encode(&mut dummydata.as_mut()).unwrap();
    assert_eq!(bytes_written, expected_len);
    let parsed = Embedded::twpb_decode_iter(dummydata[0..bytes_written].iter().map(|x| *x)).unwrap();
    assert_eq!(parsed, expected);
}

#[test]
fn test_oneof_embedded(){
    let dummydata = include_bytes!("files/bin/python.oneof.embedded.bin");

    let parsed = Embedded::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();
    let expected = Embedded {
        content: Some(embedded::Content::Ss(Simple{
            serial: heapless::String::from("serial"),
            firmware_version: heapless::String::from("firmware"),
            vendor: heapless::String::from("vendor"),
            product: heapless::String::from("product"),
        })),
        something_else: heapless::String::from("something else"),
    };
    assert_eq!(parsed, expected);
    println!("{:?}", parsed);
}
