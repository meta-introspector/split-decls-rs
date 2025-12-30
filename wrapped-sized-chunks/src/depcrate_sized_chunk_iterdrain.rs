// Generated macro for Drain (struct)
macro_rules! Depcrate_sized_chunk_iterDrain {
() => {
// Module: crate::sized_chunk::iter
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the elements of a `Chunk`."] # [doc = ""] # [doc = " \"Draining\" means that as the iterator yields each element, it's removed from"] # [doc = " the `Chunk`. When the iterator terminates, the chunk will be empty. This is"] # [doc = " different from the consuming iterator `Iter` in that `Iter` will take"] # [doc = " ownership of the `Chunk` and discard it when you're done iterating, while"] # [doc = " `Drain` leaves you still owning the drained `Chunk`."] pub struct Drain < 'a , A , const N : usize > { pub (crate) chunk : & 'a mut Chunk < A , N > , }
};
}
