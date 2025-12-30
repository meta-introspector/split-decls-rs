// Generated macro for OptionIter (struct)
macro_rules! Depcrate_sparse_chunk_iterOptionIter {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"OptionIter"}
// Dependencies: {}
# [doc = " An iterator over `Option`s of references to the elements of a `SparseChunk`."] # [doc = ""] # [doc = " Iterates over every index in the `SparseChunk`, from zero to its full capacity,"] # [doc = " returning an `Option<&A>` for each index."] pub struct OptionIter < 'a , A , const N : usize > where BitsImpl < N > : Bits , { pub (crate) index : usize , pub (crate) chunk : & 'a SparseChunk < A , N > , }
};
}
