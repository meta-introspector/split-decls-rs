// Generated macro for Iter (struct)
macro_rules! Depcrate_sparse_chunk_iterIter {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over references to the elements of a `SparseChunk`."] pub struct Iter < 'a , A , const N : usize > where BitsImpl < N > : Bits , { pub (crate) indices : BitmapIter < 'a , N > , pub (crate) chunk : & 'a SparseChunk < A , N > , }
};
}
