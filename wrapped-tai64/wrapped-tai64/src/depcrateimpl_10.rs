// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl From < [u8 ; Tai64 :: BYTE_SIZE] > for Tai64 { # [doc = " Parse TAI64 from external representation"] fn from (bytes : [u8 ; Tai64 :: BYTE_SIZE]) -> Self { Tai64 (u64 :: from_be_bytes (bytes)) } }
};
}
