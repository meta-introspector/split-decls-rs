// Generated macro for impl_96 (impl)
macro_rules! Depcrate_sized_chunkimpl_96 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_96"}
// Dependencies: {}
impl < A , const N : usize > PartialOrd for Chunk < A , N > where A : PartialOrd , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
