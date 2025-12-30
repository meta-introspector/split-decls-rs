// Generated macro for impl_18 (impl)
macro_rules! Depcrate_bytearrayimpl_18 {
() => {
// Module: crate::bytearray
// Provides: {"impl_18"}
// Dependencies: {}
impl < const N : usize > Borrow < [u8 ; N] > for ByteArray < N > { fn borrow (& self) -> & [u8 ; N] { & self . bytes } }
};
}
