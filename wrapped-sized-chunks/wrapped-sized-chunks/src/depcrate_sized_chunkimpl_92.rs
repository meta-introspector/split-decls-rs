// Generated macro for impl_92 (impl)
macro_rules! Depcrate_sized_chunkimpl_92 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_92"}
// Dependencies: {}
impl < A , const N : usize > Debug for Chunk < A , N > where A : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . write_str ("Chunk") ? ; f . debug_list () . entries (self . iter ()) . finish () } }
};
}
