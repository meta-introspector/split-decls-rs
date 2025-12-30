// Generated macro for impl_151 (impl)
macro_rules! Depcrate_sparse_chunkimpl_151 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_151"}
// Dependencies: {}
impl < A , const N : usize > Drop for SparseChunk < A , N > where BitsImpl < N > : Bits , { fn drop (& mut self) { if mem :: needs_drop :: < A > () { let bits = self . map ; for index in & bits { unsafe { ptr :: drop_in_place (& mut self . values_mut () [index]) } } } } }
};
}
