// Generated macro for IterMut (struct)
macro_rules! Depcrate_sparse_chunk_iterIterMut {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator over mutable references to the elements of a `SparseChunk`."] pub struct IterMut < 'a , A , const N : usize > where BitsImpl < N > : Bits , { pub (crate) bitmap : Bitmap < N > , pub (crate) chunk : & 'a mut SparseChunk < A , N > , }
};
}
