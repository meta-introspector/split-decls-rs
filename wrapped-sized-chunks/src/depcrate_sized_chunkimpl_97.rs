// Generated macro for impl_97 (impl)
macro_rules! Depcrate_sized_chunkimpl_97 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_97"}
// Dependencies: {}
impl < A , const N : usize > Ord for Chunk < A , N > where A : Ord , { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
