// Generated macro for impl_86 (impl)
macro_rules! Depcrate_sized_chunkimpl_86 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_86"}
// Dependencies: {}
impl < A , const N : usize > Drop for Chunk < A , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . as_mut_slice ()) } } }
};
}
