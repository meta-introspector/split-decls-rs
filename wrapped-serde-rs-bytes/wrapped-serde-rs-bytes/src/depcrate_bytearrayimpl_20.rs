// Generated macro for impl_20 (impl)
macro_rules! Depcrate_bytearrayimpl_20 {
() => {
// Module: crate::bytearray
// Provides: {"impl_20"}
// Dependencies: {}
impl < const N : usize > Deref for ByteArray < N > { type Target = [u8 ; N] ; fn deref (& self) -> & Self :: Target { & self . bytes } }
};
}
