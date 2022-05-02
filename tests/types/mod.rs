// All Rust equivalents of the protobuf files in /tests/files/proto are kept here.
// Used for testing, we often use only a subset of these types per test.
// Hence, lots of dead code warnings, disable at crate level.
#![allow(dead_code)]

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
pub struct SimpleTypes {
    #[twpb(int32,nr=1)]
    pub int32: i32,
    #[twpb(int64,nr=2)]
    pub int64: i64,
    #[twpb(uint32,nr=3)]
    pub uint32: u32,
    #[twpb(uint64,nr=4)]
    pub uint64: u64,
    #[twpb(sint32,nr=5)]
    pub sint32: i32,
    #[twpb(sint64,nr=6)]
    pub sint64: i64,
    #[twpb(fixed32,nr=7)]
    pub fixed32: u32,
    #[twpb(fixed64,nr=8)]
    pub fixed64: u64,
    #[twpb(sfixed32,nr=9)]
    pub sfixed32: i32,
    #[twpb(sfixed64,nr=10)]
    pub sfixed64: i64,
    #[twpb(double,nr=11)]
    pub double: f64,
    #[twpb(float,nr=12)]
    pub float: f32,
    #[twpb(bool,nr=13)]
    pub boolean: bool,
    #[twpb(string,nr=14)]
    pub string: heapless::String<10>,
    #[twpb(bytes,nr=15)]
    pub bytes: heapless::Vec<u8, 10>,
}

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
pub struct RepeatedTypes {
    #[twpb(int32,repeated,nr=1)]
    pub int32: heapless::Vec<i32, 10>,
    #[twpb(int64,repeated,nr=2)]
    pub int64: heapless::Vec<i64, 10>,
    #[twpb(uint32,repeated,nr=3)]
    pub uint32: heapless::Vec<u32, 10>,
    #[twpb(uint64,repeated,nr=4)]
    pub uint64: heapless::Vec<u64, 10>,
    #[twpb(sint32,repeated,nr=5)]
    pub sint32: heapless::Vec<i32, 10>,
    #[twpb(sint64,repeated,nr=6)]
    pub sint64: heapless::Vec<i64, 10>,
    #[twpb(fixed32,repeated,nr=7)]
    pub fixed32: heapless::Vec<u32, 10>,
    #[twpb(fixed64,repeated,nr=8)]
    pub fixed64: heapless::Vec<u64, 10>,
    #[twpb(sfixed32,repeated,nr=9)]
    pub sfixed32: heapless::Vec<i32, 10>,
    #[twpb(sfixed64,repeated,nr=10)]
    pub sfixed64: heapless::Vec<i64, 10>,
    #[twpb(double,repeated,nr=11)]
    pub double: heapless::Vec<f64, 10>,
    #[twpb(float,repeated,nr=12)]
    pub float: heapless::Vec<f32, 10>,
    #[twpb(bool,repeated,nr=13)]
    pub boolean: heapless::Vec<bool, 10>,
    #[twpb(string,repeated,nr=14)]
    pub string: heapless::Vec<heapless::String<10>, 10>,
    #[twpb(bytes,repeated,nr=15)]
    pub bytes: heapless::Vec<heapless::Vec<u8, 10>, 10>,
    #[twpb(int32,repeated,nr=16)]
    pub int32_notpacked: heapless::Vec<i32, 10>,
}

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
pub struct Simple{
    #[twpb(string,nr=1)]
    pub serial: heapless::String<10>,
    #[twpb(string,nr=2)]
    pub firmware_version: heapless::String<10>,
    #[twpb(string,nr=3)]
    pub vendor: heapless::String<10>,
    #[twpb(string,nr="4")]
    pub product: heapless::String<10>,
}

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
pub struct Embedded {
    #[twpb(oneof,nr="1-3")]
    pub content: ::core::option::Option<embedded::Content>,
    // To test that our oneof (which contains a Message object)
    // consumes only the bytes its supposed to, we have an extra
    // value after its bytestream.
    #[twpb(string,nr=5)]
    pub something_else: heapless::String<20>,
}

pub mod embedded {
    #[derive(PartialEq, Debug, ::twpb_derive::Enum)]
    pub enum Content {
        #[twpb(message,nr=1)]
        Ss(super::Simple),
        #[twpb(string,nr=3)]
        Test(heapless::String<10>),
    }
}

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
pub struct APIMessage {
    #[twpb(oneof,nr="1-2")]
    pub content: ::core::option::Option<apimessage::Content>,
}

pub mod apimessage {
    #[derive(PartialEq, Debug, ::twpb_derive::Enum)]
    pub enum Content {
        #[twpb(message,nr=1)]
        V1Request(super::v1::Request),
        #[twpb(message,nr=2)]
        V1Response(super::v1::Response),
    }
}

pub mod v1 {
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
            GetInfo(super::EmptyRequest),
            #[twpb(message,nr=2)]
            GetOtherThing(super::EmptyRequest),
        }
    }

    pub mod response {
        #[derive(PartialEq, Debug, ::twpb_derive::Enum)]
        pub enum Response {
            #[twpb(message,nr=1)]
            Info(super::SysInfo),
            #[twpb(message,nr=2)]
            OtherThing(super::OtherThing),
        }
    }
}