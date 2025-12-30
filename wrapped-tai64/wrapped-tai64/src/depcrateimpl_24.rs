// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl From < Tai64N > for [u8 ; Tai64N :: BYTE_SIZE] { # [doc = " Serialize TAI64 to external representation"] fn from (tai : Tai64N) -> [u8 ; Tai64N :: BYTE_SIZE] { let mut result = [0u8 ; Tai64N :: BYTE_SIZE] ; result [.. Tai64 :: BYTE_SIZE] . copy_from_slice (& tai . 0 . to_bytes ()) ; result [Tai64 :: BYTE_SIZE ..] . copy_from_slice (& tai . 1 . to_be_bytes ()) ; result } }
};
}
