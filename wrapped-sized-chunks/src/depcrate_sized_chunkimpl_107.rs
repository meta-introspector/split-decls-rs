// Generated macro for impl_107 (impl)
macro_rules! Depcrate_sized_chunkimpl_107 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_107"}
// Dependencies: {}
impl < A , const N : usize > DerefMut for Chunk < A , N > { fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
};
}
