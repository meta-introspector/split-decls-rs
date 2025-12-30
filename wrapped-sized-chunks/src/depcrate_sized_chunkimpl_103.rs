// Generated macro for impl_103 (impl)
macro_rules! Depcrate_sized_chunkimpl_103 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_103"}
// Dependencies: {}
impl < A , const N : usize > BorrowMut < [A] > for Chunk < A , N > { fn borrow_mut (& mut self) -> & mut [A] { self . as_mut_slice () } }
};
}
