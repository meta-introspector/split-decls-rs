// Generated macro for impl_163 (impl)
macro_rules! Depcrate_sparse_chunkimpl_163 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_163"}
// Dependencies: {}
impl < A , const N : usize > Debug for SparseChunk < A , N > where A : Debug , BitsImpl < N > : Bits , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("SparseChunk") ? ; f . debug_map () . entries (self . entries ()) . finish () } }
};
}
