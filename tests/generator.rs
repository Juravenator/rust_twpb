mod types;

use std::fs;

use types::*;

fn assert_same_as_python<T: twpb::MessageEncoder>(name: &str, thing: T) {
    let (encoded, python_encoded) = assert_kinda_same_as_python(name, thing);
    assert_eq!(
        python_encoded, encoded,
        "{} is not equal in twpb and python encoding implementation",
        name
    );
}

fn assert_kinda_same_as_python<T: twpb::MessageEncoder>(
    name: &str,
    thing: T,
) -> (Vec<u8>, Vec<u8>) {
    let encoded = write_msg(name, thing);
    let python_encoded = fs::read(format!("tests/files/bin/python.{}.bin", name))
        .expect(&format!("cannot read python encoded variant of {}", name));
    assert_eq!(
        python_encoded.len(),
        encoded.len(),
        "size of {} is not equal between twpb and python encoding implementation",
        name
    );
    (encoded, python_encoded)
}

fn write_msg<T: twpb::MessageEncoder>(name: &str, thing: T) -> Vec<u8> {
    let mut bytes = [0x0; 1000];
    let len = thing.twpb_encode(&mut bytes.as_mut()).unwrap();
    fs::write(format!("tests/files/bin/twpb.{}.bin", name), &bytes[0..len])
        .expect("Unable to write file");
    bytes[0..len].to_vec()
}

#[test]
fn generate_python_test_bin_files() {
    assert_same_as_python(
        "simple",
        Simple {
            serial: heapless::String::try_from("serial").unwrap(),
            firmware_version: heapless::String::try_from("firmware").unwrap(),
            vendor: heapless::String::try_from("vendor").unwrap(),
            product: heapless::String::try_from("product").unwrap(),
        },
    );

    assert_same_as_python(
        "oneof.simple",
        Embedded {
            content: Some(embedded::Content::Test(
                heapless::String::try_from("teststr").unwrap(),
            )),
            something_else: heapless::String::new(),
        },
    );

    assert_same_as_python(
        "oneof.embedded",
        Embedded {
            content: Some(embedded::Content::Ss(Simple {
                serial: heapless::String::try_from("serial").unwrap(),
                firmware_version: heapless::String::try_from("firmware").unwrap(),
                vendor: heapless::String::try_from("vendor").unwrap(),
                product: heapless::String::try_from("product").unwrap(),
            })),
            something_else: heapless::String::try_from("something else").unwrap(),
        },
    );

    assert_same_as_python(
        "api.getInfo",
        APIMessage {
            content: Some(apimessage::Content::V1Request(v1::Request {
                request: Some(v1::request::Request::GetInfo(v1::EmptyRequest {})),
            })),
        },
    );

    assert_same_as_python(
        "api.getOtherThing",
        APIMessage {
            content: Some(apimessage::Content::V1Request(v1::Request {
                request: Some(v1::request::Request::GetOtherThing(v1::EmptyRequest {})),
            })),
        },
    );

    assert_same_as_python(
        "types.simple",
        SimpleTypes {
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
            string: heapless::String::try_from("🐉").unwrap(),
            bytes: heapless::Vec::from_slice(&['A' as u8, 'S' as u8, 'D' as u8, 'F' as u8])
                .unwrap(),
        },
    );

    assert_kinda_same_as_python(
        "types.repeated",
        RepeatedTypes {
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
            string: heapless::Vec::from_slice(&[
                heapless::String::try_from("🐉").unwrap(),
                heapless::String::try_from("अरे").unwrap(),
            ])
            .unwrap(),
            bytes: heapless::Vec::from_slice(&[
                heapless::Vec::from_slice(&['A' as u8, 'S' as u8, 'D' as u8, 'F' as u8]).unwrap(),
                heapless::Vec::from_slice(&['A' as u8, 'B' as u8, 'C' as u8, 'D' as u8]).unwrap(),
            ])
            .unwrap(),
        },
    );
}
