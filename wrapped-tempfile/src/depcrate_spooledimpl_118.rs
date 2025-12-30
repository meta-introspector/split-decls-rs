// Generated macro for impl_118 (impl)
macro_rules! Depcrate_spooledimpl_118 {
() => {
// Module: crate::spooled
// Provides: {"impl_118"}
// Dependencies: {}
impl Seek for SpooledTempFile { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . seek (pos) , SpooledData :: OnDisk (file) => file . seek (pos) , } } }
};
}
