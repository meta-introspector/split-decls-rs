// Generated macro for Drain (struct)
macro_rules! Depcrate_sparse_chunk_iterDrain {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the elements of a `SparseChunk`."] # [doc = ""] # [doc = " \"Draining\" means that as the iterator yields each element, it's removed from"] # [doc = " the `SparseChunk`. When the iterator terminates, the chunk will be empty."] pub struct Drain < A , const N : usize > where BitsImpl < N > : Bits , { pub (crate) chunk : SparseChunk < A , N > , }
};
}
