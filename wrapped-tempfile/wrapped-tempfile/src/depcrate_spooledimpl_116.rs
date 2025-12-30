// Generated macro for impl_116 (impl)
macro_rules! Depcrate_spooledimpl_116 {
() => {
// Module: crate::spooled
// Provides: {"impl_116"}
// Dependencies: {}
impl Read for SpooledTempFile { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . read (buf) , SpooledData :: OnDisk (file) => file . read (buf) , } } fn read_vectored (& mut self , bufs : & mut [io :: IoSliceMut < '_ >]) -> io :: Result < usize > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . read_vectored (bufs) , SpooledData :: OnDisk (file) => file . read_vectored (bufs) , } } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . read_to_end (buf) , SpooledData :: OnDisk (file) => file . read_to_end (buf) , } } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . read_to_string (buf) , SpooledData :: OnDisk (file) => file . read_to_string (buf) , } } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { match & mut self . inner { SpooledData :: InMemory (cursor) => cursor . read_exact (buf) , SpooledData :: OnDisk (file) => file . read_exact (buf) , } } }
};
}
