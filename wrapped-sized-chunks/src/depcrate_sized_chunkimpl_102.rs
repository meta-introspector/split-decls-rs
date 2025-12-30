// Generated macro for impl_102 (impl)
macro_rules! Depcrate_sized_chunkimpl_102 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_102"}
// Dependencies: {}
impl < A , const N : usize > Borrow < [A] > for Chunk < A , N > { fn borrow (& self) -> & [A] { self . as_slice () } }
};
}
