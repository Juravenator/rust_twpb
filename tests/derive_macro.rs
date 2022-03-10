#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
struct Simple{
    #[twpb(string,nr=1)]
    serial: heapless::String<10>,
    #[twpb(string,nr=2)]
    firmware_version: heapless::String<10>,
    #[twpb(string,nr=3)]
    vendor: heapless::String<10>,
    #[twpb(string,nr="4")]
    product: heapless::String<10>,
}

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
struct Message {
    #[twpb(oneof,nr="1-3")]
    content: ::core::option::Option<message::Content>,
}

mod message {
    #[derive(PartialEq, Debug, ::twpb_derive::Enum)]
    pub enum Content {
        #[twpb(message,nr=1)]
        ss(super::Simple),
        #[twpb(string,nr=3)]
        test(heapless::String<10>),
    }
}

#[test]
fn test_simple(){
    // let dummydata = [0 as u8,0,1];
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

    let parsed = Message::twpb_decode_iter(dummydata.iter()).unwrap();
    let expected = Message {
        content: Some(message::Content::test(heapless::String::from("teststr"))),
    };
    assert_eq!(parsed, expected);
    println!("{:?}", parsed);
}

#[test]
fn test_oneof_embedded(){
    let dummydata = include_bytes!("files/bin/python.oneof.embedded.bin");

    let parsed = Message::twpb_decode_iter(dummydata.iter()).unwrap();
    let expected = Message {
        content: Some(message::Content::ss(Simple{
            serial: heapless::String::from("serial"),
            firmware_version: heapless::String::from("firmware"),
            vendor: heapless::String::from("vendor"),
            product: heapless::String::from("product"),
        })),
    };
    assert_eq!(parsed, expected);
    println!("{:?}", parsed);
}
