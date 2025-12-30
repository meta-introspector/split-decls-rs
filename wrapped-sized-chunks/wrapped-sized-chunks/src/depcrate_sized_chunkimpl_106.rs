// Generated macro for impl_106 (impl)
macro_rules! Depcrate_sized_chunkimpl_106 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_106"}
// Dependencies: {}
impl < A , const N : usize > Deref for Chunk < A , N > { type Target = [A] ; fn deref (& self) -> & Self :: Target { self . as_slice () } }
};
}
