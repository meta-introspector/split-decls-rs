// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl From < Tai64 > for [u8 ; 8] { # [doc = " Serialize TAI64 to external representation"] fn from (tai : Tai64) -> [u8 ; 8] { tai . 0 . to_be_bytes () } }
};
}
