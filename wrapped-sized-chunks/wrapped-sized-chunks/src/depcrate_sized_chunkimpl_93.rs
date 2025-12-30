// Generated macro for impl_93 (impl)
macro_rules! Depcrate_sized_chunkimpl_93 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_93"}
// Dependencies: {}
impl < A , const N : usize > Hash for Chunk < A , N > where A : Hash , { fn hash < H > (& self , hasher : & mut H) where H : Hasher , { for item in self { item . hash (hasher) } } }
};
}
