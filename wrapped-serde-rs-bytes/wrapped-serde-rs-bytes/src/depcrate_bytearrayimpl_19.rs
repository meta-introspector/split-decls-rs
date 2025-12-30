// Generated macro for impl_19 (impl)
macro_rules! Depcrate_bytearrayimpl_19 {
() => {
// Module: crate::bytearray
// Provides: {"impl_19"}
// Dependencies: {}
impl < const N : usize > BorrowMut < [u8 ; N] > for ByteArray < N > { fn borrow_mut (& mut self) -> & mut [u8 ; N] { & mut self . bytes } }
};
}
