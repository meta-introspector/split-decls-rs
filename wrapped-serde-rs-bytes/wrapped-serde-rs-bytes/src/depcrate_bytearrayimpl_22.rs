// Generated macro for impl_22 (impl)
macro_rules! Depcrate_bytearrayimpl_22 {
() => {
// Module: crate::bytearray
// Provides: {"impl_22"}
// Dependencies: {}
impl < const N : usize > Borrow < Bytes > for ByteArray < N > { fn borrow (& self) -> & Bytes { Bytes :: new (& self . bytes) } }
};
}
