#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
struct SimpleTypes {
    #[twpb(int32,nr=1)]
    int32: i32,
    #[twpb(int64,nr=2)]
    int64: i64,
    #[twpb(uint32,nr=3)]
    uint32: u32,
    #[twpb(uint64,nr=4)]
    uint64: u64,
    // #[twpb(sint32,nr=5)]
    // sint32: i32,
    // #[twpb(sint64,nr=6)]
    // sint64: i64,
    // #[twpb(fixed32,nr=7)]
    // fixed32: u32,
    // #[twpb(fixed64,nr=8)]
    // fixed64: u64,
    // #[twpb(sfixed32,nr=9)]
    // sfixed32: i32,
    // #[twpb(sfixed64,nr=10)]
    // sfixed64: i64,
    // #[twpb(double,nr=11)]
    // double: f64,
    // #[twpb(float,nr=12)]
    // float: f32,
    // #[twpb(bool,nr=13)]
    // boolean: bool,
    #[twpb(string,nr=14)]
    string: heapless::String<10>,
    // #[twpb(bytes,nr=13)]
    // bytes: heapless::Vec<u8, 10>,
}

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
struct RepeatedTypes {
    #[twpb(int32,repeated,nr=1)]
    int32: heapless::Vec<i32, 10>,
//     #[twpb(int64,nr=2)]
//     int64: i64,
//     #[twpb(uint32,nr=3)]
//     uint32: u32,
//     #[twpb(uint64,nr=4)]
//     uint64: u64,
//     #[twpb(sint32,nr=5)]
//     sint32: i32,
//     #[twpb(sint64,nr=6)]
//     sint64: i64,
//     #[twpb(fixed32,nr=7)]
//     fixed32: u32,
//     #[twpb(fixed64,nr=8)]
//     fixed64: u64,
//     #[twpb(sfixed32,nr=9)]
//     sfixed32: i32,
//     #[twpb(sfixed64,nr=10)]
//     sfixed64: i64,
//     #[twpb(double,nr=11)]
//     double: f64,
//     #[twpb(float,nr=12)]
//     float: f32,
//     #[twpb(bool,nr=13)]
//     boolean: bool,
//     #[twpb(string,nr=14)]
//     string: heapless::String<10>,
//     #[twpb(bytes,nr=15)]
//     bytes: heapless::Vec<u8, 10>,
    #[twpb(int32,repeated,nr=16)]
    int32_notpacked: heapless::Vec<i32, 10>,
}

#[test]
fn test_types(){
    let dummydata = include_bytes!("files/bin/python.types.simple.bin");

    let parsed = SimpleTypes::twpb_decode_iter(dummydata.iter()).unwrap();
    let expected = SimpleTypes {
        int32: -69,
        int64: -9223372036854775808,
        uint32: 42,
        uint64: 1,
        string: heapless::String::from("🐉"),
    };
    assert_eq!(parsed, expected);
    println!("{:?}", parsed);
}


#[test]
fn test_types_repeated(){
    let dummydata = include_bytes!("files/bin/python.types.repeated.bin");

    let parsed = RepeatedTypes::twpb_decode_iter(dummydata.iter()).unwrap();
    let expected = RepeatedTypes {
        int32: heapless::Vec::from_slice(&[4, 300]).unwrap(),
        int32_notpacked: heapless::Vec::from_slice(&[4, 300]).unwrap(),
    };
    assert_eq!(parsed, expected);
    println!("{:?}", parsed);
}
