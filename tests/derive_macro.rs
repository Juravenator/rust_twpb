mod types;

use types::{Simple, Embedded, embedded};

#[test]
fn test_simple(){
    let dummydata = include_bytes!("files/bin/python.simple.bin");

    let parsed = Simple::twpb_decode_iter(dummydata.iter()).unwrap();
    let expected = Simple {
        serial: heapless::String::from("serial"),
        firmware_version: heapless::String::from("firmware"),
        vendor: heapless::String::from("vendor"),
        product: heapless::String::from("product"),
    };
    assert_eq!(parsed, expected);
    println!("{:?}", parsed);
}

#[test]
fn test_oneof_simple(){
    let dummydata = include_bytes!("files/bin/python.oneof.simple.bin");

    let parsed = Embedded::twpb_decode_iter(dummydata.iter()).unwrap();
    let expected = Embedded {
        content: Some(embedded::Content::Test(heapless::String::from("teststr"))),
        something_else: heapless::String::from(""),
    };
    assert_eq!(parsed, expected);
    println!("{:?}", parsed);
}

#[test]
fn test_oneof_embedded(){
    let dummydata = include_bytes!("files/bin/python.oneof.embedded.bin");

    let parsed = Embedded::twpb_decode_iter(dummydata.iter()).unwrap();
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
