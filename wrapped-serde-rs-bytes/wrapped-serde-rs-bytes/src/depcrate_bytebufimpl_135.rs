// Generated macro for impl_135 (impl)
macro_rules! Depcrate_bytebufimpl_135 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_135"}
// Dependencies: {}
impl BorrowMut < Bytes > for ByteBuf { fn borrow_mut (& mut self) -> & mut Bytes { unsafe { & mut * (& mut self . bytes as & mut [u8] as * mut [u8] as * mut Bytes) } } }
};
}
