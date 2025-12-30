// Generated macro for impl_98 (impl)
macro_rules! Depcrate_sized_chunkimpl_98 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_98"}
// Dependencies: {}
# [cfg (feature = "std")] impl < const N : usize > io :: Write for Chunk < u8 , N > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let old_len = self . len () ; self . extend (buf . iter () . cloned () . take (N - old_len)) ; Ok (self . len () - old_len) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
