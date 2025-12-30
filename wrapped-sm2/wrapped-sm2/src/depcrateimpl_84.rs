// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl FieldBytesEncoding < Sm2 > for U256 { fn decode_field_bytes (field_bytes : & FieldBytes) -> Self { U256 :: from_be_byte_array (* field_bytes) } fn encode_field_bytes (& self) -> FieldBytes { self . to_be_byte_array () } }
};
}
