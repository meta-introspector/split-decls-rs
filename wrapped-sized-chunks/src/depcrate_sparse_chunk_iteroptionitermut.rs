// Generated macro for OptionIterMut (struct)
macro_rules! Depcrate_sparse_chunk_iterOptionIterMut {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"OptionIterMut"}
// Dependencies: {}
# [doc = " An iterator over `Option`s of mutable references to the elements of a `SparseChunk`."] # [doc = ""] # [doc = " Iterates over every index in the `SparseChunk`, from zero to its full capacity,"] # [doc = " returning an `Option<&mut A>` for each index."] pub struct OptionIterMut < 'a , A , const N : usize > where BitsImpl < N > : Bits , { pub (crate) index : usize , pub (crate) chunk : & 'a mut SparseChunk < A , N > , }
};
}
