#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
struct Message {
    #[twpb(oneof,nr="1-2")]
    content: ::core::option::Option<message::Content>,
}

mod message {
    #[derive(PartialEq, Debug, ::twpb_derive::Enum)]
    pub enum Content {
        #[twpb(message,nr=1)]
        v1_request(super::v1::Request),
        #[twpb(message,nr=2)]
        v1_response(super::v1::Response),
    }
}

mod v1 {
    #[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
    pub struct EmptyRequest {}

    #[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
    pub struct Request {
        #[twpb(oneof,nr="1-2")]
        pub request: ::core::option::Option<request::Request>,
    }

    #[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
    pub struct Response {
        #[twpb(oneof,nr="1-2")]
        pub response: ::core::option::Option<response::Response>,
    }

    #[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
    pub struct SysInfo{
        #[twpb(string,nr=1)]
        pub serial: heapless::String<10>,
        #[twpb(string,nr=2)]
        pub firmware_version: heapless::String<10>,
        #[twpb(string,nr=3)]
        pub vendor: heapless::String<10>,
        #[twpb(string,nr=4)]
        pub product: heapless::String<10>,
    }

    #[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
    pub struct OtherThing{
        #[twpb(string,nr=1)]
        pub other: heapless::String<10>,
    }

    pub mod request {
        #[derive(PartialEq, Debug, ::twpb_derive::Enum)]
        pub enum Request {
            #[twpb(message,nr=1)]
            getInfo(super::EmptyRequest),
            #[twpb(message,nr=2)]
            getOtherThing(super::EmptyRequest),
        }
    }

    pub mod response {
        #[derive(PartialEq, Debug, ::twpb_derive::Enum)]
        pub enum Response {
            #[twpb(message,nr=1)]
            info(super::SysInfo),
            #[twpb(message,nr=2)]
            other_thing(super::OtherThing),
        }
    }
}


#[test] // We can successfully decode a getInfo API request
fn test_get_info(){
    let dummydata = include_bytes!("files/bin/python.api.getInfo.bin");
    let message = Message::twpb_decode_iter(dummydata.iter()).unwrap();

    // show-off version
    match message.content {
        Some(message::Content::v1_request(v1::Request{request: Some(v1::request::Request::getInfo(_))})) => (),
        _ => panic!("unexpected api message content"),
    }

    // what you're likely gonna actually use
    match message.content {
        Some(message::Content::v1_request(message)) => match message.request {
            Some(v1::request::Request::getInfo(_)) => (),
            _ => panic!("wrong request type"),
        },
        _ => panic!("unexpected api message content"),
    }
}

#[test] // A getInfo API request does not decode as something else
#[should_panic(expected = "wrong request type")]
fn test_get_info_2(){
    let dummydata = include_bytes!("files/bin/python.api.getInfo.bin");
    let message = Message::twpb_decode_iter(dummydata.iter()).unwrap();

    match message.content {
        Some(message::Content::v1_request(message)) => match message.request {
            Some(v1::request::Request::getOtherThing(_)) => (),
            _ => panic!("wrong request type"),
        },
        _ => panic!("unexpected api message content"),
    }
}