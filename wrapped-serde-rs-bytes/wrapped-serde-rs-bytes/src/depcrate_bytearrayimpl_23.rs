// Generated macro for impl_23 (impl)
macro_rules! Depcrate_bytearrayimpl_23 {
() => {
// Module: crate::bytearray
// Provides: {"impl_23"}
// Dependencies: {}
impl < const N : usize > BorrowMut < Bytes > for ByteArray < N > { fn borrow_mut (& mut self) -> & mut Bytes { unsafe { & mut * (& mut self . bytes as & mut [u8] as * mut [u8] as * mut Bytes) } } }
};
}
