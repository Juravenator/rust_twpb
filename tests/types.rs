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
    #[twpb(sint32,nr=5)]
    sint32: i32,
    #[twpb(sint64,nr=6)]
    sint64: i64,
    #[twpb(fixed32,nr=7)]
    fixed32: u32,
    #[twpb(fixed64,nr=8)]
    fixed64: u64,
    #[twpb(sfixed32,nr=9)]
    sfixed32: i32,
    #[twpb(sfixed64,nr=10)]
    sfixed64: i64,
    #[twpb(double,nr=11)]
    double: f64,
    #[twpb(float,nr=12)]
    float: f32,
    #[twpb(bool,nr=13)]
    boolean: bool,
    #[twpb(string,nr=14)]
    string: heapless::String<10>,
    #[twpb(bytes,nr=15)]
    bytes: heapless::Vec<u8, 10>,
}

#[derive(Debug, PartialEq, Default, ::twpb_derive::Message)]
struct RepeatedTypes {
    #[twpb(int32,repeated,nr=1)]
    int32: heapless::Vec<i32, 10>,
    #[twpb(int64,repeated,nr=2)]
    int64: heapless::Vec<i64, 10>,
    #[twpb(uint32,repeated,nr=3)]
    uint32: heapless::Vec<u32, 10>,
    #[twpb(uint64,repeated,nr=4)]
    uint64: heapless::Vec<u64, 10>,
    #[twpb(sint32,repeated,nr=5)]
    sint32: heapless::Vec<i32, 10>,
    #[twpb(sint64,repeated,nr=6)]
    sint64: heapless::Vec<i64, 10>,
    #[twpb(fixed32,repeated,nr=7)]
    fixed32: heapless::Vec<u32, 10>,
    #[twpb(fixed64,repeated,nr=8)]
    fixed64: heapless::Vec<u64, 10>,
    #[twpb(sfixed32,repeated,nr=9)]
    sfixed32: heapless::Vec<i32, 10>,
    #[twpb(sfixed64,repeated,nr=10)]
    sfixed64: heapless::Vec<i64, 10>,
    #[twpb(double,repeated,nr=11)]
    double: heapless::Vec<f64, 10>,
    #[twpb(float,repeated,nr=12)]
    float: heapless::Vec<f32, 10>,
    #[twpb(bool,repeated,nr=13)]
    boolean: heapless::Vec<bool, 10>,
    #[twpb(string,repeated,nr=14)]
    string: heapless::Vec<heapless::String<10>, 10>,
    #[twpb(bytes,repeated,nr=15)]
    bytes: heapless::Vec<heapless::Vec<u8, 10>, 10>,
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
    println!("{:?}", parsed);
}

#[test]
fn test_types_repeated(){
    let dummydata = include_bytes!("files/bin/python.types.repeated.bin");

    let parsed = RepeatedTypes::twpb_decode_iter(dummydata.iter()).unwrap();
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
    println!("{:?}", parsed);
}
