mod types;

use std::fs;

use types::*;
use twpb::traits::MessageEncoder;

#[test]
fn generate_python_test_bin_files(){
    let mut bytes = [0x0; 1000];
    let len = Simple {
        serial: heapless::String::from("serial"),
        firmware_version: heapless::String::from("firmware"),
        vendor: heapless::String::from("vendor"),
        product: heapless::String::from("product"),
    }.twpb_encode(&mut bytes.as_mut()).unwrap();
    fs::write("tests/files/bin/twpb.oneof.simple.bin", &bytes[0..len]).expect("Unable to write file");

    let mut bytes = [0x0; 1000];
    let len = Embedded {
        content: Some(embedded::Content::Test(heapless::String::from("teststr"))),
        something_else: heapless::String::from(""),
    }.twpb_encode(&mut bytes.as_mut()).unwrap();
    fs::write("tests/files/bin/twpb.oneof.simple.bin", &bytes[0..len]).expect("Unable to write file");

    let mut bytes = [0x0; 1000];
    let len = Embedded {
        content: Some(embedded::Content::Ss(Simple{
            serial: heapless::String::from("serial"),
            firmware_version: heapless::String::from("firmware"),
            vendor: heapless::String::from("vendor"),
            product: heapless::String::from("product"),
        })),
        something_else: heapless::String::from("something else"),
    }.twpb_encode(&mut bytes.as_mut()).unwrap();
    fs::write("tests/files/bin/twpb.oneof.embedded.bin", &bytes[0..len]).expect("Unable to write file");

    let mut bytes = [0x0; 1000];
    let len = APIMessage {
        content: Some(apimessage::Content::V1Request(v1::Request{
            request: Some(v1::request::Request::GetInfo(v1::EmptyRequest{}))
        }))
    }.twpb_encode(&mut bytes.as_mut()).unwrap();
    fs::write("tests/files/bin/twpb.api.getInfo.bin", &bytes[0..len]).expect("Unable to write file");

    let mut bytes = [0x0; 1000];
    let len = SimpleTypes {
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
    }.twpb_encode(&mut bytes.as_mut()).unwrap();
    fs::write("tests/files/bin/twpb.types.simple.bin", &bytes[0..len]).expect("Unable to write file");

    let mut bytes = [0x0; 1000];
    let len = RepeatedTypes {
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
    }.twpb_encode(&mut bytes.as_mut()).unwrap();
    fs::write("tests/files/bin/twpb.types.repeated.bin", &bytes[0..len]).expect("Unable to write file");
}