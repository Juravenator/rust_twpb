mod types;

use types::{APIMessage, apimessage, v1};
use twpb::{MessageDecoder};

#[test] // We can successfully decode a getInfo API request
fn test_get_info() {
    let dummydata = include_bytes!("files/bin/python.api.getInfo.bin");
    let message = APIMessage::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();

    // show-off version
    match message.content {
        Some(apimessage::Content::V1Request(v1::Request{request: Some(v1::request::Request::GetInfo(_))})) => (),
        _ => panic!("unexpected api message content"),
    }

    // what you're likely gonna actually use
    match message.content {
        Some(apimessage::Content::V1Request(message)) => match message.request {
            Some(v1::request::Request::GetInfo(_)) => (),
            _ => panic!("wrong request type"),
        },
        _ => panic!("unexpected api message content"),
    }
}

#[test] // A getInfo API request does not decode as something else
#[should_panic(expected = "wrong request type")]
fn test_get_info_2() {
    let dummydata = include_bytes!("files/bin/python.api.getInfo.bin");
    let message = APIMessage::twpb_decode_iter(dummydata.iter().map(|x| *x)).unwrap();

    match message.content {
        Some(apimessage::Content::V1Request(message)) => match message.request {
            Some(v1::request::Request::GetOtherThing(_)) => (),
            _ => panic!("wrong request type"),
        },
        _ => panic!("unexpected api message content"),
    }
}