// Generated macro for impl_187 (impl)
macro_rules! Depcrate_readimpl_187 {
() => {
// Module: crate::read
// Provides: {"impl_187"}
// Dependencies: {}
impl < R : Read > Read for ZipFile < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . reader . read (buf) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . reader . read_exact (buf) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . reader . read_to_end (buf) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { self . reader . read_to_string (buf) } }
};
}
