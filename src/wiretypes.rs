// https://developers.google.com/protocol-buffers/docs/encoding#structure
pub mod WireTypes {
    // int32, int64, uint32, uint64, sint32, sint64, bool, enum
    pub const Varint: u8 = 0;
    // fixed64, sfixed64, double
    pub const B64: u8 = 1;
    // string, bytes, embedded messages, packed repeated fields
    pub const LengthDelimited: u8 = 2;
    // groups (deprecated)
    pub const StartGroup: u8 = 3;
    // groups (deprecated)
    pub const EndGroup: u8 = 4;
    // fixed32, sfixed32, float
    pub const B32: u8 = 5;
}