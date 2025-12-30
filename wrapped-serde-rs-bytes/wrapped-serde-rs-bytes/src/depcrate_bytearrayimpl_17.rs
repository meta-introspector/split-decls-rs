// Generated macro for impl_17 (impl)
macro_rules! Depcrate_bytearrayimpl_17 {
() => {
// Module: crate::bytearray
// Provides: {"impl_17"}
// Dependencies: {}
impl < const N : usize > AsMut < [u8 ; N] > for ByteArray < N > { fn as_mut (& mut self) -> & mut [u8 ; N] { & mut self . bytes } }
};
}
